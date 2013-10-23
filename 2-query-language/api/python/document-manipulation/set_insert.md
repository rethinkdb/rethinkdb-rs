---
layout: api-command 
language: Python
permalink: api/python/set_insert/
command: set_insert 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/document-manipulation/set_insert.md
related_commands:
    union: union/
    difference: difference/
    set_insert: set_insert/
    set_union: set_union/
    set_intersection: set_intersection/
    set_difference: set_difference/
---

{% apibody %}
array.set_insert(value) &rarr; array
{% endapibody %}

Add a value to an array and return it as a set (an array with distinct values).

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```py
r.table('marvel').get('IronMan')['equipment'].set_insert('newBoots').run(conn)
```


