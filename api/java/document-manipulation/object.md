---
layout: api-command
language: Java
permalink: api/java/object/
command: object
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
`r.expr([[A, B], [C, D]]).coerce_to('OBJECT')`.

__Example:__ Create a simple object.

```js
r.object('id', 5, 'data', ['foo', 'bar']).run(conn)
```

Result:

```js
{data: ["foo", "bar"], id: 5}
```
