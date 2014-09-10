---
layout: api-command
language: Python
permalink: api/python/get_intersecting/
command: getIntersecting
io:
    -   - table
        - array
related_commands:
    getNearest: get_nearest/
---

# Command syntax #

{% apibody %}
table.getIntersecting(geometry, {index: 'indexname'}) &rarr; selection<array>
{% endapibody %}

# Description #

Get all documents where the given geometry object intersects the geometry object of the requested geospatial index.

The `index` argument is mandatory. This command returns the same results as `table.filter(r.row('index').intersects(geometry))`. The total number of results is limited to the array size limit which defaults to 100,000, but can be changed with the `arrayLimit` option to [run](/api/python/run).

__Example:__ Which of the locations in a list of parks intersect `circle1`?

```py
var circle1 = r.circle([32.719464,-117.220406], 10, {unit: 'mi'});
r.table('parks').getIntersecting(circle1, {index: 'area'}).run(conn, callback);
```
