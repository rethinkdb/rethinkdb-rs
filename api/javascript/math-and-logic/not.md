---
layout: api-command
language: JavaScript
permalink: api/javascript/not/
command: not
io:
    -   - bool
        - bool
related_commands:
    eq: eq
    ne: ne/
---

# Command syntax #

{% apibody %}
bool.not() &rarr; bool
not(bool) &rarr; bool
{% endapibody %}

# Description #
Compute the logical inverse (not) of an expression.

`not` can be called either postfix-style, immediately after an expression that evaluates as a boolean value, or infix-style, passing the expression as a parameter to `not`.

__Example:__ Not true is false.

```js
r.expr(true).not().run(conn, callback)
r.not(true).run(conn, callback)
```

These evaluate to `false`.

__Example:__ Return all the users that do not have a "flag" field.

```js
r.table('users').filter(function(user) {
    return user.hasFields('flag').not()
}).run(conn, callback)
```

__Example:__ As above, but infix style.

```js
r.table('users').filter(function(user) {
    return r.not(user.hasFields('flag'))
}).run(conn, callback)
```
