use std::fs;
use std::io::ErrorKind;
use std::process::Command;

use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use tokio::io::Interest;
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::mpsc::{self, Sender};
use tokio::task;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ClientReturn {
    pub ok: bool,
    pub output: String,
}

async fn handle_client(stream: UnixStream, tx: Sender<()>) -> Result<()> {
    let mut buffer = [0; 256];
    stream.ready(Interest::READABLE).await?;
    let res = stream.try_read(&mut buffer)?;
    let decoded = str::from_utf8(&buffer[..res])?.trim();
    debug!("Command: {decoded}");
    if decoded == "quit" {
        let response = serde_json::to_vec(&ClientReturn {
            ok: true,
            output: "Exiting".into(),
        })?;
        stream.try_write(&response)?;
        tx.send(()).await?;
        info!("Got exit command");
        return Ok(());
    }
    let args = decoded.split_ascii_whitespace();
    let mut initial_cmd = Command::new("dokku");
    let mut cmd = &mut initial_cmd;
    for arg in args {
        cmd = cmd.arg(arg);
    }
    let response = match cmd.output() {
        Ok(res) => {
            let decoded_stdout = str::from_utf8(res.stdout.as_slice())?;
            let decoded_stderr = str::from_utf8(res.stderr.as_slice())?;
            let decoded_result = decoded_stdout.to_string() + decoded_stderr;
            debug!("Result: {decoded_result}");
            serde_json::to_vec(&ClientReturn {
                ok: res.status.success(),
                output: decoded_result.to_string(),
            })?
        }
        Err(err) => {
            warn!("Failed to execute 'dokku {decoded}': {err}");
            serde_json::to_vec(&ClientReturn {
                ok: false,
                output: format!("Failed to execute 'dokku {decoded}': {err}"),
            })?
        }
    };
    stream.try_write(&response)?;

    Ok(())
}

pub async fn run(dokku_daemon_socket_path: &str) -> Result<()> {
    if let Err(fs_err) = fs::remove_file(dokku_daemon_socket_path) {
        match fs_err.kind() {
            ErrorKind::NotFound => {}
            _ => {
                error!("Error deleting {dokku_daemon_socket_path}: {fs_err}");
            }
        }
    }
    let listener = UnixListener::bind(dokku_daemon_socket_path)
        .with_context(|| format!("Binding {dokku_daemon_socket_path}"))?;

    let (tx, mut rx) = mpsc::channel::<()>(1);
    loop {
        tokio::select! {
            _ = rx.recv() => {
                info!("Exiting");
                break;
            }
            stream = listener.accept() => {
                match stream {
                    Ok((stream, _addr)) => {
                        let local_tx = tx.clone();
                        task::spawn(async move {
                            if let Err(err) = handle_client(stream, local_tx).await {
                                warn!("Client error: {err:#}");
                            };
                        });
                    }
                    Err(err) => {
                        warn!("Stream error: {err:#}");
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}
