---
layout: api-command
language: JavaScript
permalink: api/javascript/circle/
command: circle
io:
    -   - r
        - geometry
related_commands:
    line: line/
    polygon: polygon/
    point: point/
    distance: distance/
---
# Command syntax #

{% apibody %}
r.circle([latitude, longitude], radius[, {numVertices: 32, geoSystem: 'WGS84', unit: 'm', fill: true}]) &rarr; geometry
r.circle(point, radius[, {numVertices: 32, geoSystem: 'WGS84', unit: 'm', fill: true}]) &rarr; geometry
{% endapibody %}

# Description #

Construct a circular line or polygon. A circle in RethinkDB is a polygon or line *approximating* a circle of a given radius around a given center, consisting of a specified number of vertices (default 32).

The center may be specified either by two floating point numbers, the latitude (&minus;90 to 90) and longitude (&minus;180 to 180) of the point on a perfect sphere (see [Geospatial support](/docs/geo-support/) for more information on ReQL's coordinate system), or by a point object. The radius is a floating point number whose units are meters by default, although that may be changed with the `unit` argument.

Optional arguments available with `circle` are:

* `numVertices`: the number of vertices in the polygon or line. Defaults to 32.
* `geoSystem`: the reference ellipsoid to use for geographic coordinates. Possible values are `WGS84` (the default), a common standard for Earth's geometry, or `unit_sphere`, a perfect sphere of 1 meter radius.
* `unit`: Unit for the radius distance. Possible values are `m` (meter, the default), `km` (kilometer), `mi` (international mile), `nm` (nautical mile), `ft` (international foot).
* `fill`: if `true` (the default) the circle is filled, creating a polygon; if `false` the circle is unfilled (creating a line).



__Example:__ Define a circle.

```js
r.table('geo').insert({
    id: 300,
    name: 'Hayes Valley',
    neighborhood: r.circle([37.779388,-122.423246], 1000)
}).run(conn, callback);
```
