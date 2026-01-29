#!/bin/env bash
set -e

root=$(cd "$(dirname $0)/../.." && pwd)

# build new version number
local_sdk_version=$(cat $root/sdk_version)
local_sdk_version_major=$(echo $local_sdk_version | cut -d '.' -f 1)
local_sdk_version_minor=$(echo $local_sdk_version | cut -d '.' -f 2)
new_sdk_version_minor=$(( local_sdk_version_minor + 1 ))
new_sdk_version="$local_sdk_version_major.$new_sdk_version_minor.0"

echo "$new_sdk_version" > $root/sdk_version

# generate SDK
cd "$root"
make gen
