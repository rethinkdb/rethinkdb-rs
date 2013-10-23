---
layout: api-command 
language: Python
permalink: api/python/slice/
command: '[] (slice)'
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/transformations/slice.md
related_commands:
    order_by: order_by/
    skip: skip/
    limit: limit/
---

{% apibody %}
sequence[start_index[, end_index]] &rarr; stream
array[start_index[, end_index]] &rarr; array
{% endapibody %}

Trim the sequence to within the bounds provided.

__Example:__ For this fight, we need heroes with a good mix of strength and agility.

```py
r.table('marvel').order_by('strength')[5:10].run(conn)
```


