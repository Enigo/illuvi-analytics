#!/bin/bash
SCRIPT_PATH=$(dirname $(realpath -s $0))
echo "Starting release of ui"
cd $SCRIPT_PATH && cd ../../site/ui

toml_file="Trunk.toml"

# due to https://github.com/thedodd/trunk/issues/461 I have to keep the file commented out for now.
# it is needed in order to set up the correct backend url

# removing comments
mapfile -t lines < "$toml_file"
for i in "${!lines[@]}"; do
  line="${lines[$i]}"
  lines[$i]="${line:1}"
done
printf '%s\n' "${lines[@]}" > "$toml_file"

cargo clean
trunk clean
trunk build --release

# adding comments back
mapfile -t lines < "$toml_file"
for i in "${!lines[@]}"; do
  line="${lines[$i]}"
  lines[$i]="#$line"
done
printf '%s\n' "${lines[@]}" > "$toml_file"

rsync -avO --delete --no-perms dist/ ru@frontend:/var/www/html
