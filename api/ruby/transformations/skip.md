---
layout: api-command 
language: Ruby
permalink: api/ruby/skip/
command: skip 
related_commands:
    order_by: order_by/
    limit: limit/
    '[]': slice/
---

# Command syntax #

{% apibody %}
sequence.skip(n) &rarr; stream
array.skip(n) &rarr; array
{% endapibody %}

# Description #

Skip a number of elements from the head of the sequence.

__Example:__ Here in conjunction with `order_by` we choose to ignore the most successful heroes.

```rb
r.table('marvel').order_by(:success_metric).skip(10).run(conn)
```


