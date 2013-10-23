---
layout: api-command 
language: Python
permalink: api/python/append/
command: append 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/document-manipulation/append.md
related_commands:
    prepend: prepend/
    merge: merge/
---

{% apibody %}
array.append(value) → array
{% endapibody %}

Append a value to an array.

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```py
r.table('marvel').get('IronMan')['equipment'].append('newBoots').run(conn)
```


