---
layout: api-command
language: Java
permalink: api/java/get_intersecting/
command: getIntersecting
related_commands:
    getNearest: get_nearest/
---

# Command syntax #

{% apibody %}
table.getIntersecting(geometry).optArg("index", index) &rarr; selection<stream>
{% endapibody %}

# Description #

Get all documents where the given geometry object intersects the geometry object of the requested geospatial index.

The `index` [optarg](/api/java/optarg) is mandatory. This command returns the same results as `row -> row.g(index).intersects(geometry)`. The total number of results is limited to the array size limit which defaults to 100,000, but can be changed with the `array_limit` option to [run](/api/java/run).

__Example:__ Which of the locations in a list of parks intersect `circle1`?

```java
import com.rethinkdb.gen.ast.Circle;

Circle circle1 = r.circle(r.array(-117.220406, 32.719464), 10)
                  .optArg("unit", "mi");

r.table("parks").getIntersecting(circle1).optArg("index", "area").run(conn);
```
