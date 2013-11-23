---
layout: api-command
language: Python
permalink: api/python/union/
command: union
---

# Command syntax #

{% apibody %}
sequence.union(sequence) &rarr; array
{% endapibody %}

# Description #

Concatenate two sequences.

__Example:__ Construct a stream of all heroes.

```py
r.table('marvel').union(r.table('dc')).run(conn)
```

