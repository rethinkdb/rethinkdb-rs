---
layout: api-command
language: Python
permalink: api/python/expr/
command: expr
---

# Command syntax #

{% apibody %}
r.expr(value) &rarr; value
{% endapibody %}

# Description #

Construct a ReQL JSON object from a native object.

__Example:__ Objects wrapped with expr can then be manipulated by ReQL API functions.

```py
r.expr({'a':'b'}).merge({'b':[1,2,3]}).run(conn)
```


