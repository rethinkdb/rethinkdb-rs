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
---

# Command syntax #

{% apibody %}
table.changes() &rarr; stream
{% endapibody %}

# Description #

Return an infinite stream of objects representing changes to a table. Whenever an `insert`, `delete`, `update` or `replace` is performed on the table, an object of the form `{'old_val': ..., 'new_val': ...}` will be appended to the stream. For an `insert`, `old_val` will be `null`, and for a `delete`, `new_val` will be `null`.

The server will buffer up to 100,000 elements. If the buffer limit is hit, early changes will be discarded, and the client will receive an object of the form `{error: "Changefeed cache over array size limit, skipped X elements."}` where `X` is the number of elements skipped.

If the table becomes unavailable, the changefeed will be disconnected, and a runtime exception will be thrown by the driver.

Commands that operate on streams (such as `filter` or `map`) can usually be chained after `changes`.  However, since the stream produced by `changes` has no ending, commands that need to consume the entire stream before returning (such as `reduce` or `count`) cannot.

It's a good idea to open changefeeds on their own connection. If you don't, other queries run on the same connection will experience unpredictable latency spikes while the connection blocks on more changes.

__Example:__ Subscribe to the changes on a table.

Start monitoring the change feed in one client:

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
// on change feeds.
r.table('test').changes().filter(r.row('new_val')('name').eq('Bob')).run(conn, callback)
```

__Example:__ Return all the inserts on a table.

```js
r.table('test').changes().filter(r.row('old_val').eq(null)).run(conn, callback)
```
