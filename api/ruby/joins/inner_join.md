---
layout: api-command
language: Ruby
permalink: api/ruby/inner_join/
command: inner_join
related_commands:
    eq_join: eq_join/
    outer_join: outer_join/
    zip: zip/
---

# Command syntax #

{% apibody %}
sequence.inner_join(other_sequence, predicate) &rarr; stream
array.inner_join(other_sequence, predicate) &rarr; array
{% endapibody %}

# Description #

Returns the inner product of two sequences (e.g. a table, a filter result) filtered by the predicate. The query compares each row of the left sequence with each row of the right sequence to find all pairs of rows which satisfy the predicate. When the predicate is satisfied, each matched pair of rows of both sequences are combined into a result row.

Note that `inner_join` is slower and much less efficient than using [eq_join](/api/ruby/eq_join/) or [concat_map](/api/ruby/concat_map/). You should avoid using `inner_join` in commands when possible.

__Example:__ Construct a sequence of documents containing all cross-universe matchups where a marvel hero would lose.

```rb
r.table('marvel').inner_join(r.table('dc')) {|marvel_row, dc_row|
    marvel_row[:strength] < dc_row[:strength]
}.run(conn)
```


