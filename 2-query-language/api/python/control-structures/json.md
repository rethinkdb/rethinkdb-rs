---
layout: api-command 
language: Python
permalink: api/python/json/
command: json
github_doc: https://github.com/rethinkdb/docs/blob/docs/2-query-language/api/python/control-structures/json.md
---

{% apibody %}
r.json(json_string) → value
{% endapibody %}

Parse a JSON string on the server.

__Example:__ Send an array to the server'

```py
r.json("[1,2,3]").run(conn)
```


