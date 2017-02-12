---
layout: api-command
language: Python
permalink: api/python/prepend/
command: prepend
related_commands:
    append: append/
    merge: merge/
---

# Command syntax #

{% apibody %}
array.prepend(value) &rarr; array
{% endapibody %}

# Description #

Prepend a value to an array.

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```py
r.table('marvel').get('IronMan')['equipment'].prepend('newBoots').run(conn)
```
