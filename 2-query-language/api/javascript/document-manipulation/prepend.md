---
layout: api-command 
language: JavaScript
permalink: api/javascript/prepend/
command: prepend 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/document-manipulation/prepend.md
io:
    -   - array
        - array
related_commands:
    append: append/
    insertAt: insert_at/
    deleteAt: delete_at/
    changeAt: change_at/
---

{% apibody %}
array.prepend(value) &rarr; array
{% endapibody %}

Prepend a value to an array.

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```js
r.table('marvel').get('IronMan')('equipment').prepend('newBoots').run(conn, callback)
```


