#! /usr/bin/env bash
input=$1
printf "%s\n" "Waiting for input from 'rule'"
rule_input="$(<./rule)"
printf "%s" "$rule_input"
GRAMMAR="/Users/tonywu/Library/Mobile Documents/com~apple~CloudDocs/Programming/castep-cell-io/castep-param-io/src/parser/param.pest"

printf '%s = {^"%s" ~ ":" ~ %s}\n' "$input" "$input" "$rule_input" >>"$GRAMMAR"
