---
layout: api-command
language: Java
permalink: api/java/changes/
command: changes
related_commands:
    table: table/
io:
    -   - stream
        - stream
    -   - singleSelection
        - stream
---

# Command syntax #

{% apibody %}
stream.changes() &rarr; stream
singleSelection.changes() &rarr; stream
{% endapibody %}

# Description #

Return a changefeed, an infinite stream of objects representing changes to a query. A changefeed may return changes to a table or an individual document (a "point" changefeed), and document transformation commands such as `filter` or `map` may be used before the `changes` command to affect the output.

You may specify one of four optional arguments via [optArg](/api/java/optarg).

* `squash`: Controls how change notifications are batched. Acceptable values are `true`, `false` and a numeric value:
    * `true`: When multiple changes to the same document occur before a batch of notifications is sent, the changes are "squashed" into one change. The client receives a notification that will bring it fully up to date with the server.
    * `false`: All changes will be sent to the client verbatim. This is the default.
    * `n`: A numeric value (floating point). Similar to `true`, but the server will wait `n` seconds to respond in order to squash as many changes together as possible, reducing network traffic. The first batch will always be returned immediately.
* `changefeed_queue_size`: the number of changes the server will buffer between client reads before it starts dropping changes and generates an error (default: 100,000).
* `include_initial`: if `true`, the changefeed stream will begin with the current contents of the table or selection being monitored. These initial results will have `new_val` fields, but no `old_val` fields. The initial results may be intermixed with actual changes, as long as an initial result for the changed document has already been given.
* `include_states`: if `true`, the changefeed stream will include special status documents consisting of the field `state` and a string indicating a change in the feed's state. These documents can occur at any point in the feed between the notification documents described below. If `include_states` is `false` (the default), the status documents will not be sent.

There are currently two states:

* `{state: 'initializing'}` indicates the following documents represent initial values on the feed rather than changes. This will be the first document of a feed that returns initial values.
* `{state: 'ready'}` indicates the following documents represent changes. This will be the first document of a feed that does *not* return initial values; otherwise, it will indicate the initial values have all been sent.

If the table becomes unavailable, the changefeed will be disconnected, and a runtime exception will be thrown by the driver.

Changefeed notifications take the form of a two-field object:

```json
{
    "old_val": <document before change>,
    "new_val": <document after change>
}
```

When a document is deleted, `new_val` will be `null`; when a document is inserted, `old_val` will be `null`.

{% infobox %}
Certain document transformation commands can be chained before changefeeds. For more information, read the [discussion of changefeeds](/docs/changefeeds/) in the "Query language" documentation.
{% endinfobox %}

The server will buffer up to `changefeed_queue_size` elements (default 100,000). If the buffer limit is hit, early changes will be discarded, and the client will receive an object of the form `{error: "Changefeed cache over array size limit, skipped X elements."}` where `X` is the number of elements skipped.

Commands that operate on streams (such as [filter](/api/java/filter/) or [map](/api/java/map/)) can usually be chained after `changes`.  However, since the stream produced by `changes` has no ending, commands that need to consume the entire stream before returning (such as [reduce](/api/java/reduce/) or [count](/api/java/count/)) cannot.

__Example:__ Subscribe to the changes on a table.

Start monitoring the changefeed in one client:

```java
Cursor changeCursor = r.table("games").changes().run(conn);
for (Object change : changeCursor) {
    System.out.println(change);
}
```

As these queries are performed in a second client, the first
client would receive and print the following objects:

```java
r.table("games").insert(r.hashMap("id", 1)).run(conn);
```

```json
{"old_val": null, "new_val": {"id": 1}}
```

```java
r.table("games").get(1).update(r.hashMap("player1", "Bob")).run(conn);
```

```json
{"old_val": {"id": 1}, "new_val": {"id": 1, "player1": "Bob"}}
```

```java
r.table("games").get(1).replace(
    r.hashMap("id", 1).with("player1", "Bob").with("player2", "Alice")
).run(conn);
```

```json
{"old_val": {"id": 1, "player1": "Bob"},
 "new_val": {"id": 1, "player1": "Bob", "player2": "Alice"}}
```

```java
r.table("games").get(1).delete().run(conn);
```

```json
{"old_val": {"id": 1, "player1": "Bob", "player2": "Alice"}, "new_val": null}
```

```java
r.tableDrop("games").run(conn);
```

```
ReqlRuntimeError: Changefeed aborted (table unavailable)
```

__Example:__ Return all the changes that increase a player's score.

```java
r.table("test").changes().filter(
    row -> row.g("new_val").g("score").gt(row.g("old_val").g("score"))
).run(conn);
```

__Example:__ Return all the changes to a specific player's score that increase it past 10.

```java
r.table("test").get(1).filter(row -> row.g("score").gt(10)).changes().run(conn);
```

__Example:__ Return all the inserts on a table.

```java
r.table("test").changes().filter(
    row -> row.g("old_val").eq(null)
).run(conn);
```

__Example:__ Return all the changes to game 1, with state notifications and initial values.

```java
r.table("games").get(1).changes()
 .optArg("include_initial", true).optArg("include_states", true).run(conn);
```

Result returned on changefeed:

```json
{"state": "initializing"}
{"new_val": {"id": 1, "score": 12, "arena": "Hobbiton Field"}}
{"state": "ready"}
{
	"old_val": {"id": 1, "score": 12, "arena": "Hobbiton Field"},
	"new_val": {"id": 1, "score": 14, "arena": "Hobbiton Field"}
}
{
	"old_val": {"id": 1, "score": 14, "arena": "Hobbiton Field"},
	"new_val": {"id": 1, "score": 17, "arena": "Hobbiton Field", "winner": "Frodo"}
}
```

__Example:__ Return all the changes to the top 10 games. This assumes the presence of a `score` secondary index on the `games` table.

```java
r.table("games").orderBy().optArg("index", r.desc("score"))
 .limit(10).changes().run(conn);
```
