---
layout: api-command
language: Java
permalink: api/java/includes/
command: includes
related_commands:
    intersects: intersects/
---
# Command syntax #

{% apibody %}
sequence.includes(geometry) &rarr; sequence
geometry.includes(geometry) &rarr; bool
{% endapibody %}

# Description #

Tests whether a geometry object is completely contained within another. When applied to a sequence of geometry objects, `includes` acts as a [filter](/api/java/filter), returning a sequence of objects from the sequence that include the argument.


__Example:__ Is a point included within a 2000-meter circle?

```java
Object point1 = r.point(-117.220406,32.719464);
Object point2 = r.point(-117.206201,32.725186);

r.circle(point1, 2000).includes(point2).run(conn);

// Result:
true
```

__Example:__ Which of the locations in a list of parks include a given circle?

```java
import com.rethinkdb.gen.ast.Circle;

Circle circle1 = r.circle(r.array(-117.220406, 32.719464), 10)
                  .optArg("unit", "mi");

r.table("parks").g("area").includes(circle1).run(conn);
```

{% infobox %}
The `includes` command cannot take advantage of a geospatial [secondary index](/docs/secondary-indexes/javascript). If you're working with large data sets, consider using an index and [getIntersecting](/api/javascript/get_intersecting) before `includes` to narrow down the initial result set.
{% endinfobox %}

__Example:__ Rewrite the previous example with `getIntersecting`.

```java
Circle circle1 = r.circle(r.array(-117.220406, 32.719464), 10)
                  .optArg("unit", "mi");

r.table("parks").getIntersecting(circle1)
 .optArg("index", "area").g("area")
 .includes(circle1).run(conn);
```
