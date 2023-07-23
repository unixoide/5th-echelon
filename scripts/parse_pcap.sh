#!/bin/bash

cd "$( dirname "${BASH_SOURCE[0]}" )"

file=$1
stream=$2

tshark -r "$file" -Y "udp.stream == $stream"

for line in $(tshark -r "$file" -Y "udp.stream == $stream" -Tfields -e data --disable-protocol prudp)
do
  echo $line | xxd -r -p | ../target/debug/qpacket-decoder dump | ../target/debug/rmc-decoder dump | xxd
done
