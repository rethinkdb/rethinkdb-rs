---
layout: api-command
language: JavaScript
permalink: api/javascript/expr/
command: expr
io:
    -   - r
        - value
---

# Command syntax #

{% apibody %}
r.expr(value) &rarr; value
{% endapibody %}

# Description #

Construct a ReQL JSON object from a native object.

__Example:__ Objects wrapped with expr can then be manipulated by ReQL API functions.

```js
r.expr({a:'b'}).merge({b:[1,2,3]}).run(conn, callback)
```


__Example:__ In JavaScript, you can also do this with just r.

```js
r({a: 'b'}).merge({b: [1,2,3]}).run(conn, callback)
```

