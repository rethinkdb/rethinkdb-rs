---
layout: api-command
language: Ruby
permalink: api/ruby/outer_join/
command: outer_join
related_commands:
    inner_join: inner_join/
    outer_join: outer_join/
    zip: zip/
---

# Command syntax #

{% apibody %}
sequence.outer_join(other_sequence, predicate) &rarr; stream
array.outer_join(other_sequence, predicate) &rarr; array
{% endapibody %}

# Description #

Returns the outer product of two sequences (e.g. a table, a filter result). The query returns each row of the left sequence paired with each row of the right sequence that satisfies the predicate function. In most cases, you will want to follow the join with [zip](/api/ruby/zip) to combine the left and right results.

Note that `outer_join` is slower and much less efficient than using [concat_map](/api/ruby/concat_map/) with [get_all](/api/ruby/get_all). You should avoid using `outer_join` in commands when possible.


__Example:__ Construct a sequence of documents containing all cross-universe matchups
where a Marvel hero would lose, but keep Marvel heroes who would never lose a matchup in
the sequence.

```rb
r.table('marvel').outer_join(r.table('dc')) {|marvel_row, dc_row|
    marvel_row[:strength] < dc_row[:strength]
}.run(conn)
```
