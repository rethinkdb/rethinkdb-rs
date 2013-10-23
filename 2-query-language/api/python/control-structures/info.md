---
layout: api-command 
language: Python
permalink: api/python/info/
command: info 
github_doc: https://github.com/rethinkdb/docs/blob/docs/2-query-language/api/python/control-structures/info.md
---

{% apibody %}
any.info() &rarr; object
{% endapibody %}

Get information about a RQL value.

__Example:__ Get information about a table such as primary key, or cache size.

```py
r.table('marvel').info().run(conn)
```


