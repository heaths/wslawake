// Copyright 2026 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use std::{env, process::Stdio, time::Duration};
use tokio::{io::AsyncWriteExt as _, process::Command};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let program = match env::args().nth(1) {
        Some(arg) => arg.parse()?,
        None => {
            let Some(path) = env::current_exe()?
                .parent()
                .map(|dir| dir.join("wslawake.exe"))
            else {
                return Err("failed to get path to wslawake.exe in program directory".into());
            };
            path
        }
    };
    eprintln!("Starting {}", program.display());

    let mut child = Command::new(program).stdin(Stdio::piped()).spawn()?;
    let mut stdin = child
        .stdin
        .take()
        .ok_or("failed to connect to child process")?;

    stdin.write_all("idle\n".as_bytes()).await?;
    tokio::time::sleep(Duration::from_secs(5)).await;
    stdin.write_all("\n".as_bytes()).await?;
    tokio::time::sleep(Duration::from_secs(5)).await;

    // TODO: We need to make sure the process is reaped even if we fail to write.
    child.kill().await?;

    Ok(())
}
