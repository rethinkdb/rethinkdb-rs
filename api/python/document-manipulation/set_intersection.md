---
layout: api-command 
language: Python
permalink: api/python/set_intersection/
command: set_intersection 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/document-manipulation/set_intersection.md
related_commands:
    union: union/
    difference: difference/
    set_insert: set_insert/
    set_union: set_union/
    set_difference: set_difference/
---


# Command syntax #

{% apibody %}
array.set_intersection(array) &rarr; array
{% endapibody %}

# Description #

Intersect two arrays returning values that occur in both of them as a set (an array with
distinct values).

__Example:__ Check which pieces of equipment Iron Man has from a fixed list.

```py
r.table('marvel').get('IronMan')['equipment'].set_intersection(['newBoots', 'arc_reactor']).run(conn)
```



