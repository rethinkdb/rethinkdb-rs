---
layout: api-command
language: Python
permalink: api/python/point/
command: point
related_commands:
    line: line/
    polygon: polygon/
    circle: circle/
---
# Command syntax #

{% apibody %}
r.point(latitude, longitude) &rarr; point
{% endapibody %}

# Description #

Construct a geometry object of type Point. The point is specified by two floating point numbers, the latitude (&minus;90 to 90) and longitude (&minus;180 to 180) of the point on a perfect sphere. See [Geospatial support](/docs/geo-support/) for more information on ReQL's coordinate system.

__Example:__ Define a point.

```py
r.table('geo').insert({
    'id': 1,
    'name': 'San Francisco',
    'location': r.point(37.779388,-122.423246)
}).run(conn)
```
