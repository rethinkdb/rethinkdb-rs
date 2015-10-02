---
layout: api-command
language: JavaScript
permalink: api/javascript/to_geojson/
command: toGeojson
io:
    -   - geometry
        - object
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

```js
r.table('geo').get('sfo')('location').toGeojson().run(conn);
// result passed to callback
{
    'type': 'Point',
    'coordinates': [ -122.423246, 37.779388 ]
}
```
