---
layout: api-command
language: Java
permalink: api/java/difference/
command: difference
related_commands:
    setInsert: set_insert/
    setUnion: set_union/
    setIntersection: set_intersection/
    setDifference: set_difference/
---

# Command syntax #

{% apibody %}
array.difference(array) &rarr; array
{% endapibody %}

# Description #

Remove the elements of one array from another array.

__Example:__ Retrieve Iron Man's equipment list without boots.

```java
r.table('marvel').get('IronMan')('equipment')
  .difference(['Boots'])
  .run(conn)
```

__Example:__ Remove Iron Man's boots from his equipment.

```java
r.table('marvel').get('IronMan')
  .update({
    equipment: r.row('equipment').difference(['Boots'])
  })
  .run(conn)
```


