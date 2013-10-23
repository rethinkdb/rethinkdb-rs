---
layout: api-command 
language: JavaScript
permalink: api/javascript/is_empty/
command: isEmpty 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/transformations/isEmpty.md
io:
    -   - sequence
        - bool
---

{% apibody %}
sequence.isEmpty() &rarr; bool
{% endapibody %}

Test if a sequence is empty.

__Example:__ Are there any documents in the marvel table?

```js
r.table('marvel').isEmpty().run(conn, callback)
```
