---
layout: api-command
language: Python
permalink: api/python/get_nearest/
command: get_nearest
related_commands:
    get_intersecting: get_intersecting/
---

# Command syntax #

{% apibody %}
table.get_nearest(point, index='indexname'[, max_results=100, max_dist=100000, unit='m', geo_system='WGS84']) &rarr; selection<array>
{% endapibody %}

# Description #

Get all documents where the specified geospatial index is within a certain distance of the specified point (default 100 kilometers).

The `index` argument is mandatory. This command returns the same results as `table.filter(r.row('index').intersects(geometry))`. The total number of results is limited to the array size limit which defaults to 100,000, but can be changed with the `array_limit` option to [run](/api/python/run).

Optional arguments are:

* `max_results`: the maximum number of results to return (default 100).
* `unit`: Unit for the distance. Possible values are `m` (meter, the default), `km` (kilometer), `mi` (international mile), `nm` (nautical mile), `ft` (international foot).
* `max_dist`: the maximum distance from an object to the specified point (default 100 km).
* `geo_system`: the reference ellipsoid to use for geographic coordinates. Possible values are `WGS84` (the default), a common standard for Earth's geometry, or `unit_sphere`, a perfect sphere of 1 meter radius.

__Example:__ Return a list of enemy hideouts within 5000 meters of the secret base.

```py
secret_base = r.point(37.777128,-122.422876)
r.table('hideouts').get_nearest(secret_base,
    {index: 'location', max_dist=5000}
).run(conn)
```
