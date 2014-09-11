---
layout: api-command
language: Python
permalink: api/python/includes/
command: includes
related_commands:
    intersects: intersects/
---
# Command syntax #

{% apibody %}
sequence.includes(geometry) &rarr; sequence
geometry.includes(geometry) &rarr; bool
{% endapibody %}

# Description #

Tests whether a geometry object is completely contained within another. When applied to a sequence of geometry objects, `includes` acts as a [filter](/api/python/filter), returning a sequence of objects from the sequence that include the argument.


__Example:__ Is `point2` included within a 2000-meter circle around `point1`?

```py
> point1 = r.point(32.719464,-117.220406)
> point2 = r.point(32.725186,-117.206201)
> r.circle(point1, 2000).includes(point2).run(conn)

True
```

__Example:__ Which of the locations in a list of parks include `circle1`?

```py
circle1 = r.circle([32.719464,-117.220406], 10, unit='mi')
r.table('parks')['area'].includes(circle1).run(conn)
```
