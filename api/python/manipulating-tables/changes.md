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
table.changes() &rarr; stream
singleSelection.changes() &rarr; stream
{% endapibody %}

# Description #

Return an infinite stream of objects representing changes to a table or a document.

## On tables ##

Whenever an `insert`, `delete`, `update` or `replace` is performed on the table, an object of the form `{'old_val': ..., 'new_val': ...}` will be appended to the stream. For an `insert`, `old_val` will be `null`, and for a `delete`, `new_val` will be `null`.

The server will buffer up to 100,000 elements. If the buffer limit is hit, early changes will be discarded, and the client will receive an object of the form `{'error': 'Changefeed cache over array size limit, skipped X elements.'}` where `X` is the number of elements skipped.

If the table becomes unavailable, the changefeed will be disconnected, and a runtime exception will be thrown by the driver.

Commands that operate on streams (such as `filter` or `map`) can usually be chained after `changes`.  However, since the stream produced by `changes` has no ending, commands that need to consume the entire stream before returning (such as `reduce` or `count`) cannot.

## On single documents ##

Whenever the document changes, the new document will be appended to the stream. The stream will always start with the current version of the document when the `changes` command is executed. At most only one change will be available on each read of the changefeed; if the document changes multiple times between reads, you will receive the most recent version of the document when the changefeed is read.

It's a good idea to open changefeeds on their own connection. If you don't, other queries run on the same connection will experience unpredictable latency spikes while the connection blocks on more changes.

__Example:__ Subscribe to the changes on a table.

Start monitoring the change feed in one client:

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
# on change feeds.
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
