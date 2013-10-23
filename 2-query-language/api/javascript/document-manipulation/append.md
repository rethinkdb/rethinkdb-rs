---
layout: api-command 
language: JavaScript
permalink: api/javascript/append/
command: append 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/document-manipulation/append.md
io:
    -   - array
        - array
related_commands:
    prepend: prepend/
    insertAt: insert_at/
    deleteAt: delete_at/
    changeAt: change_at/
---
{% apibody %}
array.append(value) → array
{% endapibody %}

Append a value to an array.

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```js
r.table('marvel').get('IronMan')('equipment').append('newBoots').run(conn, callback)
```


