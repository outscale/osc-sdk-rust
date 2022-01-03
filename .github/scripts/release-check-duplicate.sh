#!/bin/env bash
set -e

root=$(cd "$(dirname $0)/../.." && pwd)
if [ -e "$root/.auto-release-abort" ]; then
    echo "previous step triggered stop, abort"
    exit 0
fi
# build new version number
local_sdk_version=$(cat $root/sdk_version)
local_sdk_version_major=$(echo $local_sdk_version | cut -d '.' -f 1)
local_sdk_version_minor=$(echo $local_sdk_version | cut -d '.' -f 2)
new_sdk_version_minor=$(( local_sdk_version_minor + 1 ))
new_sdk_version="$local_sdk_version_major.$new_sdk_version_minor.0"

branch_name="autobuild-$new_sdk_version"

if [ -z "$GH_TOKEN" ]; then
    echo "GH_TOKEN is missing, abort."
    exit 1
fi

result=$(curl -s -H "Authorization: token $GH_TOKEN" "https://api.github.com/repos/outscale/osc-sdk-rust/pulls" | jq ".[] | select(.title == \"SDK v$new_sdk_version\") | .title")

if [ ! -z "$result" ]; then
    echo "Pull request seems to alread exist, abort."
    touch "$root/.auto-release-abort"
    exit 0
fi
