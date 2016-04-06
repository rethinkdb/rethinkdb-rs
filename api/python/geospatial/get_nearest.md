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
table.get_nearest(point, index='indexname'[, max_results=100, max_dist=100000, unit='m', geo_system='WGS84']) &rarr; array
{% endapibody %}

# Description #

Return a list of documents closest to a specified point based on a geospatial index, sorted in order of increasing distance.

The `index` argument is mandatory. Optional arguments are:

* `max_results`: the maximum number of results to return (default 100).
* `unit`: Unit for the distance. Possible values are `m` (meter, the default), `km` (kilometer), `mi` (international mile), `nm` (nautical mile), `ft` (international foot).
* `max_dist`: the maximum distance from an object to the specified point (default 100 km).
* `geo_system`: the reference ellipsoid to use for geographic coordinates. Possible values are `WGS84` (the default), a common standard for Earth's geometry, or `unit_sphere`, a perfect sphere of 1 meter radius.

The return value will be an array of two-item objects with the keys `dist` and `doc`, set to the distance between the specified point and the document (in the units specified with `unit`, defaulting to meters) and the document itself, respectively. The array will be sorted by the values of `dist`.

__Example:__ Return a list of the closest 25 enemy hideouts to the secret base.

```py
secret_base = r.point(-122.422876, 37.777128)
r.table('hideouts').get_nearest(secret_base, index='location',
    max_results=25).run(conn)
```

<!-- stop -->

{% infobox %}
If you wish to find all points within a certain radius of another point, it's often faster to use [get_intersecting][gi] with [circle][c], as long as the approximation of a circle that `circle` generates is sufficient.

[gi]: /api/python/get_intersecting/
[c]:  /api/python/circle/
{% endinfobox %}
