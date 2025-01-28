#! /usr/bin/env bash
printf "%s\n" "rule name:"
input="$(</dev/stdin)"
printf "%s\n" "To convert:"
rule_input="$(</dev/stdin)"
GRAMMAR="/Users/tonywu/Library/Mobile Documents/com~apple~CloudDocs/Programming/castep-cell-io/castep-param-io/src/parser/param.pest"
{
	printf '%s = {^"{%s}" ~ ":" ~ %ss}\n' "$input" "$input" "$input"
	printf '%ss = {' "$input"
	echo "$rule_input" | ./pest_enum.sh >>"$GRAMMAR"
} >>"$GRAMMAR"
echo "}" >>"$GRAMMAR"
