---
layout: api-command
language: JavaScript
permalink: api/javascript/includes/
command: includes
io:
    -   - geometry
        - bool
    -   - sequence
        - sequence
related_commands:
    intersects: intersects/
---
# Command syntax #

{% apibody %}
sequence.includes(geometry) &rarr; sequence
geometry.includes(geometry) &rarr; bool
{% endapibody %}

# Description #

Tests whether a geometry object is completely contained within another. When applied to a sequence of geometry objects, `includes` acts as a [filter](/api/javascript/filter), returning a sequence of objects from the sequence that include the argument.


__Example:__ Is `point2` included within a 2000-meter circle around `point1`?

```js
var point1 = r.point(-117.220406,32.719464);
var point2 = r.point(-117.206201,32.725186);
r.circle(point1, 2000).includes(point2).run(conn, callback);
// result returned to callback 
true
```

__Example:__ Which of the locations in a list of parks include `circle1`?

```js
var circle1 = r.circle([-117.220406,32.719464], 10, {unit: 'mi'});
r.table('parks')('area').includes(circle1).run(conn, callback);
```

{% infobox %}
The `includes` command cannot take advantage of a geospatial [secondary index](/docs/secondary-indexes/javascript). If you're working with large data sets, consider using an index and [getIntersecting](/api/javascript/get_intersecting) before `includes` to narrow down the initial result set.
{% endinfobox %}

__Example:__ Rewrite the previous example with `getIntersecting`.

```js
var circle1 = r.circle([-117.220406,32.719464], 10, {unit: 'mi'});
r.table('parks').getIntersecting(circle1, {index: 'area'})('area').
    includes(circle1).run(conn, callback);
```
