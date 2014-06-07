---
layout: api-command
language: Ruby
permalink: api/ruby/changes/
command: changes
related_commands:
    table: table/
---

# Command syntax #

{% apibody %}
table.changes() &rarr; stream
{% endapibody %}

# Description #

Takes a table and returns an infinite stream of objects representing
changes to that table.  Whenever an `insert`, `delete`, `update` or
`replace` is performed on the table, an object of the form
`{old_val:..., new_val:...}` will be added to the stream.  For an
`insert`, `old_val` will be `nil`, and for a `delete`, `new_val` will
be `nil`.

If the client is slow to consume changes, the server will buffer them,
up to 100,000 stream elements.  After that, early changes will be
discarded, and the client will instead receive an object of the form
`{error: ...}` describing how many elements were skipped.

If the table becomes unavailable, the changefeed will be disconnected,
and a runtime exception will be thrown by the driver.

Commands that operate on streams (such as `filter` or `map`) can
usually be chained after `changes`.  The exception is commands that
need to consume the entire stream before returning (such as `reduce`
or `count`), which cannot.  (`changes` produces an infinite stream, so
such commands would never terminate.)

__Example:__ Subscribing to the changes on a table.

If you were to write this in one client:

```rb
r.table('games').changes().run(conn)
```

Then performing these queries in a second client would add the objects
in the comments to the stream returned in the first client:

```rb
> r.table('games').insert({id: 1}).run(conn)
# client 1: {old_val: nil, new_val: {id: 1}}
> r.table('games').get(1).update({player1: 'Bob'}).run(conn)
# client 1: {old_val: {id: 1}, new_val: {id: 1, player1: 'Bob'}}
> r.table('games').get(1).replace({id: 1, player1: 'Bob', player2: 'Alice'}).run(conn)
# client 1: {old_val: {id: 1, player1: 'Bob'},
#            new_val: {id: 1, player1: 'Bob', player2: 'Alice'}}
> r.table('games').get(1).delete().run(conn)
# client 1: {old_val: {id: 1, player1: 'Bob', player2: 'Alice'}, new_val: nil}
> r.table_drop('games').run(conn)
# client 1: RUNTIME ERROR
```

__Example:__ Return all the inserts on a table.

```rb
r.table('test').changes().filter{|row| row['old_val'].eq(nil)}.run(conn)
```

__Example:__ `changes` produces an infinite stream, so terminals
cannot be chained on the end.

```rb
r.table('test').changes().count().run(conn) # Produces an error.
```
