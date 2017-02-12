---
layout: api-command
language: JavaScript
permalink: api/javascript/append/
command: append
io:
    -   - array
        - array
related_commands:
    prepend: prepend/
    insertAt: insert_at/
    deleteAt: delete_at/
    changeAt: change_at/
---
# Command syntax #

{% apibody %}
array.append(value) &rarr; array
{% endapibody %}

# Description #

Append a value to an array.

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```js
r.table('marvel').get('IronMan')('equipment').append('newBoots').run(conn, callback)
```


