---
layout: api-command 
language: Python
permalink: api/python/keys/
command: keys 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/document-manipulation/keys.md
---

{% apibody %}
singleSelection.keys() → array
object.keys() → array
{% endapibody %}

Return an array containing all of the object's keys.

__Example:__ Get all the keys of a row.

```py
r.table('marvel').get('ironman').keys().run(conn)
```


