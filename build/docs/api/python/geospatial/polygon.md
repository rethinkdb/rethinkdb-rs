---
layout: api-command
language: Python
permalink: api/python/polygon/
command: polygon
related_commands:
    point: point/
    line: line/
    circle: circle/
---
# Command syntax #

{% apibody %}
r.polygon([lon1, lat1], [lon2, lat2], [lon3, lat3], ...) &rarr; polygon
r.polygon(point1, point2, point3, ...) &rarr; polygon
{% endapibody %}

# Description #

Construct a geometry object of type Polygon. The Polygon can be specified in one of two ways:

* Three or more two-item arrays, specifying latitude and longitude numbers of the polygon's vertices;
* Three or more [Point](/api/python/point) objects specifying the polygon's vertices.

<!-- break -->

Longitude (&minus;180 to 180) and latitude (&minus;90 to 90) of vertices are plotted on a perfect sphere. See [Geospatial support](/docs/geo-support/) for more information on ReQL's coordinate system.

If the last point does not specify the same coordinates as the first point, `polygon` will close the polygon by connecting them. You cannot directly construct a polygon with holes in it using `polygon`, but you can use [polygon_sub](/api/python/polygon_sub) to use a second polygon within the interior of the first to define a hole.


__Example:__ Define a polygon.

```py
r.table('geo').insert({
    'id': 101,
    'rectangle': r.polygon(
        [-122.423246, 37.779388],
        [-122.423246, 37.329898],
        [-121.886420, 37.329898],
        [-121.886420, 37.779388]
    )
}).run(conn)
```

__Example:__ Define a polygon using an array of vertices.

You can use the [args](/api/python/args) command to pass an array of Point objects (or latitude-longitude pairs) to `polygon`.

```py
vertices = [
    [-122.423246, 37.779388],
    [-122.423246, 37.329898],
    [-121.886420, 37.329898],
    [-121.886420, 37.779388]
]
r.table('geo').insert({
    'id': 102,
    'rectangle': r.polygon(r.args(vertices))
}).run(conn)
```
