---
layout: api-command 
language: Python
permalink: api/python/server/
command: server
---

# Command syntax #

{% apibody %}
conn.server()
{% endapibody %}

# Description #

Return the server name and server UUID being used by a connection.

__Example:__ Return the server name and UUID.

```py
> conn.server()

{ "id": "404bef53-4b2c-433f-9184-bc3f7bda4a15", "name": "amadeus" }
```
