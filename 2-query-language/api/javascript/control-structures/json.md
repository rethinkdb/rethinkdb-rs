---
layout: api-command 
language: JavaScript
permalink: api/javascript/json/
command: json 
github_doc: https://github.com/rethinkdb/docs/blob/docs/2-query-language/api/javascript/control-structures/json.md
io:
    -   - r
        - value
---

# Command syntax #

{% apibody %}
r.json(json_string) &rarr; value
{% endapibody %}

# Description #

Parse a JSON string on the server.

__Example:__ Send an array to the server'

```js
r.json("[1,2,3]").run(conn, callback)
```
