---
layout: api-command
language: Java
permalink: api/java/order_by/
alias:
    - api/java/asc/
    - api/java/desc/
command: orderBy
related_commands:
    skip: skip/
    limit: limit/
    slice: slice/
---

# Command syntax #

{% apibody %}
table.orderBy([key | function]).optArg("index", index) &rarr; selection<stream>
selection.orderBy(key | function[, ...]) &rarr; selection<array>
sequence.orderBy(key | function[, ...]) &rarr; array
{% endapibody %}

# Description #

Sort the sequence by document values of the given key(s). To specify
the ordering, wrap the attribute with either `r.asc` or `r.desc`
(defaults to ascending).

__Note:__ RethinkDB uses byte-wise ordering for `orderBy` and does not support Unicode collations; non-ASCII characters will be sorted by UTF-8 codepoint. For more information on RethinkDB's sorting order, read the section in [ReQL data types](/docs/data-types/#sorting-order).

Sorting without an index requires the server to hold the sequence in
memory, and is limited to 100,000 documents (or the setting of the `arrayLimit` option for [run](/api/java/run)). Sorting with an index can
be done on arbitrarily large tables, or after a [between](/api/java/between/) command
using the same index. This applies to both secondary indexes and the primary key (e.g., `{"index": "id"}`).

__Example:__ Order all the posts using the index `date`.   

```java
r.table("posts").orderBy().optArg("index", "date").run(conn);
```

<!-- stop -->

The index must either be the primary key or have been previously created with [indexCreate](/api/java/index_create/).

```java
r.table("posts").indexCreate("date").run(conn);
```

You can also select a descending ordering:

```java
r.table("posts").orderBy().optArg("index", r.desc("date")).run(conn);
```

__Example:__ Order a sequence without an index.

```java
r.table("posts").get(1).g("comments").orderBy("date").run(conn);
```

You can also select a descending ordering:

```java
r.table("posts").get(1).g("comments").orderBy(r.desc("date")).run(conn);
```

If you're doing ad-hoc analysis and know your table won't have more then 100,000
elements (or you've changed the setting of the `array_limit` option for [run](/api/java/run)) you can run `orderBy` without an index:

```java
r.table("small_table").orderBy("date").run(conn);
```

__Example:__ You can efficiently order using multiple fields by using a
[compound index](http://www.rethinkdb.com/docs/secondary-indexes/java/).

Order by date and title.

```java
r.table("posts").orderBy().optArg("index", "date_and_title").run(conn);
```

The index must either be the primary key or have been previously created with [indexCreate](/api/java/index_create/).

```java
r.table("posts").indexCreate("date_and_title",
    post -> r.array(post.g("date"), post.g("title"))
).run(conn);
```

_Note_: You cannot specify multiple orders in a compound index. See [issue #2306](https://github.com/rethinkdb/rethinkdb/issues/2306)
to track progress.

__Example:__ If you have a sequence with fewer documents than the `arrayLimit`, you can order it
by multiple fields without an index.

```java
r.table("small_table").orderBy("date", r.desc("title")).run(conn);
```

__Example:__ Notice that an index ordering always has highest
precedence. The following query orders posts by date, and if multiple
posts were published on the same date, they will be ordered by title.

```java
r.table("post").orderBy("title").optArg("index", "date").run(conn);
```

__Example:__ Use [nested field](/docs/cookbook/javascript/#filtering-based-on-nested-fields) syntax to sort on fields from subdocuments. (You can also create indexes on nested fields using this syntax with `indexCreate`.)

```java
r.table("user").orderBy(user -> user.g("group").g("id")).run(conn);
```

__Example:__ You can efficiently order data on arbitrary expressions using indexes.

```java
r.table("posts").orderBy().optArg("index", "votes").run(conn);
```

The index must have been previously created with [indexCreate](/api/java/index_create/).

```java
r.table("posts").indexCreate("votes",
    post -> post.g("upvotes").sub(post.g("downvotes"))
).run(conn);
```

__Example:__ If you have a sequence with fewer documents than the `arrayLimit`, you can order it with an arbitrary function directly.

```java
r.table("small_table").orderBy(
    doc -> doc.g("upvotes").sub(doc.g("downvotes"))
).run(conn);
```

You can also select a descending ordering:

```java
r.table("small_table").orderBy(
    r.desc(doc -> doc.g("upvotes").sub(doc.g("downvotes")))
).run(conn);
```

__Example:__ Ordering after a `between` command can be done as long as the same index is being used.

```java
r.table("posts")
 .between(r.time(2013, 1, 1, "+00:00"), r.time(2013, 1, 1, "+00:00"))
 .optArg("index", "date")
 .orderBy().optArg("index", "date")
 .run(conn);
```
