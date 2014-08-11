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

If the native object is of the `bytes` type, then `expr` will return a binary object. See [binary](/api/python/binary) for more information.

__Example:__ Objects wrapped with expr can then be manipulated by ReQL API functions.

```py
r.expr({'a':'b'}).merge({'b':[1,2,3]}).run(conn)
```


