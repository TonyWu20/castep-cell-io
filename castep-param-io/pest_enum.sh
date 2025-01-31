#! /bin/bash
input=$(</dev/stdin)
replaced=$(printf "%s" "$input" | sd '.*(".*").*' '^$1 |' | gsed '$ s/|//g')
printf "%s" "$replaced" | tee >(pbcopy) | bat
