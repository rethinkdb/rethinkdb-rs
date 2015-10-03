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

Add a several values to an array and return it as a set (an array with distinct values).

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots and an arc reactor.

```js
r.table('marvel').get('IronMan')('equipment').setUnion(['newBoots', 'arc_reactor']).run(conn)
```

