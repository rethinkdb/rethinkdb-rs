---
layout: api-command 
language: Python
permalink: api/python/prepend/
command: prepend 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/document-manipulation/prepend.md
related_commands:
    append: append/
    merge: merge/
---

{% apibody %}
array.prepend(value) → array
{% endapibody %}

Prepend a value to an array.

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```py
r.table('marvel').get('IronMan')['equipment'].prepend('newBoots').run(conn)
```
