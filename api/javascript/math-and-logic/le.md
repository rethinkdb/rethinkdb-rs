---
layout: api-command
language: JavaScript
permalink: api/javascript/le/
command: le
io:
    -   - value
        - bool
related_commands:
    eq: eq/
    ne: ne/
    gt: gt/
    ge: ge/
    lt: lt/
---

# Command syntax #

{% apibody %}
value.le(value[, value, ...]) &rarr; bool
{% endapibody %}

# Description #

Compare values, testing if the left-hand value is less than or equal to the right-hand.

__Example:__ Test if a player has scored 10 points or less.

```javascript
r.table('players').get(1)('score').le(10).run(conn, callback);
```

__Example:__ Test if variables are ordered from highest to lowest.

```javascript
var a = 20, b = 10, c = 15;
r.le(a, b, c).run(conn, callback);
```

This is the equivalent of the following:

```javascript
r.le(a, b).and(r.le(b, c)).run(conn, callback);
```
