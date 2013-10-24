---
layout: api-command 
language: Ruby
permalink: api/ruby/json/
command: json 
github_doc: https://github.com/rethinkdb/docs/blob/docs/2-query-language/api/ruby/control-structures/json.md
---

# Command syntax #

{% apibody %}
r.json(json_string) &rarr; value
{% endapibody %}

# Description #

Parse a JSON string on the server.

__Example:__ Send an array to the server'

```rb
r.json("[1,2,3]").run(conn)
```
