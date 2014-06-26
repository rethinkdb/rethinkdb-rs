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

Returns the outer product of two sequences (e.g. a table, a filter result). The query returns each row of the left sequence paired with each row of the right sequence that satisfies the predicate. The predicate can be either a field name to join on or a function that filters the input sequence.

Note that `outer_join` is slower and much less efficient than using [concat_map](/api/ruby/concat_map/) with [get_all](/api/ruby/get_all). You should avoid using `outer_join` in commands when possible.


__Example:__ Construct a sequence of documents containing all cross-universe matchups
where a Marvel hero would lose, but keep Marvel heroes who would never lose a matchup in
the sequence.

```rb
r.table('marvel').outer_join(r.table('dc')) {|marvel_row, dc_row|
    marvel_row[:strength] < dc_row[:strength]
}.run(conn)
```

__Example:__ Join a sequence on a simple field name.

```rb
r.table('players').outer_join(r.table('games'), 'game_id').zip().run(conn)
```

__Example:__ Use [nested field](/docs/cookbook/javascript/#filtering-based-on-nested-fields) syntax to join on fields from subdocuments.

```rb
r.table('players').outer_join(r.table('games'), { :game => 'id' } ).zip().run(conn)
```
