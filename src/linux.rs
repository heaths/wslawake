// Copyright 2026 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use futures::stream::StreamExt as _;
use tokio::{
    select,
    signal::unix::{SignalKind, signal},
};
use zbus::{Connection, proxy};

mod process;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Starting wslawake.exe");
    let mut child = process::Child::spawn().await?;
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;

    eprintln!("Connecting to system D-Bus");
    let conn = Connection::system().await?;
    let proxy = Login1ManagerProxy::new(&conn).await?;
    let value = proxy.block_inhibited().await?;
    eprintln!("Current BlockInhibited: {value}");

    let mut stream = proxy.receive_block_inhibited_changed().await;
    loop {
        select! {
            biased;

            Some(value) = stream.next() => {
                if let Ok(value) = value.get().await {
                    eprintln!("BlockInhibited changed: {value}");
                    child.write_str(&format!("{value}\n")).await?;
                }
            }
            _ = sigint.recv() => {
                eprintln!("Interrupted");
                break;
            }
            _ = sigterm.recv() => {
                eprintln!("Terminated");
                break;
            },
        }
    }

    eprintln!("Killing wslawake.exe");
    child.kill().await?;

    Ok(())
}

#[proxy(
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1",
    interface = "org.freedesktop.login1.Manager"
)]
trait Login1Manager {
    #[zbus(property)]
    fn block_inhibited(&self) -> zbus::Result<String>;
}
