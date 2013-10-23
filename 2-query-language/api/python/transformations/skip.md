---
layout: api-command 
language: Python
permalink: api/python/skip/
command: skip 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/transformations/skip.md
related_commands:
    order_by: order_by/
    limit: limit/
    '[]': slice/
---

{% apibody %}
sequence.skip(n) &rarr; stream
array.skip(n) &rarr; array
{% endapibody %}

Skip a number of elements from the head of the sequence.

__Example:__ Here in conjunction with `order_by` we choose to ignore the most successful heroes.

```py
r.table('marvel').order_by('successMetric').skip(10).run(conn)
```


