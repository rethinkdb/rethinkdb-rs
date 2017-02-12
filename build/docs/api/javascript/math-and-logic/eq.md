---
layout: api-command
language: JavaScript
permalink: api/javascript/eq/
command: eq
io:
    -   - value
        - bool
related_commands:
    and: and/
    or: or/
    ne: ne/
---

# Command syntax #

{% apibody %}
value.eq(value[, value, ...]) &rarr; bool
{% endapibody %}

# Description #

Test if two or more values are equal.

__Example:__ See if a user's `role` field is set to `administrator`. 

```js
r.table('users').get(1)('role').eq('administrator').run(conn, callback);
```

__Example:__ See if three variables contain equal values.

```js
r.eq(a, b, c).run(conn, callback);
```
