---
layout: api-command
language: Python
permalink: api/python/type_of/
command: type_of
---

# Command syntax #

{% apibody %}
any.type_of() &rarr; string
{% endapibody %}

# Description #

Gets the type of a value.

__Example:__ Get the type of a string.

```py
r.expr("foo").type_of().run(conn)
```


