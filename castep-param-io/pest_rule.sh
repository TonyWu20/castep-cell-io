#! /bin/bash
input="$(</dev/stdin)"
rule=$(printf "%s" "$input" | gsed 's/rule(\(Rule::\(.*\)\)))/rule=\1,keyword="\U\2")/' | sd "ast" "rule")
inner=$(printf "%s" "$input" | sd '(rule.*)\)]' 'inner($1,with(from_span)))]')
printf "%s\n%s" "$rule" "$inner"
printf "\n%s\n%s\n" "$rule" "$inner" | pbcopy
