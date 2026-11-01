# Keep Windows Awake from WSL

Keep Windows awake from Linux.

A systemd service monitors D-Bus signals for inhibitor blocks to idle, sleep, or shutdown actions.
In response, a Windows child process will keep the system awake.

## Installation

This is intended only for Windows Subsystem for Linux (WSL2) and requires that systemd is enabled.
These instructions will be simplified in the near future.

1. [Enable](https://learn.microsoft.com/windows/wsl/systemd) support if you haven't already.
2. [Install Rust](https://rust-lang.org/tools/install) in Windows and your Linux distribution.
3. Clone this repository in Linux:

   ```sh
   git clone https://github.com/heaths/wslawake
   ```

4. Build a release executable in Linux:

   ```sh
   cargo build --release
   ```

5. Start a Developer Command shell in Windows.
   You'll need to have at least [Microsoft Visual Studio](https://visualstudio.microsoft.com/downloads/) Community edition installed with Visual C++.

   Change directories to the WSL directory into which you cloned the repo e.g.:

   ```sh
   # Replace "Ubuntu" with your distro name.
   cd \\wsl$\Ubuntu\home\heaths\src\wslawake
   cargo build --release --target=x86_64-pc-windows-msvc
   ```

6. Run the install script in Linux with elevated permissions:

   ```sh
   sudo ./service/install.sh -v
   ```

### Test the Installation

The `install.sh` script will enable and start `wslawake.service` automatically.

To see this in action:

1. Simulate some work in Linux:

   ```sh
   systemd-inhibit --what=idle --mode=block sleep 10
   ```

2. Start an elevated shell in Windows and run:

   ```batch
   powercfg -requests
   ```

   You'll see `wslawake.exe` requiring the system.
   If you don't, run the `systemd-inhibit` command in Linux again.
   It may have already completed.

To see journal events from `wslawake` or `wslawake.exe` run:

```sh
journalctl -u wslawake.service
```

## License

Licensed under the [MIT](LICENSE.txt) license.
