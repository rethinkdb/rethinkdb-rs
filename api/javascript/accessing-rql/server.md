---
layout: api-command 
language: JavaScript
permalink: api/javascript/server/
command: server
io:
  - - connection
    - null
---

# Command syntax #

{% apibody %}
conn.server(callback)
conn.server() &rarr; promise
{% endapibody %}

# Description #

Return the server name and server UUID being used by a connection.

__Example:__ Return the server name and UUID.

```js
conn.server(callback);

// Result passed to callback
{ "id": "404bef53-4b2c-433f-9184-bc3f7bda4a15", "name": "amadeus" }
```

If no callback is provided, a promise will be returned.
