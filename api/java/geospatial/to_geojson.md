---
layout: api-command
language: Java
permalink: api/java/to_geojson/
command: toGeojson
related_commands:
    geojson: geojson/
---
# Command syntax #

{% apibody %}
geometry.toGeojson() &rarr; object
{% endapibody %}

# Description #

Convert a ReQL geometry object to a [GeoJSON][] object.

[GeoJSON]: http://geojson.org

__Example:__ Convert a ReQL geometry object to a GeoJSON object.

```java
r.table("geo").get("sfo")("location").toGeojson().run(conn);

// Result:
{
    "type": "Point",
    "coordinates": [ -122.423246, 37.779388 ]
}
```
