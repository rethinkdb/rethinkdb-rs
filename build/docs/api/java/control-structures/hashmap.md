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

`hashMap` is a convenience provided by the RethinkDB Java driver, and is not actually a ReQL term. It returns a `MapObject`, a RethinkDB-provided class that inherits from `Map<Object,Object>`. You can use `hashMap` outside the context of a ReQL query.


__Example:__ Create a hashmap.

```java
import com.rethinkdb.model.MapObject;

MapObject newData = r.hashMap("user", "fred")
    .with("email", "fred@example.com")
    .with("id", 101)
    .with("admin", true);
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
