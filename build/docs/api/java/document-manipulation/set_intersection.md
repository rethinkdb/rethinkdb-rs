---
layout: api-command
language: Java
permalink: api/java/set_intersection/
command: setIntersection
related_commands:
    difference: difference/
    setInsert: set_insert/
    setUnion: set_union/
    setDifference: set_difference/
---

# Command syntax #

{% apibody %}
array.setIntersection(array) &rarr; array
{% endapibody %}

# Description #

Intersect two arrays returning values that occur in both of them as a set (an array with
distinct values).

__Example:__ Check which pieces of equipment Iron Man has from a fixed list.

```java
r.table("marvel").get("IronMan").g("equipment")
 .setIntersection(r.array("newBoots", "arc_reactor")).run(conn);
```

