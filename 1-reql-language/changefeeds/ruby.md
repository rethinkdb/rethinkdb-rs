---
layout: documentation
title: Changefeeds in RethinkDB
docs_active: changefeeds
permalink: docs/changefeeds/ruby/
alias: docs/changefeeds/
switcher: true
language: Ruby
---

**Changefeeds** lie at the heart of RethinkDB's real-time functionality.  

{% toctag %}

They allow clients to receive changes on a table, a single document, or even the results from a specific query as they happen. Nearly any ReQL query can be turned into a changefeed.

<img alt="Data Modeling Illustration" class="api_command_illustration"
    src="/assets/images/docs/api_illustrations/change-feeds.png" />

# Basic usage #

Subscribe to a feed by calling [changes][] on a table:

[changes]: /api/ruby/changes

```rb
r.table('users').changes.run(conn).each{|change| p(change)}
```

The `changes` command returns a cursor (like the `table` or `filter` commands do). You can iterate through its contents using ReQL. Unlike other cursors, the output of `changes` is infinite: the cursor will block until more elements are available. Every time you make a change to the table or document the `changes` feed is monitoring, a new object will be returned to the cursor. For example, if you insert a user `{id: 1, name: Slava, age: 31}` into the `users` table, RethinkDB will post this document to changefeeds subscribed to `users`:

```rb
{
  :old_val => nil,
  :new_val => { :id => 1, :name => 'Slava', :age => 31 }
}
```

Here `old_val` is the old version of the document, and `new_val` is a new version of the document. On an `insert`, `old_val` will be `null`; on a `delete`, `new_val` will be `null`. On an `update`, both  `old_val` and `new_val` are present.

# Point (single document) changefeeds #

A "point" changefeed returns changes to a single document within a table rather than the table as a whole.

```rb
r.table('users').get(100).changes().run(conn)
```

The output format of a point changefeed is identical to a table changefeed.

# Changefeeds with filtering and aggregation queries #

Like any ReQL command, `changes` integrates with the rest of the query language. You can call `changes` after most commands that transform or select data:

* [filter](/api/ruby/filter)
* [get_all](/api/ruby/get_all)
* [map](/api/ruby/map)
* [pluck](/api/ruby/pluck)
* [between](/api/ruby/between)
* [union](/api/ruby/union)
* [min](/api/ruby/min)
* [max](/api/ruby/max)
* [order_by](/api/ruby/order_by).[limit](/api/ruby/limit)

You can also chain `changes` before any command that operates on a sequence of documents, as long as that command doesn't consume the entire sequence. (For instance, `count` and `orderBy` cannot come after the `changes` command.)

Suppose you have a chat application with multiple clients posting messages to different chat rooms. You can create feeds that subscribe to messages posted to a specific room:

```rb
r.table('messages').filter{ |row|
  row['room_id'].eq(ROOM_ID)
}.changes().run(conn)
```

You can also use more complicated expressions. Let's say you have a table `scores` that contains the latest game score for every user of your game. You can create a feed of all games where a user beats their previous score, and get only the new value:

```rb
r.table('scores').changes().filter{ |change|
  change['new_val']['score'] > change['old_val']['score']
}['new_val'].run(conn)
```

Limitations and caveats on chaining with changefeeds:

* `min`, `max` and `order_by` must be used with indexes.
* `order_by` requires `limit`; neither command works by itself.
* `order_by` must be used with a [secondary index](/docs/secondary-indexes/ruby) or the primary index; it cannot be used with an unindexed field.
* You cannot use changefeeds after [concat_map](/api/ruby/concat_map) or other transformations whose results cannot be pushed to the shards.
* You cannot apply a `filter` after `order_by.limit` in a changefeed.
* Transformations are applied before changes are calculated.

# Including state changes #

The `include_states` optional argument to `changes` allows you to receive extra "status" documents in changefeed streams. These can allow your application to distinguish between initial values returned at the start of a stream and subsequent changes. Read the [changes][] API documentation for a full explanation and example.

# Including initial values #

By specifying `true` to the `include_initial` optional argument, the changefeed stream will start with the current contents of the table or selection being monitored. The initial results will have `new_val` fields, but no `old_val` fields, so it's easy to distinguish them from change events.

If an initial result for a document has been sent and a change is made to that document that would move it to the unsent part of the result set (for instance, a changefeed monitors the top 100 posters, the first 50 have been sent, and poster 48 has become poster 52), an "uninitial" notification will be sent, with an `old_val` field but no `new_val` field. This is distinct from a delete change event, which would have a `new_val` of `nil`. (In the top 100 posters example, that could indicate the poster has been deleted, or has dropped out of the top 100.)

If you specify `true` for both `include_states` and `include_initial`, the changefeed stream will start with a `{:state => 'initializing'}` status document, followed by initial values. A `{:state => 'ready'}` status document will be sent when all the initial values have been sent.

# Handling latency #

Depending on how fast your application makes changes to monitored data and how fast it processes change notifications, it's possible that more than one change will happen between calls to the `changes` command. You can control what happens in that case with the `squash` optional argument.

By default, if more than one change occurs between invocations of `changes`, your application will receive a single change object whose `new_val` will incorporate *all* the changes to the data. Suppose three updates occurred to a monitored document between `change` reads:

| Change | Data |
| ----- | ------ |
| Initial state (`old_val`) |  { name: "Fred", admin: true } |
| update({name: "George"}) | { name: "George", admin: true } |
| update({admin: false}) | { name: "George", admin: false } |
| update({name: "Jay"}) | { name: "Jay", admin: false } |
| `new_val` | { name: "Jay", admin: false } |

Your application would by default receive the object as it existed in the database after the *most recent* change. The previous two updates would be "squashed" into the third.

If you wanted to receive *all* the changes, including the interim states, you could do so by passing `squash: false`. The server will buffer up to 100,000 changes. (This number can be changed with the `changefeed_queue_size` optional argument.)

A third option is to specify how many seconds to wait between squashes. Passing `squash: 5` to the `changes` command tells RethinkDB to squash changes together every five seconds. Depending on your application's use case, this might reduce the load on the server. A number passed to `squash` may be a float. Note that the requested interval is not guaranteed, but is rather a best effort.

# Scaling considerations #

Changefeeds perform well as they scale, although they create extra intracluster messages in proportion to the number of servers with open feed connections on each write. This can be mitigated by running a RethinkDB proxy server (the `rethinkdb proxy` startup option); read [Running a proxy node](/docs/sharding-and-replication/#running-a-proxy-node) for details.

Since changefeeds are unidirectional with no acknowledgement returned from clients, they cannot guarantee delivery. If you need real-time updating with delivery guarantees, consider using a model that distributes to the clients through a message broker such as [RabbitMQ][ps].

[ps]: /docs/rabbitmq/ruby/

# Read more #

- The [changes](/api/ruby/changes) command API reference
- [Introduction to ReQL](/docs/introduction-to-reql/)
- [ReQL data types](/docs/data-types/)
