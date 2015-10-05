---
layout: api-command
language: Java
permalink: api/java/r/
command: r
---

# Command syntax #

{% apibody %}
r &rarr; r
{% endapibody %}

# Description #

The top-level ReQL namespace.

__Example:__ Set up your top-level namespace.

```java
import com.rethinkdb.RethinkDB;
import com.rethinkdb.gen.exc.ReqlError;
import com.rethinkdb.gen.exc.ReqlQueryLogicError;
import com.rethinkdb.model.MapObject;
import com.rethinkdb.net.Connection;


public static final RethinkDB r = RethinkDB.r;
```
