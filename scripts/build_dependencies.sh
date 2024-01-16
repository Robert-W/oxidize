#!/bin/bash

# Read the file and store the results in an array
lockfile=$(<./Cargo.lock)

# Loop over the file and split on [[package]]
while read -d "[[package]]" chunk; do
  # Find the name with grep, grab the value with awk, cut out the " with td
  name=$(grep -o 'name = ".*"' <<< $chunk | awk '{print $3}' | tr -d '"')
  # Find the version the using the same method
  version=$(grep -o 'version = ".*"' <<< $chunk | awk '{print $3}' | tr -d '"')

  # make sure both of these values are set
  if [[ -n "$name" && -n "$version" ]]; then
    cargo build -p $name:$version
  fi
done <<< $lockfile
