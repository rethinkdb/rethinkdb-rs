---
layout: api-command
language: Python
permalink: api/python/difference/
command: difference
related_commands:
    union: union/
    set_insert: set_insert/
    set_union: set_union/
    set_intersection: set_intersection/
    set_difference: set_difference/
---

# Command syntax #

{% apibody %}
array.difference(array) &rarr; array
{% endapibody %}

# Description #

Remove the elements of one array from another array.

__Example:__ Retrieve Iron Man's equipment list without boots.

```py
r.table('marvel').get('IronMan')['equipment'].difference(['Boots']).run(conn)
```

__Example:__ Remove Iron Man's boots from his equipment.

```py
r.table('marvel').get('IronMan')['equipment'].update(lambda doc:
    {'equipment': doc['equipment'].difference(['Boots'])}
).run(conn)
```
