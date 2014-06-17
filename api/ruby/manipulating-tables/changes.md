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
`{error: "Changefeed cache over array size limit, skipped X
elements."}` where `X` is the number of elements skipped.

If the table becomes unavailable, the changefeed will be disconnected,
and a runtime exception will be thrown by the driver.

Commands that operate on streams (such as `filter` or `map`) can
usually be chained after `changes`.  The exception is commands that
need to consume the entire stream before returning (such as `reduce`
or `count`), which cannot.  (`changes` produces an infinite stream, so
such commands would never terminate.)

It's usually a good idea to open changefeeds on their own connection.
If you don't, other queries run on the same connection will experience
unpredictable latency spikes while the connection blocks on more
changes.

__Example:__ Subscribe to the changes on a table.

If you were to write this in one client:

```rb
r.table('games').changes().run(conn).each{|change| p(change)}
```

Then performing these queries in a second client would cause the first
client to print the objects in the comments:

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

__Example:__ Return all the changes that increase a player's score.

```rb
r.table('test').changes().filter{|row|
  row['new_val']['score'] > row['old_val']['score']
}.run(conn)
```

__Example:__ Return all the changes to Bob's score.

```rb
# Note that this will have to look at and discard all the changes to
# rows besides Bob's.  This is currently no way to filter with an index
# on change feeds.
r.table('test').changes().filter{|row| row['new_val]['name'].eq('Bob')}.run(conn)
```

__Example:__ Return all the inserts on a table.

```rb
r.table('test').changes().filter{|row| row['old_val'].eq(nil)}.run(conn)
```
