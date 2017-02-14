---
layout: api-command
language: JavaScript
permalink: api/javascript/ge/
command: ge
io:
    -   - value
        - bool
related_commands:
    eq: eq/
    ne: ne/
    gt: gt/
    lt: lt/
    le: le/
---

# Command syntax #

{% apibody %}
value.ge(value[, value, ...]) &rarr; bool
{% endapibody %}

# Description #

Compare values, testing if the left-hand value is greater than or equal to the right-hand.

__Example:__ Test if a player has scored 10 points or more.

```javascript
r.table('players').get(1)('score').ge(10).run(conn, callback);
```

__Example:__ Test if variables are ordered from lowest to highest.

```javascript
var a = 10, b = 20, c = 15;
r.ge(a, b, c).run(conn, callback);
```

This is the equivalent of the following:

```javascript
r.ge(a, b).and(r.ge(b, c)).run(conn, callback);
```
