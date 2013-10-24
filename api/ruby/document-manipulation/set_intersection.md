---
layout: api-command 
language: Ruby
permalink: api/ruby/set_intersection/
command: set_intersection
related_commands:
    union: union/
    difference: difference/
    set_insert: set_insert/
    set_difference: set_difference/
    set_intersection: set_intersection/
---

# Command syntax #

{% apibody %}
array.set_intersection(array) &rarr; array
{% endapibody %}

# Description #

Intersect two arrays returning values that occur in both of them as a set (an array with
distinct values).

__Example:__ Check which pieces of equipment Iron Man has from a fixed list.

```rb
r.table('marvel').get('IronMan')[:equipment].set_intersection(['newBoots', 'arc_reactor']).run(conn)
```


