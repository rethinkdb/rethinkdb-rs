---
layout: api-command 
language: JavaScript
permalink: api/javascript/difference/
command: difference 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/document-manipulation/difference.md
io:
    -   - array
        - array
related_commands:
    setInsert: set_insert/
    setUnion: set_union/
    setIntersection: set_intersection/
    setDifference: set_difference/
---

{% apibody %}
array.difference(array) → array
{% endapibody %}

Remove the elements of one array from another array.

__Example:__ Retrieve Iron Man's equipment list without boots.

```js
r.table('marvel').get('IronMan')('equipment').difference(['Boots']).run(conn, callback)
```


