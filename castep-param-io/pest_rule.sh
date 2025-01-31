#! /bin/bash
input=$1
file=$2
pos=$3
inner_type=$4
with_funcs=$(</dev/stdin)
dependency="use pest::{Span,Parser};\n\
	use pest_ast::FromPest;\n\
	use from_pest::FromPest;\n\
	use crate::parser::Rule;"
rule_pos=$(echo "$pos" | awk '{print $1+1}')
inner_pos=$(echo "$rule_pos" | awk '{print $1+2}')
ast_rule="#[pest_ast(rule(Rule::${input}))]"
inner_rule="#[pest_ast(inner(rule(Rule::${inner_type}),${with_funcs}))]"
rule=$(printf "%s" "$ast_rule" | gsed 's/rule(\(Rule::\(.*\)\)))/rule=\1,keyword="\U\2")/' | sd "ast" "rule")
printf "%s\n%s" "$rule" "$inner_rule"
gsed -i "${pos}a ${ast_rule}" "$file"
gsed -i "${rule_pos}a ${rule}" "$file"
gsed -i "${inner_pos}a ${inner_rule}" "$file"
gsed -i "2a ${dependency}" "$file"
