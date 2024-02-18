#!/usr/bin/env bash

set -euxo pipefail

src=../astrid.tech-blogging/content
dst=test_data/astrid_dot_tech_example
scripts/transform-old.py $src -o $dst -f --published "2024-02-10 21:39:22-08:00"
rm -rf $dst/blog/2020/02 $dst/_untitled_posts