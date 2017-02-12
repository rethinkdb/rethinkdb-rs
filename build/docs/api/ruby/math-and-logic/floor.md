---
layout: api-command
language: Ruby
permalink: api/ruby/floor/
command: floor
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

```rb
> r.floor(12.345).run(conn)

12.0
```

The `floor` command can also be chained after an expression.

__Example:__ Return the floor of -12.345.

```rb
> r.expr(-12.345).floor().run(conn)

-13.0
```

__Example:__ Return Iron Man's weight, rounded down with `floor`.

```rb
r.table('superheroes').get('ironman')['weight'].floor().run(conn)
```
