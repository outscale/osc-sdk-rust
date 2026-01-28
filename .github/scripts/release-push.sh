#!/bin/env bash
set -e

root=$(cd "$(dirname $0)/../.." && pwd)
if [ -e "$root/.auto-release-abort" ]; then
    echo "previous step triggered stop, abort"
    exit 0
fi
new_sdk_version=$(cat $root/sdk_version)
branch_name="autobuild-$new_sdk_version"

git push -f origin $branch_name
