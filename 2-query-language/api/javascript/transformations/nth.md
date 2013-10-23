---
layout: api-command 
language: JavaScript
permalink: api/javascript/nth/
command: nth 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/transformations/nth.md
io:
    -   - sequence
        - object
related_commands:
    skip: skip/
    limit: limit/
    slice: slice/
---

{% apibody %}
sequence.nth(index) &rarr; object
{% endapibody %}

Get the nth element of a sequence.

__Example:__ Select the second element in the array.

```js
r.expr([1,2,3]).nth(1).run(conn, callback)
```
