---
layout: api-command
language: JavaScript
permalink: api/javascript/round/
command: round
io:
    -   - number
        - number
related_commands:
    ceil: ceil/
    floor: floor/
---
# Command syntax #

{% apibody %}
r.round(number) &rarr; number
number.round() &rarr; number
{% endapibody %}

# Description #

Rounds the given value to the nearest whole integer. Values of 1.0 up to but not including 1.5 will return the largest integer value less than or equal to the given value, similar to [floor][]; values of 1.5 up to but not including 2.0 will return the smallest integer value greater than or equal to the given value, similar to [ceil][].

[floor]: /api/javascript/floor/
[ceil]:  /api/javascript/ceil/

__Example:__ Round 12.345 to the nearest integer.

```js
r.round(12.345).run(conn, callback);
// Result passed to callback
12.0
```

The `round` command can also be chained after an expression.

__Example:__ Round -12.345 to the nearest integer.

```js
r.expr(-12.345).round().run(conn, callback);
// Result passed to callback
-12.0
```

__Example:__ Return Iron Man's weight, rounded to the nearest integer.

```js
r.table('superheroes').get('ironman')('weight').round().run(conn, callback);
```
