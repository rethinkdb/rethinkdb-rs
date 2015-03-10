---
layout: api-command
language: JavaScript
permalink: api/javascript/changes/
command: changes
related_commands:
    table: table/
io:
    -   - table
        - stream
    -   - singleSelection
        - stream
---

# Command syntax #

{% apibody %}
table.changes({squash: true, includeStates: false}) &rarr; stream
singleSelection.changes({squash: true, includeStates: false}) &rarr; stream
{% endapibody %}

# Description #

Return an infinite stream of objects representing changes to a query.

The `squash` optional argument controls how `changes` batches change notifications:

* `true`: When multiple changes to the same document occur before a batch of notifications is sent, the changes are "squashed" into one change. The client receives a notification that will bring it fully up to date with the server. This is the default.
* `false`: All changes will be sent to the client verbatim.
* `n`: A numeric value (floating point). Similar to `true`, but the server will wait `n` seconds to respond in order to squash as many changes together as possible, reducing network traffic.

If the `includeStates` optional argument is `true`, the changefeed stream will include special status documents consisting of the field `state` and a string indicating a change in the feed's state. These documents can occur at any point in the feed between the notification documents described below. There are currently two states:

* `{state: 'initializing'}` indicates the following documents represent initial values on the feed rather than changes. This will be the first document of a feed that returns initial values.
* `{state: 'ready'}` indicates the following documents represent changes. This will be the first document of a feed that does *not* return initial values; otherwise, it will indicate the initial values have all been sent.

If `includeStates` is `false` (the default), the status documents will not be sent on the feed.

If the table becomes unavailable, the changefeed will be disconnected, and a runtime exception will be thrown by the driver.

Changefeed notifications take the form of a two-field object:

```js
{
    "old_val": <document before change>,
    "new_val": <document after change>
}
```

The first notification object in the changefeed stream will contain the query's initial value in `new_val` and have no `old_val` field. When a document is deleted, `new_val` will be `null`; when a document is inserted, `old_val` will be `null`.

Certain document transformation commands can be chained before changefeeds. For more information, read the [discussion of changefeeds](/docs/changefeeds/javascript/) in the "Query language" documentation.

The server will buffer up to 100,000 elements. If the buffer limit is hit, early changes will be discarded, and the client will receive an object of the form `{error: "Changefeed cache over array size limit, skipped X elements."}` where `X` is the number of elements skipped.

Commands that operate on streams (such as `filter` or `map`) can usually be chained after `changes`.  However, since the stream produced by `changes` has no ending, commands that need to consume the entire stream before returning (such as `reduce` or `count`) cannot.

It's a good idea to open changefeeds on their own connection. If you don't, other queries run on the same connection will experience unpredictable latency spikes while the connection blocks on more changes.

__Example:__ Subscribe to the changes on a table.

Start monitoring the changefeed in one client:

```js
r.table('games').changes().run(conn, function(err, cursor) {
  cursor.each(console.log);
});
```

As these queries are performed in a second client, the first
client would receive and print the following objects:

```js
> r.table('games').insert({id: 1}).run(conn, callback);
{old_val: null, new_val: {id: 1}}

> r.table('games').get(1).update({player1: 'Bob'}).run(conn, callback);
{old_val: {id: 1}, new_val: {id: 1, player1: 'Bob'}}

> r.table('games').get(1).replace({id: 1, player1: 'Bob', player2: 'Alice'}).run(conn, callback);
{old_val: {id: 1, player1: 'Bob'},
 new_val: {id: 1, player1: 'Bob', player2: 'Alice'}}

> r.table('games').get(1).delete().run(conn, callback)
{old_val: {id: 1, player1: 'Bob', player2: 'Alice'}, new_val: null}

> r.tableDrop('games').run(conn, callback);
RqlRuntimeError: Changefeed aborted (table unavailable)
```

__Example:__ Return all the changes that increase a player's score.

```js
r.table('test').changes().filter(
  r.row('new_val')('score').gt(r.row('old_val')('score'))
).run(conn, callback)
```

__Example:__ Return all the changes to Bob's score.

```js
// Note that this will have to look at and discard all the changes to
// rows besides Bob's.  This is currently no way to filter with an index
// on changefeeds.
r.table('test').changes().filter(r.row('new_val')('name').eq('Bob')).run(conn, callback)
```

__Example:__ Return all the inserts on a table.

```js
r.table('test').changes().filter(r.row('old_val').eq(null)).run(conn, callback)
```

__Example:__ Return all the changes to game 1.

```js
r.table('games').get(1).changes().run(conn, callback);
```

__Example:__ Return all the changes to the top 10 games. This assumes the presence of a `score` secondary index on the `games` table.

```js
r.table('games').orderBy({index: r.desc('score')}).limit(10).run(conn, callback);
```
