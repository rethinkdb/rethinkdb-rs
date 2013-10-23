---
layout: api-command 
language: Python
permalink: api/python/nth/
command: '[] (nth)'
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/transformations/nth.md
related_commands:
    order_by: order_by/
    skip: skip/
    limit: limit/
    '[] (slice)': slice/
---

# Command syntax #

{% apibody %}
sequence.nth(index) &rarr; object
{% endapibody %}

# Description #

Get the nth element of a sequence.

__Example:__ Select the second element in the array.

```py
r.expr([1,2,3])[1].run(conn)
```


