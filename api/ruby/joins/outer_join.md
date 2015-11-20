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
sequence.outer_join(other_sequence, predicate_function) &rarr; stream
array.outer_join(other_sequence, predicate_function) &rarr; array
{% endapibody %}

# Description #

Returns a left outer join of two sequences. The returned sequence represents a union of the left-hand sequence and the right-hand sequence: all documents in the left-hand sequence will be returned, each matched with a document in the right-hand sequence if one satisfies the predicate condition. In most cases, you will want to follow the join with [zip](/api/ruby/zip) to combine the left and right results.


{% infobox %}
Note that `outer_join` is slower and much less efficient than using [concat_map](/api/ruby/concat_map/) with [get_all](/api/ruby/get_all). You should avoid using `outer_join` in commands when possible.
{% endinfobox %}

__Example:__ Return a list of all Marvel heroes, paired with any DC heroes who could beat them in a fight.

```rb
r.table('marvel').outer_join(r.table('dc')) {|marvel_row, dc_row|
    marvel_row[:strength] < dc_row[:strength]
}.run(conn)
```

(Compare this to an [inner_join](/api/ruby/inner_join) with the same inputs and predicate, which would return a list only of the matchups in which the DC hero has the higher strength.)