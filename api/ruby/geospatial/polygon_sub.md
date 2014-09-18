---
layout: api-command
language: Ruby
permalink: api/ruby/polygon_sub/
command: polygon_sub
related_commands:
    polygon: polygon/
---

# Command syntax #

{% apibody %}
polygon1.polygon_sub(polygon2) &rarr; polygon
{% endapibody %}

# Description #

Use `polygon2` to "punch out" a hole in `polygon1`. `polygon2` must be completely contained within `polygon1` and must have no holes itself (it must not be the output of `polygon_sub` itself).


__Example:__ Define a polygon with a hole punched in it.

```rb
outer_polygon = r.polygon(
    [-122.4,37.7],
    [-122.4,37.3],
    [-121.8,37.3],
    [-121.8,37.7]
)
inner_polygon = r.polygon(
    [-122.3,37.4],
    [-122.3,37.6],
    [-122.0,37.6],
    [-122.0,37.4]
)
outer_polygon.polygon_sub(inner_polygon).run(conn)
```
