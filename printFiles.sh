#!/usr/bin/env bash

SRC_DIR="./src"
CRATE_DIRS="$(find $SRC_DIR/* -type d)"


for crate_dir in $CRATE_DIRS; do 
  crate="${crate_dir##./src/}"  
  CRATES+=($crate)
  
  modules="$crate_dir/*"

  echo CRATE: $crate
  for module in $modules; do
    filename="  // ${module##*/$crate/}"
    echo
    echo "$filename"
    echo
    echo "$(cat $module  | sed 's/^/  /')"
    echo 
  done
done

