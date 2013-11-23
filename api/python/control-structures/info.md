---
layout: api-command
language: Python
permalink: api/python/info/
command: info
---

# Command syntax #

{% apibody %}
any.info() &rarr; object
{% endapibody %}

# Description #

Get information about a ReQL value.

__Example:__ Get information about a table such as primary key, or cache size.

```py
r.table('marvel').info().run(conn)
```


