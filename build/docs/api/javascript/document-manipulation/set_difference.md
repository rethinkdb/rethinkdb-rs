---
layout: api-command
language: JavaScript
permalink: api/javascript/set_difference/
command: setDifference
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
array.setDifference(array) &rarr; array
{% endapibody %}

# Description #

Remove the elements of one array from another and return them as a set (an array with
distinct values).

__Example:__ Check which pieces of equipment Iron Man has, excluding a fixed list.

```javascript
r.table('marvel').get('IronMan')('equipment').setDifference(['newBoots', 'arc_reactor']).run(conn, callback)
```


