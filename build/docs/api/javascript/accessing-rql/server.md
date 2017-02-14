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

Return information about the server being used by a connection.

The `server` command returns either two or three fields:

* `id`: the UUID of the server the client is connected to.
* `proxy`: a boolean indicating whether the server is a [RethinkDB proxy node][rp].
* `name`: the server name. If `proxy` is `true`, this field will not be returned.

[rp]: /docs/sharding-and-replication/#running-a-proxy-node

__Example:__ Return server information.

```javascript
conn.server(callback);

// Result passed to callback
{
    "id": "404bef53-4b2c-433f-9184-bc3f7bda4a15",
    "name": "amadeus",
    "proxy": false
}
```

If no callback is provided, a promise will be returned.
