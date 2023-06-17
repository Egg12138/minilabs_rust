#!/bin/zsh

origin="https://jyywiki.cn/pages/OS/2023/"
demo=$1
concate_url="$origin$demo"
target_dir=$2


echo "download: $concate_url to $target_dir"
cmd="wget  -c  $concate_url -P $target_dir"
output=$(eval "$cmd")
echo "$output"
