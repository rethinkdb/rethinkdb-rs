---
layout: api-command 
language: Python
permalink: api/python/outer_join/
command: outer_join 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/joins/outer_join.md
related_commands:
    eq_join: eq_join/
    inner_join: inner_join/
    zip: zip/
---

{% apibody %}
sequence.outer_join(other_sequence, predicate) → stream
array.outer_join(other_sequence, predicate) → array
{% endapibody %}

Computes a left outer join by retaining each row in the left table even if no match was
found in the right table.

__Example:__ Construct a sequence of documents containing all cross-universe matchups
where a marvel hero would lose, but keep marvel heroes who would never lose a matchup in
the sequence.

```py
r.table('marvel').outer_join(r.table('dc'),
  lambda marvelRow, dcRow: marvelRow['strength'] < dcRow['strength']).run(conn)
```


