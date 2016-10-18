#!/usr/bin/env bash

repeat_amount="$1"
url="$2"

function sleep_for_time_then_run {
    number_in="$1"
    url="$2"
    time_sleep=$(bc -l <<< "$number_in / 100")
    sleep "$time_sleep"
    curl --retry 10 -s "$url"
}
export -f sleep_for_time_then_run

seq "$repeat_amount" | parallel -j0 --joblog log sleep_for_time_then_run {} $url
cut -f 4 log