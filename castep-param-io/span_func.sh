#! /bin/bash
type=$1
file=$2
line=$3
printf "%s\n" "To convert:"
input="$(<./paste)"
match=$(echo "$input" | sd '\s+(.*\:\:.*) =>.*(".*").*' '$2 => Some($1),\n')
template="
fn from_span(span: Span<'_>) -> $type {
    let input = span.as_str();
    match input.to_lowercase().as_str() {
     $match
      _ => None
  }.expect(\"Always correct\")
}
"
formatted=$(printf "%s" "$template" | rustfmt)
printf "%s" "$formatted" | bat -l rust
paste=$(printf "%s\n" "$formatted" | sd '\n' '\\n')
gsed -i -e "${line}a ${paste}" "${file}"
