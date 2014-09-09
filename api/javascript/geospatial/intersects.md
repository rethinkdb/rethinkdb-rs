---
layout: api-command
language: JavaScript
permalink: api/javascript/intersects/
command: intersects
io:
    -   - geometry
        - bool
related_commands:
    includes: includes/
---
# Command syntax #

{% apibody %}
sequence.intersects(geometry) &rarr; sequence
geometry.intersects(geometry) &rarr; bool
{% endapibody %}

# Description #

Tests whether two geometry objects intersect with one another. When applied to a sequence of geometry objects, `intersects` acts as a [filter](/api/javascript/filter), returning a sequence of objects from the sequence that intersect with the argument.


__Example:__ Is `point2` within a 2000-meter circle around `point1`?

```js
var point1 = r.point(32.719464,-117.220406);
var point2 = r.point(32.725186,-117.206201);
r.circle(point1, 2000).intersects(point2).run(conn, callback);
// result returned to callback 
true
```

__Example:__ Which of the locations in a list of parks intersect `circle1`?

```js
var circle1 = r.circle([32.719464,-117.220406], 10, {unit: 'mi'});
r.table('parks')('area').intersects(circle1).run(conn, callback);
```
