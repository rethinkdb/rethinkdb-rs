---
layout: api-command
language: JavaScript
permalink: api/javascript/floor/
command: floor
io:
    -   - number
        - number
related_commands:
    ceil: ceil/
    round: round/
---
# Command syntax #

{% apibody %}
r.floor(number) &rarr; number
number.floor() &rarr; number
{% endapibody %}

# Description #

Rounds the given value down, returning the largest integer value less than or equal to the given value (the value's floor).

__Example:__ Return the floor of 12.345.

```js
r.floor(12.345).run(conn);
// Result passed to callback
12.0
```

The `floor` command can also be chained after an expression.

__Example:__ Return the floor of -12.345.

```js
r.expr(-12.345).floor().run(conn);
// Result passed to callback
-13.0
```

__Example:__ Return Iron Man's weight, rounded down with `floor`.

```js
r.table('superheroes').get('ironman')('weight').floor().run(conn);
```
