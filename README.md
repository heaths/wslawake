# Keep Windows Awake from WSL

Keep Windows awake from Linux.

A systemd service monitors D-Bus signals for inhibitor blocks to idle, sleep, or shutdown actions.
In response, a Windows child process will keep the system awake.

## Installation

This is intended only for Windows Subsystem for Linux (WSL2) and requires that systemd is enabled.
These instructions will be simplified in the near future.

1. Download an appropriate package from the [latest release](https://github.com/heaths/wslawake/releases/latest)
   or as a pull request artifact for your platform e.g., `linux-x64.tar.gz`.

2. Extract it e.g.:

   ```sh
   tar -C wslawake -xvf linux-x64.tar.gz
   ```

3. Run the install script with elevated privileges:

   ```sh
   sudo wslawake/install.sh
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

## License

Licensed under the [MIT](LICENSE.txt) license.
