---
layout: api-command
language: Python
permalink: api/python/outer_join/
command: outer_join
related_commands:
    eq_join: eq_join/
    inner_join: inner_join/
    zip: zip/
---

# Command syntax #

{% apibody %}
sequence.outer_join(other_sequence, predicate) &rarr; stream
array.outer_join(other_sequence, predicate) &rarr; array
{% endapibody %}

# Description #

Computes a left outer join by retaining each row in the left table even if no match was found in the right table.

Note that `outer_join` is slower and much less efficient than using [concat_map](/api/python/concat_map/) with [get_all](/api/python/get_all). You should avoid using `outer_join` in commands when possible.

__Example:__ Construct a sequence of documents containing all cross-universe matchups
where a marvel hero would lose, but keep marvel heroes who would never lose a matchup in
the sequence.

```py
r.table('marvel').outer_join(r.table('dc'),
  lambda marvelRow, dcRow: marvelRow['strength'] < dcRow['strength']).run(conn)
```


