---
layout: documentation
title: Changefeeds in RethinkDB
active: docs
docs_active: changefeeds
permalink: docs/changefeeds/ruby/
alias: docs/changefeeds/
switcher: true
language: Ruby
---

<img alt="Data Modeling Illustration" class="api_command_illustration"
    src="/assets/images/docs/api_illustrations/change-feeds.png" />


Changefeeds are a way for clients to subscribe to changes on a table or a
document within that table. Any time a document is inserted, updated, or
deleted, the client driver will be notified about the change. RethinkDB
implements changefeeds via the [changes](/api/ruby/changes) command.

Changefeeds offer a convenient way to perform certain tasks:

- Integrate with other databases or middleware such as ElasticSearch or RabbitMQ.
- Write applications where clients are notified of changes in realtime.

# Basic usage #

You can subscribe to a feed by calling `.changes()` on a table:

```rb
r.table('users').changes.run(conn).each{|change| p(change)}
```

The `changes` command returns a RethinkDB cursor, and like any cursor
you can iterate through its contents in your driver. However, unlike
other cursors, the cursor returned by the `changes` command doesn't
terminate &mdash; when you iterate through all the elements, the
cursor blocks until more elements are available.

Every time you insert, update, or delete a document in a table, an
object describing the change will be added to relevant
changefeeds. For example, if you insert a user `{ 'id': 1, 'name':
'Slava', 'age': 31 }` into the `users` table, RethinkDB will post the
following document into the feeds subscribed to `users`:

```rb
{
  :old_val => nil,
  :new_val => { :id => 1, :name => 'Slava', :age => 31 }
}
```

Here `old_val` is the old version of the document, and `new_val` is a
new version of the document. When a new document is inserted,
RethinkDB sets `old_val` to `nil`, and when a document is deleted
RethinkDB sets `new_val` to `nil`. When a document is updated, both
`old_val` and `new_val` are present.

You can then grab the old version or the new version of the document
(or both), and do anything you like &mdash; log them to a file, send
them to a queueing system or another database, or perform other
queries on RethinkDB.

# Point changefeeds #

A "point" changefeed returns changes to a single document within a table rather than the table as a whole.

```rb
r.table('users').get(100).changes().run(conn)
```

The output format of a point changefeed is identical to a table
changefeed, with the exception that the point changefeed stream will start
with the initial value of the document: a notification with the `new_val`
field, but no `old_val` field.

# Chaining changefeeds #

Like any ReQL command, `changes` integrates with the rest of the query
language. You can call `changes` after most commands that transform or
select data:

* [filter](/api/ruby/filter)
* [map](/api/ruby/map)
* [pluck](/api/ruby/pluck)
* [between](/api/ruby/between) (returns an initial value)
* [min](/api/ruby/min) (returns an initial value)
* [max](/api/ruby/max) (returns an initial value)
* [order_by](/api/ruby/order_by) (returns an initial value)
* [limit](/api/ruby/limit) (returns an initial value)

You can't use changefeeds after [concat_map](/api/ruby/concat_map) or
other transformations whose results cannot be pushed to the shards.
Transformations are applied before changes are calculated.

In addition, `changes` can be chained before any command that operates on
a sequence of documents, as long as that command doesn't require the
entire sequence. (For instance, `count` and `order_by` cannot come after
the `changes` command.)

Suppose you have a chat application with multiple clients posting messages
to different chat rooms. You can create feeds that subscribe to messages
posted to a specific room:

```rb
r.table('messages').changes.filter{ |row|
  row['new_val']['room_id'].eq(ROOM_ID)
}.run(conn)
```

You can also use more complicated expressions. Let's say you have a
table `scores` that contains the latest game score for every user of
your game. You can create a feed of all games where a user beats their
previous score, and get only the new value:

```rb
r.table('scores').changes.filter{ |change|
  change['new_val']['score'] > change['old_val']['score']
}['new_val'].run(conn)
```

# Read More #

Browse the following resources to learn more about ReQL and the
`changes` command:

- The [changes](/api/ruby/changes) command API reference
- [Introduction to ReQL](/docs/introduction-to-reql/)
- [ReQL data types](/docs/data-types/)
