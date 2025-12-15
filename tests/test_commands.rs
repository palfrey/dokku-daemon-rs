use std::{fs, time::Duration};

use anyhow::{Context, Result};
use dokku_daemon_rs::{ClientReturn, run};
use serial_test::serial;
use tokio::{net::UnixSocket, task};

async fn run_command(command: &str) -> Result<ClientReturn> {
    let socket_path = "./target/socket";
    let _ = fs::remove_file(&socket_path);
    task::spawn(async {
        run(socket_path).await.unwrap();
    });

    loop {
        if fs::exists(socket_path)? {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    let socket = UnixSocket::new_stream()?;
    let stream = socket
        .connect(socket_path)
        .await
        .with_context(|| format!("Connecting to '{socket_path}'"))?;
    stream.writable().await?;
    stream.try_write(command.as_bytes())?;
    stream.readable().await?;
    let mut buffer = [0; 256];
    let count = stream.try_read(&mut buffer)?;
    let cr: ClientReturn = serde_json::from_slice(&buffer[..count])?;
    Ok(cr)
}

#[tokio::test]
#[serial]
async fn test_real_command() -> Result<()> {
    let cr = run_command("test").await?;
    assert_eq!(
        cr,
        ClientReturn {
            ok: false,
            output: "Failed to execute 'dokku test': No such file or directory (os error 2)".into()
        }
    );
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_quit() -> Result<()> {
    let cr = run_command("quit").await?;
    assert_eq!(
        cr,
        ClientReturn {
            ok: true,
            output: "Exiting".into()
        }
    );
    Ok(())
}
