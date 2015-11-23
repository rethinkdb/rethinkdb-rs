---
layout: api-command 
language: Java
permalink: api/java/server/
command: server
---

# Command syntax #

{% apibody %}
conn.server()
{% endapibody %}

# Description #

Return the server name and server UUID being used by a connection.

__Example:__ Return the server name and UUID.

```java
conn.server();
```

```json
{ "id": "404bef53-4b2c-433f-9184-bc3f7bda4a15", "name": "amadeus" }
```
