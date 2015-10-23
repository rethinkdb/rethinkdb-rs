---
layout: api-command
language: Java
permalink: api/java/index_create/
command: indexCreate
related_commands:
    indexWait: index_wait/
    indexStatus: index_status/
    indexList: index_list/
    indexDrop: index_drop/

---

# Command syntax #

{% apibody %}
table.indexCreate(indexName[, indexFunction]) &rarr; object
{% endapibody %}

# Description #

Create a new secondary index on a table. Secondary indexes improve the speed of many read queries at the slight cost of increased storage space and decreased write performance. For more information about secondary indexes, read the article "[Using secondary indexes in RethinkDB](/docs/secondary-indexes/)."

RethinkDB supports different types of secondary indexes:

- *Simple indexes* based on the value of a single field.
- *Compound indexes* based on multiple fields.
- *Multi indexes* based on arrays of values, created when the `multi` [optArg](/api/java/optarg) argument is `true`.
- *Geospatial indexes* based on indexes of geometry objects, created when the `geo` optArg is `true`.
- Indexes based on *arbitrary expressions*.

The `indexFunction` can be an anonymous function or a binary representation obtained from the `function` field of [indexStatus](/api/java/index_status).

If successful, `createIndex` will return an object of the form `{"created": 1}`. If an index by that name already exists on the table, a `ReqlRuntimeError` will be thrown.

__Example:__ Create a simple index based on the field `postId`.

```java
r.table("comments").indexCreate("postId").run(conn);
```

__Example:__ Create a simple index based on the nested field `author > name`.


```java
r.table("comments").indexCreate("author_name", row -> row.g("author").g("name"))
 .run(conn);
```

__Example:__ Create a geospatial index based on the field `location`.

```java
r.table("places").indexCreate("location").optArg("geo", true).run(conn);
```

A geospatial index field should contain only geometry objects. It will work with geometry ReQL terms ([getIntersecting](/api/java/get_intersecting/) and [getNearest](/api/java/get_nearest/)) as well as index-specific terms ([indexStatus](/api/java/index_status), [indexWait](/api/java/index_wait), [indexDrop](/api/java/index_drop) and [indexList](/api/java/index_list)). Using terms that rely on non-geometric ordering such as [getAll](/api/java/get_all/), [orderBy](/api/java/order_by/) and [between](/api/java/between/) will result in an error.

__Example:__ Create a compound index based on the fields `postId` and `date`.

```java
r.table("comments").indexCreate("postAndDate",
    row -> r.array(row.g("postId"), row.g("date"))
).run(conn);
```

__Example:__ Create a multi index based on the field `authors`.

```java
r.table("posts").indexCreate("authors").optArg("multi", true).run(conn);
```

__Example:__ Create a geospatial multi index based on the field `towers`.

```java
r.table("networks").indexCreate("towers")
 .optArg("geo", true).optArg("multi", true).run(conn);
```

__Example:__ Create an index based on an arbitrary expression.

```java
r.table("posts").indexCreate("authors", doc -> r.branch(
    doc.hasFields("updatedAt"),
    doc.g("updatedAt"),
    doc.g("createdAt")
)).run(conn);
```

__Example:__ Create a new secondary index based on an existing one.

```java
byte[] index = r.table("posts").indexStatus("authors").nth(0).g("function")
    .run(conn);
r.table("newPosts").indexCreate("authors", index).run(conn);
```

__Example:__ Rebuild an outdated secondary index on a table.

```java
byte[] oldIndex = r.table("posts")
    .indexStatus("oldIndex").nth(0).g("function").run(conn);

r.table("posts").indexCreate("newIndex", oldIndex).run(conn);
r.table("posts").indexWait("newIndex").run(conn);
r.table("posts").indexRename("newIndex", "oldIndex")
 .optArg("overwrite", true).run(conn);
```
