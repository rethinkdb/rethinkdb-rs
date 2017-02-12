---
layout: api-command
language: JavaScript
permalink: api/javascript/object/
command: object
io:
    -   - r
        - object
related_commands:
    coerceTo: coerce_to/
    merge: merge/
    keys: keys/
---

# Command syntax #

{% apibody %}
r.object([key, value,]...) &rarr; object
{% endapibody %}

# Description #

Creates an object from a list of key-value pairs, where the keys must
be strings.  `r.object(A, B, C, D)` is equivalent to
`r.expr([[A, B], [C, D]]).coerceTo('OBJECT')`.

__Example:__ Create a simple object.

```js
r.object('id', 5, 'data', ['foo', 'bar']).run(conn, callback)
```

Result:

```js
{data: ["foo", "bar"], id: 5}
```
