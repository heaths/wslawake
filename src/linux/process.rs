// Copyright 2026 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use std::{env, process::Stdio};
use tokio::{
    io::{self, AsyncWriteExt as _},
    process::{self, ChildStdin, Command},
};

#[derive(Debug)]
pub(super) struct Child {
    child: Option<process::Child>,
    stdin: ChildStdin,
}

impl Child {
    pub async fn spawn() -> Result<Self, Box<dyn std::error::Error>> {
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
        let stdin = child
            .stdin
            .take()
            .ok_or("failed to connect to child process")?;

        Ok(Self {
            child: Some(child),
            stdin,
        })
    }

    pub async fn write_str(&mut self, s: &str) -> io::Result<()> {
        self.stdin.write_all(s.as_bytes()).await
    }

    pub async fn kill(&mut self) -> io::Result<()> {
        if let Some(mut child) = self.child.take() {
            child.kill().await?;
        }
        Ok(())
    }
}

impl Drop for Child {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            tokio::spawn(async move {
                let _ = child.kill().await;
            });
        }
    }
}
