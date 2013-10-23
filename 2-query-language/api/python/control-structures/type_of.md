---
layout: api-command 
language: Python
permalink: api/python/type_of/
command: type_of 
github_doc: https://github.com/rethinkdb/docs/blob/docs/2-query-language/api/python/control-structures/type_of.md
---

{% apibody %}
any.type_of() → string
{% endapibody %}

Gets the type of a value.

__Example:__ Get the type of a string.

```py
r.expr("foo").type_of().run(conn)
```


