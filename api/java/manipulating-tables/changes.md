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

You may specify one of two optional arguments via [optArg](/api/java/optarg), `squash` and `include_states`.

The `squash` optional argument controls how `changes` batches change notifications:

* `true`: When multiple changes to the same document occur before a batch of notifications is sent, the changes are "squashed" into one change. The client receives a notification that will bring it fully up to date with the server.
* `false`: All changes will be sent to the client verbatim. This is the default.
* `n`: A numeric value (floating point). Similar to `true`, but the server will wait `n` seconds to respond in order to squash as many changes together as possible, reducing network traffic.

If the `include_states` optional argument is `true`, the changefeed stream will include special status documents consisting of the field `state` and a string indicating a change in the feed's state. These documents can occur at any point in the feed between the notification documents described below. There are currently two states:

* `{"state": "initializing"}` indicates the following documents represent initial values on the feed rather than changes. This will be the first document of a feed that returns initial values.
* `{"state": "ready"}` indicates the following documents represent changes. This will be the first document of a feed that does *not* return initial values; otherwise, it will indicate the initial values have all been sent.

Point changefeeds will always return initial values and have an `initializing` state; feeds that return changes on unfiltered tables will never return initial values. Feeds that return changes on more complex queries may or may not return return initial values, depending on the kind of aggregation. Read the article on [Changefeeds in RethinkDB][cfr] for a more detailed discussion. If `includeStates` is `true` on a changefeed that does not return initial values, the first document on the feed will be `{"state": "ready"}`.

[cfr]: /docs/changefeeds/javascript/

If `include_states` is `false` (the default), the status documents will not be sent on the feed.

If the table becomes unavailable, the changefeed will be disconnected, and a runtime exception will be thrown by the driver.

Changefeed notifications take the form of a two-field object:

```json
{
    "old_val": <document before change>,
    "new_val": <document after change>
}
```

The first notification object in the changefeed stream will contain the query's initial value in `new_val` and have no `old_val` field. When a document is deleted, `new_val` will be `null`; when a document is inserted, `old_val` will be `null`.

{% infobox %}
Certain document transformation commands can be chained before changefeeds. For more information, read the [discussion of changefeeds](/docs/changefeeds/) in the "Query language" documentation.
{% endinfobox %}

The server will buffer up to 100,000 elements. If the buffer limit is hit, early changes will be discarded, and the client will receive an object of the form `{error: "Changefeed cache over array size limit, skipped X elements."}` where `X` is the number of elements skipped.

Commands that operate on streams (such as [filter](/api/java/filter/) or [map](/api/java/map/)) can usually be chained after `changes`.  However, since the stream produced by `changes` has no ending, commands that need to consume the entire stream before returning (such as [reduce](/api/java/reduce/) or [count](/api/java/count/)) cannot.

__Example:__ Subscribe to the changes on a table.

Start monitoring the changefeed in one client:

```java
r.table('games').changes().run(conn, function(err, cursor) {
  cursor.each(console.log);
});
```

As these queries are performed in a second client, the first
client would receive and print the following objects:

```java
> r.table('games').insert({id: 1}).run(conn);
{old_val: null, new_val: {id: 1}}

> r.table('games').get(1).update({player1: 'Bob'}).run(conn);
{old_val: {id: 1}, new_val: {id: 1, player1: 'Bob'}}

> r.table('games').get(1).replace({id: 1, player1: 'Bob', player2: 'Alice'}).run(conn);
{old_val: {id: 1, player1: 'Bob'},
 new_val: {id: 1, player1: 'Bob', player2: 'Alice'}}

> r.table('games').get(1).delete().run(conn)
{old_val: {id: 1, player1: 'Bob', player2: 'Alice'}, new_val: null}

> r.tableDrop('games').run(conn);
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
r.table("test").get(1).filter(row -> row("score").gt(10)).changes().run(conn);
```

__Example:__ Return all the inserts on a table.

```java
r.table("test").changes().filter(
    row -> row("old_val").eq((ReqlExpr) null)
).run(conn);
```

__Example:__ Return all the changes to game 1, with state notifications.

```java
r.table("games").get(1).changes().optArg("include_states", true).run(conn);
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
r.table("games").orderBy().optArg("index", r.desc("score")).limit(10).run(conn);
```
