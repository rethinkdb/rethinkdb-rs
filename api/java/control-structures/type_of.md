---
layout: api-command
language: JavaScript
permalink: api/javascript/type_of/
command: typeOf
io:
    -   - any
        - string
---

# Command syntax #

{% apibody %}
any.typeOf() &rarr; string
{% endapibody %}

# Description #

Gets the type of a value.

__Example:__ Get the type of a string.

```js
r.expr("foo").typeOf().run(conn)
```

