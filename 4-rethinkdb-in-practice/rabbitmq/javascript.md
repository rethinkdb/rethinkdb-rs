---
layout: documentation
title: Integrating RethinkDB with RabbitMQ
active: docs
docs_active: rabbitmq
permalink: docs/rabbitmq/javascript/
switcher: true
language: JavaScript
---

RethinkDB supports [changefeeds](/docs/changefeeds), which allow you
to subscribe to changes on a table. The database pushes these changes
to you as they happen.

This opens up the possibility of notifying client applications
immediately when a change occurs in a table. For real-time
applications, this push behavior is essential.

RabbitMQ is a natural choice for distributing notifications of change
events. It's designed to efficiently route messages to many listeners,
and there are client libraries for most popular languages. In this
tutorial we take advantage of RabbitMQ's
[topic exchanges](https://www.rabbitmq.com/tutorials/amqp-concepts.html#topic-exchange).
Topic exchanges allow clients to subscribe to the messages they're
interested in, and ignore the rest.

{% infobox %}
**Before you start**

* Read the [thirty-second quickstart](/docs/quickstart)
* Ensure you have [RethinkDB installed](/docs/install) for your platform
* Install [amqplib](http://www.squaremobius.net/amqp.node/), a RabbitMQ library for NodeJS

{% endinfobox %}

# Pushing changes to RabbitMQ #

Let's write a script that listens for changes in the RethinkDB server
and pushes them to RabbitMQ.

First we'll need to set up the connection to the RethinkDB server:

```javascript

var r = require('rethinkdb');
var amqp = require('amqplib');

var rethinkConn = null;
var rabbitConn = null;
var channel = null;

var promise = r.connect({host: 'localhost', port: 28015}).then(function(conn){
   rethinkConn = conn;
})
```

Next, we'll connect to the RabbitMQ server using amqplib:

```javascript
promise = promise.then(function(){
    return amqp.connect('amqp://localhost:5672');
}).then(function(conn){
    rabbitConn = conn;
    return rabbitConn.createChannel();
}).then(function(ch){
    channel = ch;
})
```

Channels multiplex a single TCP connection. All RabbitMQ operations
are performed on the channel, rather than directly on a
connection. Next, we'll declare the topic exchange so we have
somewhere to send our change notifications:

```javascript
promise = promise.then(function(){
    return channel.assertExchange('rethinkdb', 'topic', {durable: false});
})
```

This asserts that a topic exchange named "rethinkdb" exists, and that
it's set to be non-durable. If the exchange doesn't exist, it'll be
created. If it does exist and has different properties, an exception
will occur. Being non-durable means it won't persist across RabbitMQ
restarts (this is the default).

For this tutorial, we'll assume the RethinkDB server has a database
named "change_example" and a table named "mytable." Here's the query
that watches for changes:

```javascript
var tableChanges = r.db('change_example').table('mytable').changes();
```

The output of the `changes` query adheres to the following protocol:

* If `old_val` is `null`, then `new_val` contains the newly created document.
* If `new_val` is `null`, then `old_val` contains the document that was deleted.
* Otherwise, a document was updated from `new_val` to `old_val`

Now we can plug our changes directly into Rabbit:

```javascript
promise = promise.then(function(){
    return tableChanges.run(rethinkConn);
}).then(function(changeCursor){
    changeCursor.each(function(err, change){
        var routingKey = 'mytable.' + typeOfChange(change);
        var payload = new Buffer(JSON.stringify(change));
        channel.publish('rethinkdb', routingKey, payload);
    })
})
```

Every time a change occurs, `changeCursor.each` will push the message
into the exchange. The `routingKey` is the topic we'll be sending it
on. For this example, we have three different topics:
`mytable.create`, `mytable.update`, and `mytable.delete`. Each topic
contains only changes of the corresponding type. The function
`typeOfChange` does this mapping using the protocol described above.

# Listening to RabbitMQ messages #

The listener is the other side of the interaction: it connects to
RabbitMQ, signs up to be notified of messages it's interested in, and
does something when it receives a message.

As before, we need to create a RabbitMQ connection and channel, and
we'll need to assert that the exchange exists:

```javascript
amqp = require('amqplib');

var rabbit_conn = null;
var channel = null;
var queue = null;

var promise = amqp.connect('amqp://localhost:5672').then(function(conn){
    rabbitConn = conn;
    return rabbitConn.createChannel();
}).then(function(ch){
    channel = ch;
    return channel.assertExchange('rethinkdb', 'topic', {durable: false});
})
```

Unlike the script that pushes data into Rabbit, to listen we need to
create a _queue_. Queues are basically mailboxes. You go to an
exchange and sign up a queue for different topics from that exchange:

```javascript
promise = promise.then(function(){
    return channel.assertQueue('', {exclusive: true});
}).then(function(q){
    queue = q.queue;
})
```

You can give the queue a name if you want, but since we passed an
empty string to `assertQueue` it'll create a randomly generated name
for us.

Now we need to "bind" the queue to the topics we're interested
in. Other listeners can subscribe to the same topic, and Rabbit will
copy the message for every queue. Here, we'll just keep it simple and
bind to all events from "mytable":

```javascript
promise = promise.then(function(){
    return channel.bindQueue(queue, 'rethinkdb', 'mytable.*');
})
```

Finally, to listen to the queue, we use the `channel.consume`
generator. Similar to the changefeed cursor from RethinkDB, `consume`
will will invoke its callback whenever a message arrives in the queue.

```javascript
promise = promise.then(function(){
    channel.consume(queue, function(msg){
        var change = JSON.parse(msg.content);
        var tablename = msg.fields.routingKey.split('.')[0];
        var changeType = msg.fields.routingKey.split('.')[1];

        console.log(tablename, 'got a change of type:', changeType);
        console.log(JSON.stringify(change, undefined, 2));
    })
})
```

This will deserialize the change message, and pretty print it, along
with a short description of what kind of change it is.

# Further reading #

* [Full source code for this tutorial](http://github.com/rethinkdb/example-rabbitmq/tree/master/javascript)
* [In-depth description of the RabbitMQ model](https://www.rabbitmq.com/tutorials/amqp-concepts.html)
* [RethinkDB changefeeds](/docs/changefeeds)
