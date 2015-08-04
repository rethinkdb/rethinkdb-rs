---
layout: api-command
language: Ruby
permalink: api/ruby/to_geojson/
command: to_geojson
related_commands:
    geojson: geojson/
---
# Command syntax #

{% apibody %}
geometry.to_geojson() &rarr; object
{% endapibody %}

# Description #

Convert a ReQL geometry object to a [GeoJSON][] object.

[GeoJSON]: http://geojson.org

__Example:__ Convert a ReQL geometry object to a GeoJSON object.

```rb
> r.table('geo').get('sfo')['location'].to_geojson.run(conn)

{
    :type => 'Point',
    :coordinates => [ -122.423246, 37.779388 ]
}
```
