---
layout: api-command
language: Java
permalink: api/java/hashmap/
command: hashMap
py: false
js: false
rb: false
related_commands:
    array: array/
---

# Command syntax #

{% apibody %}
r.hashMap(key, value)[.with(key, value) ...] &rarr; object
{% endapibody %}

# Description #

Take a key/value pair, with extra key/value pairs optionally specified by chaining one or more `with(key, value)` terms after `hashMap`, and return an object.

__Example:__ Create a hashmap.

```java
r.expr(r.hashMap("user", "fred")
    .with("email", "fred@example.com")
    .with("id", 101)
    .with("admin", true)
).run(conn);
```

This creates the object (in JSON):

```json
{
    "admin": true,
    "email": "fred@example.com",
    "id": 101,
    "user": "fred"
}
```

__Example:__ Create a hashmap using MapObject.

The RethinkDB Java driver provides a `MapObject` class that extends `HashMap` by adding a chainable `with(key, value)` method for convenience. To create the object above as a `MapObject`:

```java
import com.rethinkdb.model.MapObject;

MapObject newData = new MapObject()
    .with("user", "fred")
    .with("email", "fred@example.com")
    .with("id", 101)
    .with("admin", true);
```
