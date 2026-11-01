# Contributing

Contributions are welcome!
You'll need both Linux and Windows build environments.

## Prerequisites

1. [Git] to clone this repository from your fork in linux e.g.:

   ```sh
   # Fork https://github.com/heaths/wslawake first.
   git clone https://github.com/your-username/wslawake.git ~/src/wslawake
   ```

2. [Rust] installed on both Linux and Windows.
3. [Visual Studio Community] or a commercial edition with the following workloads and components:
   1. Workload: Desktop development with C++
   2. One or both of the following components:
      - Component: MSVC x64/x86 build tools (latest)
      - Component: MSVC arm64 build tools (latest)
   3. Required component: Windows 11 SDK (latest)

   You can also [import](https://learn.microsoft.com/visualstudio/install/import-export-installation-configurations) the `.vsconfig` file in the root of this repository.

## Build

I recommend you build both the Linux and Windows binaries in the same WSL-mounted directory.

### Linux

1. Make sure you have the right target(s) installed for what you want to build:
   - `aarch64-unknown-linux-gnu`
   - `x86_64-unknown-linux-gnu`

2. Build the executable with or without an explicit `--target`.

   ```sh
   cargo build
   cargo build --target=aarch64-unknown-linux-gnu
   ```

   Building without an explicit target will build with the target matching your current system.

### Windows

1. Make sure you have the right target(s) installed for what you want to build:
   - `aarch64-pc-windows-msvc`
   - `x86_64-pc-windows-msvc`

2. Build the executable with an explicit target to avoid overwriting Linux files from the previous step.
   The implicit target always ends up in the `target/debug` or `target/release` directory otherwise.

   ```sh
   cargo build --target=x86_64-pc-windows-msvc
   ```

   Building without an explicit target will build with the target matching your current system.

## Package

Once both Linux and Windows executables have been build, copy them—ideally for the same architecture—to the `service/` directory.
Both `wslawake` and `wslawake.exe` are ignored by `git` in that directory.
You can `chmod +x` both of them but it is not necessary since the `install.sh` script will do that to their destination paths.

```sh
cp target/release/wslawake service/
cp target/x86_64-pc-windows-msvc/release/wslawake.exe service/
sudo ./service/install.sh -v
```

## Test

The install script will enable and start the `wslawake.service` unit automatically, but you can also start and stop it manually.

```sh
sudo systemctl start wslawake.service
sudo systemctl stop wslawake.service
```

To see it working, start an elevated shell in Windows. Then in a Linux shell, run:

```sh
systemd-inhibit --what=idle --mode=block sleep 10
```

In the elevated Windows shell, run:

```batch
powercfg -requests
```

You should see `wslawake.exe` requiring the system e.g.:

```text
SYSTEM:
[PROCESS] \Device\Mup\wsl.localhost\Ubuntu\usr\local\bin\wslawake.exe
```

You can also see the process tree:

```sh
ps -HC wslawake,wslawake.exe
```

## Diagnostics

`wslawake.exe` writes to `stderr` that is piped to `wslawake`.
The `wslawake.service` unit redirects both processes' writes to `stderr` to the journal so you can view them:

```sh
journalctl -u wslawake.service
```

## License

Licensed under the [MIT](LICENSE.txt) license.

[Git]: https://git-scm.com/install/
[Rust]: https://rust-lang.org/tools/install/
[Visual Studio Community]: https://visualstudio.microsoft.com/vs/community/
