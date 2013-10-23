---
layout: api-command 
language: Python
permalink: api/python/difference/
command: difference 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/document-manipulation/difference.md
related_commands:
    union: union/
    set_insert: set_insert/
    set_union: set_union/
    set_intersection: set_intersection/
    set_difference: set_difference/
---

{% apibody %}
array.difference(array) &rarr; array
{% endapibody %}

Remove the elements of one array from another array.

__Example:__ Retrieve Iron Man's equipment list without boots.

```py
r.table('marvel').get('IronMan')['equipment'].difference(['Boots']).run(conn)
```


