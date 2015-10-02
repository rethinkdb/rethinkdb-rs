---
layout: api-command
language: JavaScript
permalink: api/javascript/geojson/
command: geojson
io:
    -   - r
        - geometry
related_commands:
    toGeojson: to_geojson/
---
# Command syntax #

{% apibody %}
r.geojson(geojson) &rarr; geometry
{% endapibody %}

# Description #

Convert a [GeoJSON][] object to a ReQL geometry object.

[GeoJSON]: http://geojson.org

RethinkDB only allows conversion of GeoJSON objects which have ReQL equivalents: Point, LineString, and Polygon. MultiPoint, MultiLineString, and MultiPolygon are not supported. (You could, however, store multiple points, lines and polygons in an array and use a geospatial multi index with them.)

Only longitude/latitude coordinates are supported. GeoJSON objects that use Cartesian coordinates, specify an altitude, or specify their own coordinate reference system will be rejected.

__Example:__ Convert a GeoJSON object to a ReQL geometry object.

```js
var geoJson = {
    'type': 'Point',
    'coordinates': [ -122.423246, 37.779388 ]
};
r.table('geo').insert({
    id: 'sfo',
    name: 'San Francisco',
    location: r.geojson(geoJson)
}).run(conn);
```
