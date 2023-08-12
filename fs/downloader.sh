#!/bin/zsh

echo "Start downloading...from $(date)"

target_dir=$1
mkdir -p $target_dir

origin="https://jyywiki.cn/pages/OS/2023/"



for demo in $@;
do
        echo "$target_dir, $demo"
        if [[ $demo != $target_dir ]]; then
            concate_url="$origin$demo"
            echo "download: $concate_url to $target_dir"
            cmd="wget  -c  $concate_url -P $target_dir"
            output=$(eval "$cmd")
            echo "$output"
        fi

done


