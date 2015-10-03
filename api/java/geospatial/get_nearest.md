---
layout: api-command
language: JavaScript
permalink: api/javascript/get_nearest/
command: getNearest
related_commands:
    getIntersecting: get_intersecting/
---

# Command syntax #

{% apibody %}
table.getNearest(point, {index: 'indexname'[, maxResults: 100, maxDist: 100000, unit: 'm', geoSystem: 'WGS84']}) &rarr; array
{% endapibody %}

# Description #

Get all documents where the specified geospatial index is within a certain distance of the specified point (default 100 kilometers).

The `index` argument is mandatory. Optional arguments are:

* `maxResults`: the maximum number of results to return (default 100).
* `unit`: Unit for the distance. Possible values are `m` (meter, the default), `km` (kilometer), `mi` (international mile), `nm` (nautical mile), `ft` (international foot).
* `maxDist`: the maximum distance from an object to the specified point (default 100 km).
* `geoSystem`: the reference ellipsoid to use for geographic coordinates. Possible values are `WGS84` (the default), a common standard for Earth's geometry, or `unit_sphere`, a perfect sphere of 1 meter radius.

The return value will be an array of two-item objects with the keys `dist` and `doc`, set to the distance between the specified point and the document (in the units specified with `unit`, defaulting to meters) and the document itself, respectively.

__Example:__ Return a list of enemy hideouts within 5000 meters of the secret base.

```js
var secretBase = r.point(-122.422876,37.777128);
r.table('hideouts').getNearest(secretBase,
    {index: 'location', maxDist: 5000}
).run(conn)
```
