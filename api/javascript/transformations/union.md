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
sequence.union(sequence[, sequence, ...]) &rarr; array
{% endapibody %}

# Description #

Concatenate two or more sequences.

__Example:__ Construct an array of all heroes.

```js
r.table('marvel').union(r.table('dc')).run(conn, callback);
```

__Example:__ Combine four arrays into one.

```js
r.expr([1, 2]).union([3, 4], [5, 6], [7, 8, 9]).run(conn, callback);
// Result passed to callback
[1, 2, 3, 4, 5, 6, 7, 8, 9]
```
