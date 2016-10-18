#!/usr/bin/env bash

repeat_amount="$1"
url="$2"

seq "$repeat_amount" | parallel -j0 --joblog log curl -s "$url"
cut -f 4 log