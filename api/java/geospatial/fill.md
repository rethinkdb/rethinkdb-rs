---
layout: api-command
language: JavaScript
permalink: api/javascript/fill/
command: fill
io:
    -   - line
        - polygon
related_commands:
    polygon: polygon/
    line: line/
---
# Command syntax #

{% apibody %}
line.fill() &rarr; polygon
{% endapibody %}

# Description #

Convert a Line object into a Polygon object. If the last point does not specify the same coordinates as the first point, `polygon` will close the polygon by connecting them.

Longitude (&minus;180 to 180) and latitude (&minus;90 to 90) of vertices are plotted on a perfect sphere. See [Geospatial support](/docs/geo-support/) for more information on ReQL's coordinate system.

If the last point does not specify the same coordinates as the first point, `polygon` will close the polygon by connecting them. You cannot directly construct a polygon with holes in it using `polygon`, but you can use [polygonSub](/api/javascript/polygon_sub) to use a second polygon within the interior of the first to define a hole.


__Example:__ Create a line object and then convert it to a polygon.

```js
r.table('geo').insert({
    id: 201,
    rectangle: r.line(
        [-122.423246,37.779388],
        [-122.423246,37.329898],
        [-121.886420,37.329898],
        [-121.886420,37.779388]
    )
}).run(conn, callback);

r.table('geo').get(201).update({
    rectangle: r.row('rectangle').fill()
}, {nonAtomic: true}).run(conn, callback);
```
