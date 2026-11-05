#!/bin/sh
# Copyright 2026 Heath Stewart.
# Licensed under the MIT License. See LICENSE.txt in the project root for license information.

set -u

_service_path="/etc/systemd/system/wslawake.service"
_linux_bin_path="/usr/local/bin/wslawake"
_windows_bin_path="/usr/local/bin/wslawake.exe"

usage() {
    cat <<EOF
Uninstall the wslawake service to keep Windows awake from Linux.

Supported only in Windows Subsystem for Linux (WSL).

Usage: uninstall.sh [OPTIONS]
  -h, --help
        Print help
  -v, --verbose
        Enable verbose logging
EOF
}

_verbose=false

main() {
    need_cmd grep
    need_cmd rm
    need_cmd systemctl
    need_cmd uname

    is_wsl || exit 1

    while [ $# -gt 0 ]; do
        case "$1" in
            -h|--help)
                shift
                usage
                exit 0
                ;;
            -v|--verbose)
                shift
                _verbose=true
                ;;
            *)
                err "unknown argument: $1"
                exit 1
                ;;
            esac
    done

    if has_service wslawake.service; then
        echo 'Stopping "wslawake.service"'
        systemctl disable --now wslawake.service || exit 1
    fi

    echo 'Deleting files'
    delete "$_service_path" || exit 1
    delete "$_linux_bin_path" || exit 1
    delete "$_windows_bin_path" || exit 1

    systemctl daemon-reload || exit 1
}

is_wsl() {
    if ! uname -a | grep -iq microsoft; then
        err "supported only in Windows Subsystem for Linux (WSL)"
        exit 1
    fi
}

log() {
    if [ "$_verbose" = true ]; then
        echo "$1"
    fi
}

err() {
    echo "Error: $1" >&2
}

has_service() {
    systemctl is-enabled "$1" > /dev/null 2>&1
}

need_cmd() {
    if ! check_cmd "$1"; then
        err "need \"$1\" (command not found)"
        exit 1
    fi
}

delete() {
    if [ -f "$1" ]; then
        log "Deleting \"$1\""
        rm -f "$1"
    fi
}

check_cmd() {
    command -v "$1" > /dev/null 2>&1
}

main "$@"
