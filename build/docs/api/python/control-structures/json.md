---
layout: api-command
language: Python
permalink: api/python/json/
command: json
---

# Command syntax #

{% apibody %}
r.json(json_string) &rarr; value
{% endapibody %}

# Description #

Parse a JSON string on the server.

__Example:__ Send an array to the server.

```py
r.json("[1,2,3]").run(conn)
```


