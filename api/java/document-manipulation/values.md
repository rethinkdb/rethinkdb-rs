---
layout: api-command
language: Java
permalink: api/javascript/values/
command: values
related_commands:
    keys: keys/
---

# Command syntax #

{% apibody %}
singleSelection.values() &rarr; array
object.values() &rarr; array
{% endapibody %}

# Description #

Return an array containing all of an object's values. `values()` guarantees the values will come out in the same order as [keys](/api/javascript/keys).

__Example:__ Get all of the values from a table row.

```js
// row: { id: 1, mail: "fred@example.com", name: "fred" }

r.table('users').get(1).values().run(conn);
// Result passed to callback
[ 1, "fred@example.com", "fred" ]
```
