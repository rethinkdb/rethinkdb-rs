---
layout: api-command 
language: Python
permalink: api/python/union/
command: union
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/transformations/union.md
---

{% apibody %}
sequence.union(sequence) &rarr; array
{% endapibody %}

Concatenate two sequences.

__Example:__ Construct a stream of all heroes.

```py
r.table('marvel').union(r.table('dc')).run(conn)
```

