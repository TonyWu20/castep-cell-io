#! /bin/bash
printf "%s\n" "Type:"
type="$(</dev/stdin)"
printf "%s\n" "To convert:"
input="$(</dev/stdin)"
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
printf "\n%s\n" "$formatted" | pbcopy
