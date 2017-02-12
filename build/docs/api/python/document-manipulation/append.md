---
layout: api-command
language: Python
permalink: api/python/append/
command: append
related_commands:
    prepend: prepend/
    merge: merge/
---

# Command syntax #

{% apibody %}
array.append(value) &rarr; array
{% endapibody %}

# Description #

Append a value to an array.

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```py
r.table('marvel').get('IronMan')['equipment'].append('newBoots').run(conn)
```


