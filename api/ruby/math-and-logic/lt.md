---
layout: api-command
language: Ruby
permalink: api/ruby/lt/
command: '<, lt'
related_commands:
    '>, gt': gt/
    '>=, ge': ge/
    '<=, le': le/
---

# Command syntax #

{% apibody %}
value.lt(value[, value, ...]) &rarr; bool
value < value &rarr; bool
{% endapibody %}

# Description #

Compare values, testing if the left-hand value is less than the right-hand.

__Example:__ Test if a player has scored less than 10 points.

```rb
r.table('players').get(1)['score'].lt(10).run(conn)
# alternative syntax
(r.table('players').get(1)['score'] < 10).run(conn)
```

__Example:__ Test if variables are ordered from highest to lowest, with no values being equal to one another.

```rb
a = 20
b = 10
c = 15
r.lt(a, b, c).run(conn)
```

This is the equivalent of the following:

```rb
r.lt(a, b).and(r.lt(b, c)).run(conn)
```
