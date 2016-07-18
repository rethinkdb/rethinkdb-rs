---
layout: api-command
language: JavaScript
permalink: api/javascript/changes/
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
stream.changes([options]) &rarr; stream
singleSelection.changes([options]) &rarr; stream
{% endapibody %}

# Description #

Turn a query into a changefeed, an infinite stream of objects representing changes to the query's results as they occur. A changefeed may return changes to a table or an individual document (a "point" changefeed). Commands such as `filter` or `map` may be used before the `changes` command to transform or filter the output, and many commands that operate on sequences can be chained after `changes`.

There are six optional arguments to `changes`.

* `squash`: Controls how change notifications are batched. Acceptable values are `true`, `false` and a numeric value:
    * `true`: When multiple changes to the same document occur before a batch of notifications is sent, the changes are "squashed" into one change. The client receives a notification that will bring it fully up to date with the server.
    * `false`: All changes will be sent to the client verbatim. This is the default.
    * `n`: A numeric value (floating point). Similar to `true`, but the server will wait `n` seconds to respond in order to squash as many changes together as possible, reducing network traffic. The first batch will always be returned immediately.
* `changefeedQueueSize`: the number of changes the server will buffer between client reads before it starts dropping changes and generates an error (default: 100,000).
* `includeInitial`: if `true`, the changefeed stream will begin with the current contents of the table or selection being monitored. These initial results will have `new_val` fields, but no `old_val` fields. The initial results may be intermixed with actual changes, as long as an initial result for the changed document has already been given. If an initial result for a document has been sent and a change is made to that document that would move it to the unsent part of the result set (e.g., a changefeed monitors the top 100 posters, the first 50 have been sent, and poster 48 has become poster 52), an "uninitial" notification will be sent, with an `old_val` field but no `new_val` field.
* `includeStates`: if `true`, the changefeed stream will include special status documents consisting of the field `state` and a string indicating a change in the feed's state. These documents can occur at any point in the feed between the notification documents described below. If `includeStates` is `false` (the default), the status documents will not be sent.
* `includeOffsets`: if `true`, a changefeed stream on an `orderBy.limit` changefeed will include `old_offset` and `new_offset` fields in status documents that include `old_val` and `new_val`. This allows applications to maintain ordered lists of the stream's result set. If `old_offset` is set and not `null`, the element at `old_offset` is being deleted; if `new_offset` is set and not `null`, then `new_val` is being inserted at `new_offset`. Setting `includeOffsets` to `true` on a changefeed that does not support it will raise an error.
* `includeTypes`: if `true`, every result on a changefeed will include a `type` field with a string that indicates the kind of change the result represents: `add`, `remove`, `change`, `initial`, `uninitial`, `state`. Defaults to `false`.

There are currently two states:

* `{state: 'initializing'}` indicates the following documents represent initial values on the feed rather than changes. This will be the first document of a feed that returns initial values.
* `{state: 'ready'}` indicates the following documents represent changes. This will be the first document of a feed that does *not* return initial values; otherwise, it will indicate the initial values have all been sent.

{% infobox %}
Starting with RethinkDB 2.2, state documents will *only* be sent if the `includeStates` option is `true`, even on point changefeeds. Initial values will only be sent if `includeInitial` is `true`. If `includeStates` is `true` and `includeInitial` is false, the first document on the feed will be `{state: 'ready'}`.
{% endinfobox %}

If the table becomes unavailable, the changefeed will be disconnected, and a runtime exception will be thrown by the driver.

Changefeed notifications take the form of a two-field object:

```js
{
    "old_val": <document before change>,
    "new_val": <document after change>
}
```

When `includeTypes` is `true`, there will be three fields:

```js
{
    "old_val": <document before change>,
    "new_val": <document after change>,
    "type": <result type>
}
```

When a document is deleted, `new_val` will be `null`; when a document is inserted, `old_val` will be `null`.

{% infobox %}
Certain document transformation commands can be chained before changefeeds. For more information, read the [discussion of changefeeds](/docs/changefeeds/javascript/) in the "Query language" documentation.

__Note:__ Changefeeds ignore the `read_mode` flag to `run`, and always behave as if it is set to `single` (i.e., the values they return are in memory on the primary replica, but have not necessarily been written to disk yet). For more details read [Consistency guarantees](/docs/consistency).
{% endinfobox %}

The server will buffer up to `changefeedQueueSize` elements (default 100,000). If the buffer limit is hit, early changes will be discarded, and the client will receive an object of the form `{error: "Changefeed cache over array size limit, skipped X elements."}` where `X` is the number of elements skipped.

Commands that operate on streams (such as [filter](/api/javascript/filter/) or [map](/api/javascript/map/)) can usually be chained after `changes`.  However, since the stream produced by `changes` has no ending, commands that need to consume the entire stream before returning (such as [reduce](/api/javascript/reduce/) or [count](/api/javascript/count/)) cannot.

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
ReqlRuntimeError: Changefeed aborted (table unavailable)
```

__Example:__ Return all the changes that increase a player's score.

```js
r.table('test').changes().filter(
  r.row('new_val')('score').gt(r.row('old_val')('score'))
).run(conn, callback)
```

__Example:__ Return all the changes to a specific player's score that increase it past 10.

```js
r.table('test').get(1).filter(r.row('score').gt(10)).changes().run(conn, callback)
```

__Example:__ Return all the inserts on a table.

```js
r.table('test').changes().filter(r.row('old_val').eq(null)).run(conn, callback)
```

__Example:__ Return all the changes to game 1, with state notifications and initial values.

```js
r.table('games').get(1).changes({includeInitial: true, includeStates: true}).run(conn, callback);
// Result returned on changefeed
{state: 'initializing'}
{new_val: {id: 1, score: 12, arena: 'Hobbiton Field'}}
{state: 'ready'}
{
	old_val: {id: 1, score: 12, arena: 'Hobbiton Field'},
	new_val: {id: 1, score: 14, arena: 'Hobbiton Field'}
}
{
	old_val: {id: 1, score: 14, arena: 'Hobbiton Field'},
	new_val: {id: 1, score: 17, arena: 'Hobbiton Field', winner: 'Frodo'}
}
```

__Example:__ Return all the changes to the top 10 games. This assumes the presence of a `score` secondary index on the `games` table.

```js
r.table('games').orderBy(
    { index: r.desc('score') }
).limit(10).changes().run(conn, callback);
```
