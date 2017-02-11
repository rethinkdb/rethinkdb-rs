---
layout: api-command
language: Ruby
permalink: api/ruby/gt/
command: '>, gt'
related_commands:
    '>=, ge': ge/
    '<, lt' : lt/
    '<=, le': le/
---

# Command syntax #

{% apibody %}
value.gt(value[, value, ...]) &rarr; bool
value > value &rarr; bool
{% endapibody %}

# Description #

Compare values, testing if the left-hand value is greater than the right-hand.

__Example:__ Test if a player has scored more than 10 points.

```rb
r.table('players').get(1)['score'].gt(10).run(conn)
# alternative syntax
(r.table('players').get(1)['score'] > 10).run(conn)
```

__Example:__ Test if variables are ordered from lowest to highest, with no values being equal to one another.

```rb
a = 10
b = 20
c = 15
r.gt(a, b, c).run(conn)
```

This is the equivalent of the following:

```rb
r.gt(a, b).and(r.gt(b, c)).run(conn)
```
