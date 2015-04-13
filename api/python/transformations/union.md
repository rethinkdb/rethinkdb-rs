---
layout: api-command
language: Python
permalink: api/python/union/
command: union
---

# Command syntax #

{% apibody %}
stream.union(sequence[, sequence, ...]) &rarr; stream
array.union(sequence[, sequence, ...]) &rarr; array
{% endapibody %}

# Description #

Concatenate two or more sequences.

__Example:__ Construct a stream of all heroes.

```py
r.table('marvel').union(r.table('dc')).run(conn)
```

__Example:__ Combine four arrays into one.

```py
r.expr([1, 2]).union([3, 4], [5, 6], [7, 8, 9]).run(conn)

[1, 2, 3, 4, 5, 6, 7, 8, 9]
```
