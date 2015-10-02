---
layout: api-command
language: JavaScript
permalink: api/javascript/skip/
command: skip
io:
    -   - sequence
        - stream
    -   - array
        - array
related_commands:
    limit: limit/
    slice: slice/
    nth: nth/
---

# Command syntax #

{% apibody %}
sequence.skip(n) &rarr; stream
array.skip(n) &rarr; array
{% endapibody %}

# Description #

Skip a number of elements from the head of the sequence.

__Example:__ Here in conjunction with [orderBy](/api/javascript/order_by/) we choose to ignore the most successful heroes.

```js
r.table('marvel').orderBy('successMetric').skip(10).run(conn)
```
