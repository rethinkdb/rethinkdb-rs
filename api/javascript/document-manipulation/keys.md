---
layout: api-command
language: JavaScript
permalink: api/javascript/keys/
command: keys
io:
    -   - singleSelection
        - array
    -   - object
        - array
related_commands:
    values: values/
---

# Command syntax #

{% apibody %}
singleSelection.keys() &rarr; array
object.keys() &rarr; array
{% endapibody %}

# Description #

Return an array containing all of an object's keys.

__Example:__ Get all the keys from a table row.

```js
// row: { id: 1, name: "fred", email: "fred@example.com" }

r.table('marvel').get('ironman').keys().run(conn, callback);
// Result passed to callback
[ "id", "name", "email" ]
```
