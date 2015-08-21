---
layout: api-command
language: JavaScript
permalink: api/javascript/ne/
command: ne
io:
    -   - value
        - bool
related_commands:
    and: and/
    or: or/
    eq: eq/
---

# Command syntax #

{% apibody %}
value.ne(value[, value, ...]) &rarr; bool
{% endapibody %}

# Description #

Test if two or more values are not equal.

__Example:__ See if a user's `role` field is not set to `administrator`. 

```js
r.table('users').get(1)('role').ne('administrator').run(conn, callback);
```

__Example:__ See if three variables do not contain equal values.

```js
r.ne(a, b, c).run(conn, callback);
```
