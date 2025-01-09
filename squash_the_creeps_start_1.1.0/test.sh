#!/bin/sh
echo -ne '\033c\033]0;Squash the Creeps (3D)\a'
base_path="$(dirname "$(realpath "$0")")"
"$base_path/test" "$@"
