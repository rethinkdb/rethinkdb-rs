---
layout: api-command
language: Ruby
permalink: api/ruby/union/
command: union
---

# Command syntax #

{% apibody %}
sequence.union(sequence[, sequence, ...]) &rarr; array
{% endapibody %}

# Description #

Concatenate two or more sequences.

__Example:__ Construct an array of all heroes.

```rb
r.table('marvel').union(r.table('dc')).run(conn)
```

__Example:__ Combine four arrays into one.

```rb
r.expr([1, 2]).union([3, 4], [5, 6], [7, 8, 9]).run(conn)

[1, 2, 3, 4, 5, 6, 7, 8, 9]
```
