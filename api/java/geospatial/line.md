---
layout: api-command
language: Java
permalink: api/java/line/
command: line
related_commands:
    point: point/
    polygon: polygon/
    circle: circle/
---
# Command syntax #

{% apibody %}
r.line([lon1, lat1], [lon2, lat2], ...) &rarr; line
r.line(point1, point2, ...) &rarr; line
{% endapibody %}

# Description #

Construct a geometry object of type Line. The line can be specified in one of two ways:

* Two or more two-item arrays, specifying latitude and longitude numbers of the line's vertices;
* Two or more [Point](/api/java/point) objects specifying the line's vertices.

Longitude (&minus;180 to 180) and latitude (&minus;90 to 90) of vertices are plotted on a perfect sphere. See [Geospatial support](/docs/geo-support/) for more information on ReQL's coordinate system.

__Example:__ Define a line.

```java
r.table('geo').insert({
    id: 101,
    route: r.line([-122.423246,37.779388], [-121.886420,37.329898])
}).run(conn);
```
