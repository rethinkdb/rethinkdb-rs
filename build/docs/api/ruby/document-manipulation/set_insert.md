---
layout: api-command
language: Ruby
permalink: api/ruby/set_insert/
command: set_insert
related_commands:
    union: union/
    difference: difference/
    set_union: set_union/
    set_intersection: set_intersection/
    set_difference: set_difference/
---

# Command syntax #

{% apibody %}
array.set_insert(value) &rarr; array
{% endapibody %}

# Description #

Add a value to an array and return it as a set (an array with distinct values).

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```rb
r.table('marvel').get('IronMan')[:equipment].set_insert('new_boots').run(conn)
```


