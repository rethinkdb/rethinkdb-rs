---
layout: api-command
language: Java
permalink: api/java/set_union/
command: setUnion
related_commands:
    difference: difference/
    setInsert: set_insert/
    setIntersection: set_intersection/
    setDifference: set_difference/
---

# Command syntax #

{% apibody %}
array.setUnion(array) &rarr; array
{% endapibody %}

# Description #

Perform a set intersection of two arrays, returning an array with all unique items from both.

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots and an arc reactor.

```java
r.table("marvel").get("IronMan").g("equipment")
 .setUnion(r.array("newBoots", "arc_reactor")).run(conn);
```

