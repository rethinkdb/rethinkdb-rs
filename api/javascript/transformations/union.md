---
layout: api-command
language: JavaScript
permalink: api/javascript/union/
command: union
io:
    -   - sequence
        - array
---

# Command syntax #

{% apibody %}
stream.union(sequence[, sequence, ...]) &rarr; stream
array.union(sequence[, sequence, ...]) &rarr; array
{% endapibody %}

# Description #

Concatenate two sequences.

__Example:__ Construct a stream of all heroes.

```js
r.table('marvel').union(r.table('dc')).run(conn, callback)
```
