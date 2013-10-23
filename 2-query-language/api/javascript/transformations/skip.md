---
layout: api-command 
language: JavaScript
permalink: api/javascript/skip/
command: skip
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/transformations/skip.md
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

{% apibody %}
sequence.skip(n) &rarr; stream
array.skip(n) &rarr; array
{% endapibody %}

Skip a number of elements from the head of the sequence.

__Example:__ Here in conjunction with `order_by` we choose to ignore the most successful heroes.

```js
r.table('marvel').orderBy('successMetric').skip(10).run(conn, callback)
```
