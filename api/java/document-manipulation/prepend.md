---
layout: api-command
language: Java
permalink: api/java/prepend/
command: prepend
related_commands:
    append: append/
    insertAt: insert_at/
    deleteAt: delete_at/
    changeAt: change_at/
---

# Command syntax #

{% apibody %}
array.prepend(value) &rarr; array
{% endapibody %}

# Description #

Prepend a value to an array.

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```js
r.table('marvel').get('IronMan')('equipment').prepend('newBoots').run(conn)
```


