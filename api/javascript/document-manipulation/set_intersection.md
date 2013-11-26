---
layout: api-command
language: JavaScript
permalink: api/javascript/set_intersection/
command: setIntersection
io:
    -   - array
        - array
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

```js
r.table('marvel').get('IronMan')('equipment').setIntersection(['newBoots', 'arc_reactor']).run(conn, callback)
```

