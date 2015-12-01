---
layout: documentation
title: Installing the Java driver
title_image: /assets/images/docs/driver-languages/javascript.png
docs_active: install-drivers
permalink: docs/install-drivers/kava/
---
{% include docs/install-driver-docs-header.md %}

# Installation #

(To come)

# Usage #

You can use the drivers from Java like this:

```java
import com.rethinkdb.RethinkDB;
import com.rethinkdb.gen.exc.ReqlError;
import com.rethinkdb.gen.exc.ReqlQueryLogicError;
import com.rethinkdb.model.MapObject;
import com.rethinkdb.net.Connection;


public static final RethinkDB r = RethinkDB.r;

conn = r.connection().hostname("localhost").port(28015).connect();

r.db("test").tableCreate("tv_shows").run(conn);
r.table("tv_shows").insert(r.hashMap("name", "Star Trek TNG")).run(conn);
```

__Note:__ RethinkDB connection objects are not thread-safe. It's recommended that applications open a separate connection per thread, or establish a connection pool.

# Next steps #

{% infobox %}
Move on to the [ten-minute guide](/docs/guide/java/) and learn how to use RethinkDB.
{% endinfobox %}
