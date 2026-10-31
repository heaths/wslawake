// Copyright 2026 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#![allow(clippy::upper_case_acronyms)]
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = tokio::io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        if line.is_empty() {
            unsafe {
                SetThreadExecutionState(ES_CONTINUOUS);
            }
        } else {
            unsafe {
                SetThreadExecutionState(ES_SYSTEM_REQUIRED | ES_CONTINUOUS);
            }
        }
    }

    Ok(())
}

const ES_CONTINUOUS: u32 = 0x80000000;
const ES_SYSTEM_REQUIRED: u32 = 0x00000001;

#[link(name = "kernel32")]
unsafe extern "C" {
    unsafe fn SetThreadExecutionState(esFlags: u32) -> u32;
}
