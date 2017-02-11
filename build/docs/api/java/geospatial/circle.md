---
layout: api-command
language: Java
permalink: api/java/circle/
command: circle
related_commands:
    line: line/
    polygon: polygon/
    point: point/
    distance: distance/
---
# Command syntax #

{% apibody %}
r.circle(r.array(longitude, latitude), radius) &rarr; geometry
r.circle(point, radius) &rarr; geometry
{% endapibody %}

# Description #

Construct a circular line or polygon. A circle in RethinkDB is a polygon or line *approximating* a circle of a given radius around a given center, consisting of a specified number of vertices (default 32).

The center may be specified either by two floating point numbers, the latitude (&minus;90 to 90) and longitude (&minus;180 to 180) of the point on a perfect sphere (see [Geospatial support](/docs/geo-support/) for more information on ReQL's coordinate system), or by a point object. The radius is a floating point number whose units are meters by default, although that may be changed with the `unit` argument.

Optional arguments that can be specified with [optArg](/api/java/optarg) are:

* `num_vertices`: the number of vertices in the polygon or line. Defaults to 32.
* `geo_system`: the reference ellipsoid to use for geographic coordinates. Possible values are `WGS84` (the default), a common standard for Earth's geometry, or `unit_sphere`, a perfect sphere of 1 meter radius.
* `unit`: Unit for the radius distance. Possible values are `m` (meter, the default), `km` (kilometer), `mi` (international mile), `nm` (nautical mile), `ft` (international foot).
* `fill`: if `true` (the default) the circle is filled, creating a polygon; if `false` the circle is unfilled (creating a line).



__Example:__ Define a circle.

```java
r.table("geo").insert(
    r.hashMap("id", 300)
     .with("name", "Hayes Valley")
     .with("neighborhood", r.circle(r.array(-122.423246, 37.779388), 1000))
).run(conn);
```
