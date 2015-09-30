---
layout: api-command
language: JavaScript
permalink: api/javascript/gt/
command: gt
io:
    -   - value
        - bool
related_commands:
    eq: eq
    ne: ne/
    ge: ge/
    lt: lt/
    le: le/
---

# Command syntax #

{% apibody %}
value.gt(value[, value, ...]) &rarr; bool
{% endapibody %}

# Description #

Compare values, testing if the left-hand value is greater than the right-hand.

__Example:__ Test if a player has scored more than 10 points.

```js
r.table('players').get(1)('score').gt(10).run(conn, callback);
```

__Example:__ Test if variables are ordered from lowest to highest, with no values being equal to one another.

```js
var a = 10, b = 20, c = 15;
r.gt(a, b, c).run(conn, callback);
```

This is the equivalent of the following:

```js
r.gt(a, b).and(r.gt(b, c)).run(conn, callback);
```
