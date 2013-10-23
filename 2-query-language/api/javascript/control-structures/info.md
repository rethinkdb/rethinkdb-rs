---
layout: api-command 
language: JavaScript
permalink: api/javascript/info/
command: info 
github_doc: https://github.com/rethinkdb/docs/blob/docs/2-query-language/api/javascript/control-structures/info.md
io:
    -   - any
        - object
---

{% apibody %}
any.info() &rarr; object
{% endapibody %}

Get information about a RQL value.

__Example:__ Get information about a table such as primary key, or cache size.

```js
r.table('marvel').info().run(conn, callback)
```
