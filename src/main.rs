// Copyright 2026 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "linux")]
    {
        linux::run().await
    }

    #[cfg(target_os = "windows")]
    {
        windows::run().await
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        eprintln!("Supported only on Linux or Windows");
        std::process::exit(1);
    }
}
