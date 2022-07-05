#!/bin/bash

TARGET_DIR=generated/

mkdir -p "${TARGET_DIR}"


for f in $(ls -1 ddls/*.json)
do
  ./gen-protos.py "$f" "${TARGET_DIR}"
done

files=($(cd "${TARGET_DIR}" && ls -1 *.rs))
echo > "${TARGET_DIR}/mod.rs"
for f in "${files[@]}"
do
  if [ "$f" != "mod.rs" ]
  then
    echo "pub mod ${f%.rs};" >> "${TARGET_DIR}/mod.rs"
  fi
done

rustfmt "${TARGET_DIR}"/*/*.rs "${TARGET_DIR}"/*.rs
