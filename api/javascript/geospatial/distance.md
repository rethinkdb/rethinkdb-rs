---
layout: api-command
language: JavaScript
permalink: api/javascript/distance/
command: distance
io:
    -   - geometry
        - number
related_commands:
    polygon: polygon/
    line: line/
---
# Command syntax #

{% apibody %}
geometry.distance(geometry[, {geoSystem: 'WGS84', unit: 'm'}]) &rarr; number
r.distance(geometry, geometry[, {geoSystem: 'WGS84', unit: 'm'}]) &rarr; number
{% endapibody %}

# Description #

Compute the distance between a point and another geometry object. At least one of the geometry objects specified must be a point.

Optional arguments available with `distance` are:

* `geoSystem`: the reference ellipsoid to use for geographic coordinates. Possible values are `WGS84` (the default), a common standard for Earth's geometry, or `unit_sphere`, a perfect sphere of 1 meter radius.
* `unit`: Unit to return the distance in. Possible values are `m` (meter, the default), `km` (kilometer), `mi` (international mile), `nm` (nautical mile), `ft` (international foot).

If one of the objects is a polygon or a line, the point will be projected onto the line or polygon assuming a perfect sphere model before the distance is computed (using the model specified with `geoSystem`). As a consequence, if the polygon or line is extremely large compared to Earth's radius and the distance is being computed with the default WGS84 model, the results of `distance` should be considered approximate due to the deviation between the ellipsoid and spherical models.


__Example:__ Compute the distance between two points on the Earth in kilometers.

```js
var point1 = r.point(-122.423246,37.779388);
var point2 = r.point(-117.220406,32.719464);
r.distance(point1, point2, {unit: 'km'}).run(conn, callback);
// result returned to callback 
734.1252496021841
```
