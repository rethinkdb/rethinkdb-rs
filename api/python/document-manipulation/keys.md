---
layout: api-command 
language: Python
permalink: api/python/keys/
command: keys 
---

# Command syntax #

{% apibody %}
singleSelection.keys() &rarr; array
object.keys() &rarr; array
{% endapibody %}

# Description #

Return an array containing all of the object's keys.

__Example:__ Get all the keys of a row.

```py
r.table('marvel').get('ironman').keys().run(conn)
```


