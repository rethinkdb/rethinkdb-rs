---
layout: api-command
language: JavaScript
permalink: api/javascript/set_insert/
command: setInsert
io:
    -   - array
        - array
related_commands:
    difference: difference/
    setUnion: set_union/
    setIntersection: set_intersection/
    setDifference: set_difference/
---

# Command syntax #

{% apibody %}
array.setInsert(value) &rarr; array
{% endapibody %}

# Description #

Add a value to an array and return it as a set (an array with distinct values).

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```js
r.table('marvel').get('IronMan')('equipment').setInsert('newBoots').run(conn)
```


