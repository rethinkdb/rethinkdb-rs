---
layout: api-command
language: JavaScript
permalink: api/javascript/get/
command: get
io:
    -   - table
        - singleSelection
related_commands:
    getAll: get_all/
    between: between/
---

# Command syntax #

{% apibody %}
table.get(key) &rarr; singleRowSelection
{% endapibody %}

# Description #

Get a document by primary key.

__Example:__ Find a document with the primary key 'superman'.

```js
r.table('marvel').get('superman').run(conn, callback)
```

If no document exists with that primary key, `get` will return `null`.

__Example:__ Find a document and apply a function to it using [do](/api/javascript/do).

```js
r.table('users').get(3).do(function(user) {
    return user.merge({ powers: ['invisibility', 'speed'] })
}).run(conn, callback)
```
