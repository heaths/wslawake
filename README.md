# Keep Windows Awake from WSL

Keep Windows awake from Linux.

A systemd service monitors D-Bus signals for inhibitor blocks to idle, sleep, or shutdown actions.
In response, a Windows child process will keep the system awake.

## Installation

This is intended only for Windows Subsystem for Linux (WSL2) and requires that systemd is enabled.

### Homebrew

Use [Homebrew](https://brew.sh/) to install the service in Linux distributions.
You first need to install the tap, but only the first time:

```sh
brew tap heaths/tap
```

Once the tap is installed, you can install or update the `wslawake` formula.
The service is not started automatically up installation and must be started manually.

```sh
brew install wslawake
brew services start wslawake
```

### Manual

1. Download an appropriate tarball from the [latest release](https://github.com/heaths/wslawake/releases/latest)
   or as a pull request artifact for your platform e.g., `linux-x64.tar.gz`.

2. Extract it e.g.:

   ```sh
   tar -C wslawake -xvf linux-x64.tar.gz
   ```

3. Run the install script with elevated privileges:

   ```sh
   sudo wslawake/install.sh
   ```

#### Uninstall

Run the `uninstall.sh` in the tarball to stop the service and remove the service files.

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
