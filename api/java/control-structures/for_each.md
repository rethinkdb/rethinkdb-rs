---
layout: api-command
language: Java
permalink: api/javascript/for_each/
command: forEach
related_commands:
    map: map/
---

# Command syntax #

{% apibody %}
sequence.forEach(write_function) &rarr; object
{% endapibody %}

# Description #

Loop over a sequence, evaluating the given write query for each element.

__Example:__ Now that our heroes have defeated their villains, we can safely remove them from the villain table.

```js
r.table('marvel').forEach(function(hero) {
    return r.table('villains').get(hero('villainDefeated')).delete()
}).run(conn)
```
