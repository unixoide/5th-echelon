#!/bin/bash

cd "$( dirname "${BASH_SOURCE[0]}" )"

set -e

cargo build --release -p wireshark-dissector "${@}"

target="${HOME}/.local/lib/wireshark/plugins/$(tshark --version | grep TShark | egrep -o '[0-9]\.[0-9]+')/epan"

mkdir -p "${target}"

ln -fs "$(cargo metadata --format-version=1 --no-deps|jq -r '.target_directory')/debug/libprudp_rmc_plugin.so" "${target}/prudp_rmc_plugin.so"
