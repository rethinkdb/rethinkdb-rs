---
layout: api-command
language: Java
permalink: api/java/binary/
command: binary
---

# Command syntax #

{% apibody %}
r.binary(data) &rarr; binary
{% endapibody %}

# Description #

Encapsulate binary data within a query.

The type of data `binary` accepts depends on the client language. In Java, it expects a parameter of `byte[]` type (or ReQL queries that return binary data).

Binary objects returned to the client in Java will also be `byte[]` types. This can be changed with the `binary_format` [optArg](/api/java/optarg) provided to [run](/api/java/run) to return "raw" objects.

Only a limited subset of ReQL commands may be chained after `binary`:

* [coerceTo](/api/java/coerce_to/) can coerce `binary` objects to `string` types
* [count](/api/java/count/) will return the number of bytes in the object
* [slice](/api/java/slice/) will treat bytes like array indexes (i.e., `slice(10,20)` will return bytes 10&ndash;19)
* [typeOf](/api/java/type_of) returns `PTYPE<BINARY>`
* [info](/api/java/info) will return information on a binary object.

__Example:__ Save an avatar image to a existing user record.

```java
import java.nio.file.*;

Path path = Paths.get("./defaultAvatar.png");
byte[] avatarImage = Files.readAllBytes(path);
r.table("users").get(100).update(r.hashMap("avatar", avatarImage));
```

__Example:__ Get the size of an existing avatar image.

```java
r.table("users").get(100)("avatar").count().run(conn);

// Result:
14156
```

Read more details about RethinkDB's binary object support: [Storing binary objects](/docs/storing-binary/).
