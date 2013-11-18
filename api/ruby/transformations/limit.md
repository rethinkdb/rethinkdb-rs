---
layout: api-command 
language: Ruby
permalink: api/ruby/limit/
command: limit 
related_commands:
    order_by: order_by/
    skip: skip/
    '[]': slice/
---

# Command syntax #

{% apibody %}
sequence.limit(n) &rarr; stream
array.limit(n) &rarr; array
{% endapibody %}

# Description #


End the sequence after the given number of elements.

__Example:__ Only so many can fit in our Pantheon of heroes.

```rb
r.table('marvel').order_by(:belovedness).limit(10).run(conn)
```


