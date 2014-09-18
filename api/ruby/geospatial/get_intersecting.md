---
layout: api-command
language: Ruby
permalink: api/ruby/get_intersecting/
command: get_intersecting
related_commands:
    get_nearest: get_nearest/
---

# Command syntax #

{% apibody %}
table.get_intersecting(geometry, {:index => 'indexname'}) &rarr; selection<array>
{% endapibody %}

# Description #

Get all documents where the given geometry object intersects the geometry object of the requested geospatial index.

The `index` argument is mandatory. This command returns the same results as `table.filter(r.row('index').intersects(geometry))`. The total number of results is limited to the array size limit which defaults to 100,000, but can be changed with the `array_limit` option to [run](/api/ruby/run).

__Example:__ Which of the locations in a list of parks intersect `circle1`?

```rb
circle1 = r.circle([-117.220406,32.719464], 10, {:unit => 'mi'})
r.table('parks').get_intersecting(circle1, {:index => 'area'}).run(conn)
```
