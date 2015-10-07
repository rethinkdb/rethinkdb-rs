---
layout: api-command
language: Python
permalink: api/python/intersects/
command: intersects
related_commands:
    includes: includes/
    get_intersecting: get_intersecting/
---
# Command syntax #

{% apibody %}
sequence.intersects(geometry) &rarr; sequence
geometry.intersects(geometry) &rarr; bool
r.intersects(sequence, geometry) &rarr; sequence
r.intersects(geometry, geometry) &rarr; bool
{% endapibody %}

# Description #

Tests whether two geometry objects intersect with one another. When applied to a sequence of geometry objects, `intersects` acts as a [filter](/api/python/filter), returning a sequence of objects from the sequence that intersect with the argument.


__Example:__ Is `point2` within a 2000-meter circle around `point1`?

```py
> point1 = r.point(-117.220406, 32.719464)
> point2 = r.point(-117.206201, 32.725186)
> r.circle(point1, 2000).intersects(point2).run(conn)

True
```

__Example:__ Which of the locations in a list of parks intersect `circle1`?

```py
circle1 = r.circle([-117.220406, 32.719464], 10, unit='mi')
r.table('parks')('area').intersects(circle1).run(conn)
```
