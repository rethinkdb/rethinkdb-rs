---
layout: api-command
language: JavaScript
permalink: api/javascript/difference/
command: difference
io:
    -   - array
        - array
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

```javascript
r.table('marvel').get('IronMan')('equipment')
  .difference(['Boots'])
  .run(conn, callback)
```

__Example:__ Remove Iron Man's boots from his equipment.

```javascript
r.table('marvel').get('IronMan')
  .update({
    equipment: r.row('equipment').difference(['Boots'])
  })
  .run(conn, callback)
```


