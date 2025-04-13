#! /usr/bin/env bash
input=$1
printf "%s\n" "To convert:"
rule_input="$(<./paste)"
GRAMMAR="/Users/tonywu/Library/Mobile Documents/com~apple~CloudDocs/Programming/castep-cell-io/castep-param-io/src/parser/param.pest"
{
	printf '%s = {^"%s" ~ ":" ~ %ss}\n' "$input" "$input" "$input"
	printf '%ss = {' "$input"
	echo "$rule_input" | ./pest_enum.sh
} >>"$GRAMMAR"
echo "}" >>"$GRAMMAR"
