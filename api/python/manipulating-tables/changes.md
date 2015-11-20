---
layout: api-command
language: Python
permalink: api/python/changes/
command: changes
related_commands:
    table: table/
---

# Command syntax #

{% apibody %}
stream.changes([options]) &rarr; stream
singleSelection.changes([options]) &rarr; stream
{% endapibody %}

# Description #

Return a changefeed, an infinite stream of objects representing changes to a query. A changefeed may return changes to a table or an individual document (a "point" changefeed), and document transformation commands such as `filter` or `map` may be used before the `changes` command to affect the output.

There are four optional arguments to `changes`.

* `squash`: Controls how change notifications are batched. Acceptable values are `True`, `False` and a numeric value:
    * `True`: When multiple changes to the same document occur before a batch of notifications is sent, the changes are "squashed" into one change. The client receives a notification that will bring it fully up to date with the server.
    * `False`: All changes will be sent to the client verbatim. This is the default.
    * `n`: A numeric value (floating point). Similar to `True`, but the server will wait `n` seconds to respond in order to squash as many changes together as possible, reducing network traffic. The first batch will always be returned immediately.
* `changefeed_queue_size`: the number of changes the server will buffer between client reads before it starts dropping changes and generates an error (default: 100,000).
* `include_initial`: if `True`, the changefeed stream will begin with the current contents of the table or selection being monitored. These initial results will have `new_val` fields, but no `old_val` fields. The initial results may be intermixed with actual changes, as long as an initial result for the changed document has already been given.
* `include_states`: if `True`, the changefeed stream will include special status documents consisting of the field `state` and a string indicating a change in the feed's state. These documents can occur at any point in the feed between the notification documents described below. If `include_states` is `False` (the default), the status documents will not be sent.

There are currently two states:

* `{"state": "initializing"}` indicates the following documents represent initial values on the feed rather than changes. This will be the first document of a feed that returns initial values.
* `{"state": "ready"}` indicates the following documents represent changes. This will be the first document of a feed that does *not* return initial values; otherwise, it will indicate the initial values have all been sent.

{% infobox %}
Starting with RethinkDB 2.2, state documents will *only* be sent if the `include_states` option is `true`, even on point changefeeds. Initial values will only be sent if `include_initial` is `true`. If `include_states` is `true` and `include_initial` is false, the first document on the feed will be `{'state': 'ready'}`.
{% endinfobox %}

If the table becomes unavailable, the changefeed will be disconnected, and a runtime exception will be thrown by the driver.

Changefeed notifications take the form of a two-field object:

```py
{
    "old_val": <document before change>,
    "new_val": <document after change>
}
```

When a document is deleted, `new_val` will be `None`; when a document is inserted, `old_val` will be `None`.

{% infobox %}
Certain document transformation commands can be chained before changefeeds. For more information, read the [discussion of changefeeds](/docs/changefeeds/python/) in the "Query language" documentation.
{% endinfobox %}

The server will buffer up to 100,000 elements. If the buffer limit is hit, early changes will be discarded, and the client will receive an object of the form `{"error": "Changefeed cache over array size limit, skipped X elements."}` where `X` is the number of elements skipped.

Commands that operate on streams (such as [filter](/api/python/filter/) or [map](/api/python/map/)) can usually be chained after `changes`.  However, since the stream produced by `changes` has no ending, commands that need to consume the entire stream before returning (such as [reduce](/api/python/reduce/) or [count](/api/python/count/)) cannot.

__Example:__ Subscribe to the changes on a table.

Start monitoring the changefeed in one client:

```py
for change in r.table('games').changes().run(conn):
  print change
```

As these queries are performed in a second client, the first client would receive and print the following objects:

```py
> r.table('games').insert({'id': 1}).run(conn)
{'old_val': None, 'new_val': {'id': 1}}

> r.table('games').get(1).update({'player1': 'Bob'}).run(conn)
{'old_val': {'id': 1}, 'new_val': {'id': 1, 'player1': 'Bob'}}

> r.table('games').get(1).replace({'id': 1, 'player1': 'Bob', 'player2': 'Alice'}).run(conn)
{'old_val': {'id': 1, 'player1': 'Bob'},
 'new_val': {'id': 1, 'player1': 'Bob', 'player2': 'Alice'}}

> r.table('games').get(1).delete().run(conn)
{'old_val': {'id': 1, 'player1': 'Bob', 'player2': 'Alice'}, 'new_val': None}

> r.table_drop('games').run(conn)
ReqlRuntimeError: Changefeed aborted (table unavailable)
```

__Example:__ Return all the changes that increase a player's score.

```py
r.table('test').changes().filter(
  r.row['new_val']['score'] > r.row['old_val']['score']
).run(conn)
```

__Example:__ Return all the changes to a specific player's score that increase it past 10.

```py
r.table('test').get(1).filter(r.row['score'].gt(10)).changes().run(conn)
```

__Example:__ Return all the inserts on a table.

```py
r.table('test').changes().filter(r.row['old_val'].eq(None)).run(conn)
```

__Example:__ Return all the changes to game 1, with state notifications and initial values.

```py
r.table('games').get(1).changes(include_initial=True, include_states=True).run(conn)

# result returned on changefeed
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

```py
r.table('games').order_by(index=r.desc('score')).limit(10).changes().run(conn)
```
