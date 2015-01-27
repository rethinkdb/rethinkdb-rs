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
table.changes(squash=True) &rarr; stream
singleSelection.changes(squash=True) &rarr; stream
{% endapibody %}

# Description #

Return an infinite stream of objects representing changes to a table or a document.

The `squash` optional argument controls how `changes` batches change notifications:

* `True`: When multiple changes to the same table or document occur before a batch of notifications is sent, the changes are "squashed" into one change. The client receives a notification that will bring it fully up to date with the server. This is the default.
* `False`: All changes will be sent to the client verbatim.
* `n`: A numeric value (floating point). Similar to `True`, but the server will wait `n` seconds to respond in order to squash as many changes together as possible, reducing network traffic.

## Notification format ##

Changefeed notifications take the form of a two-field object:

```py
{
    "old_val": <document before change>,
    "new_val": <document after change>
}
```

When a document is deleted, `new_val` will be `None`; when a document is inserted, `old_val` will be `None`.

Certain document transformation commands can be chained before changefeeds. For more information, read the [discussion of changefeeds](docs/changefeeds/python/) in the "Query language" documentation.

## Changefeeds on tables ##

The server will buffer up to 100,000 elements. If the buffer limit is hit, early changes will be discarded, and the client will receive an object of the form `{"error": "Changefeed cache over array size limit, skipped X elements."}` where `X` is the number of elements skipped.

If the table becomes unavailable, the changefeed will be disconnected, and a runtime exception will be thrown by the driver.

Commands that operate on streams (such as `filter` or `map`) can usually be chained after `changes`.  However, since the stream produced by `changes` has no ending, commands that need to consume the entire stream before returning (such as `reduce` or `count`) cannot.

## On single documents ##

Whenever the document changes, the new document will be appended to the stream. The stream will always start with a notification that *only* has the `new_val` field (no `old_val` will be listed).

It's a good idea to open changefeeds on their own connection. If you don't, other queries run on the same connection will experience unpredictable latency spikes while the connection blocks on more changes.

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
RqlRuntimeError: Changefeed aborted (table unavailable)
```

__Example:__ Return all the changes that increase a player's score.

```py
r.table('test').changes().filter(
  r.row['new_val']['score'] > r.row['old_val']['score']
).run(conn)
```

__Example:__ Return all the changes to Bob's score.

```py
# Note that this will have to look at and discard all the changes to
# rows besides Bob's.  This is currently no way to filter with an index
# on changefeeds.
r.table('test').changes().filter(r.row['new_val']['name'].eq('Bob')).run(conn)
```

__Example:__ Return all the inserts on a table.

```py
r.table('test').changes().filter(r.row['old_val'].eq(None)).run(conn)
```

__Example:__ Return all the changes to game 1.

```py
r.table('games').get(1).changes().run(conn)
```

__Example:__ Return all the changes to the top 10 games. This assumes the presence of a `score` secondary index on the `games` table.

```py
r.table('games').order_by(index=r.desc('score')).limit(10).run(conn)
```
