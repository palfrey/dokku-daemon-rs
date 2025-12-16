use anyhow::Result;

use dokku_daemon_rs::run;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    clap::Command::new("dokku-daemon-rs")
        .version(clap::crate_version!())
        .get_matches();

    let dokku_daemon_socket_path = std::env::var("DOKKU_DAEMON_SOCKET_PATH")
        .unwrap_or_else(|_| "/var/run/dokku-daemon/dokku-daemon.sock".into());

    run(&dokku_daemon_socket_path).await
}
