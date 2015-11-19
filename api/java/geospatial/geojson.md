---
layout: api-command
language: Java
permalink: api/java/geojson/
command: geojson
related_commands:
    toGeojson: to_geojson/
---
# Command syntax #

{% apibody %}
r.geojson(geojson) &rarr; geometry
{% endapibody %}

# Description #

Convert a [GeoJSON](http://geojson.org) object to a ReQL geometry object.

RethinkDB only allows conversion of GeoJSON objects which have ReQL equivalents: `Point`, `LineString`, and `Polygon`. `MultiPoint`, `MultiLineString`, and `MultiPolygon` are not supported. (You could, however, store multiple points, lines and polygons in an array and use a geospatial multi index with them.)

Only longitude/latitude coordinates are supported. GeoJSON objects that use Cartesian coordinates, specify an altitude, or specify their own coordinate reference system will be rejected.

__Example:__ Convert a GeoJSON object to a ReQL geometry object.

```java
import com.rethinkdb.model.Geojson;

// GeoJSON object:
//      {
//          "type": "Point",
//          "coordinates": [ -122.423246, 37.779388 ]
//      }
Geojson geo = r.hashMap("type, "Point")
               .with("coordinates", r.array(-122.423246, 37.779388));

r.table("geo").insert(
    r.hashMap("id", "sfo")
     .with("name", "San Francisco")
     .with("location", r.geojson(geo))
).run(conn);
```
