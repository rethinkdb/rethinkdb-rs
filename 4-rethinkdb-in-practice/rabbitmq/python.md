---
layout: documentation
title: Integrating RethinkDB with RabbitMQ
docs_active: rabbitmq
permalink: docs/rabbitmq/python/
switcher: true
language: Python
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
* Install [pika](http://pika.readthedocs.org), a RabbitMQ library for Python

{% endinfobox %}

# Pushing changes to RabbitMQ #

Let's write a script that listens for changes in the RethinkDB server
and pushes them to RabbitMQ.

First we'll need to set up the connection to the RethinkDB server:

```python

import rethinkdb as r
import pika
import json

rethink_conn = r.connect(host='localhost', port=28015)
```

Next, we'll connect to the RabbitMQ server using pika:

```python
rabbit_conn = pika.BlockingConnection(
    pika.ConnectionParameters(host='localhost', port=5672)
)
channel = rabbit_conn.channel()
```

Channels multiplex a single TCP connection. All RabbitMQ operations
are performed on the channel, rather than directly on a
connection. Next, we'll declare the topic exchange so we have
somewhere to send our change notifications:

```python
channel.exchange_declare('rethinkdb', exchange_type='topic', durable=False)
```

This asserts that a topic exchange named "rethinkdb" exists, and that
it's set to be non-durable. If the exchange doesn't exist, it'll be
created. If it does exist and has different properties, an exception
will occur. Being non-durable means it won't persist across RabbitMQ
restarts (this is the default).

For this tutorial, we'll assume the RethinkDB server has a database
named "change_example" and a table named "mytable." Here's the query
that watches for changes:

```python
table_changes = r.db('change_example').table('mytable').changes()
```

The output of the `changes` query adheres to the following protocol:

* If `old_val` is `None`, then `new_val` contains the newly created document.
* If `new_val` is `None`, then `old_val` contains the document that was deleted.
* Otherwise, a document was updated from `new_val` to `old_val`

Now we can plug our changes directly into Rabbit:

```python
for change in table_changes.run(rethink_conn):
    routing_key = 'mytable.' + type_of_change(change)
    channel.basic_publish(exchange, routing_key, json.dumps(change))
```

`table_changes.run()` will block until a change occurs, at which time
we push it into the exchange. The `routing_key` is the topic we'll be
sending it on. For this example, we have three different topics:
`mytable.create`, `mytable.update`, and `mytable.delete`. Each topic
contains only changes of the corresponding type. The function
`type_of_change` does this mapping using the protocol described above.

# Listen to RabbitMQ messages#

The listener is the other side of the interaction: it connects to
RabbitMQ, signs up to be notified of messages it's interested in, and
does something when it receives a message.

As before, we need to create a RabbitMQ connection and channel, and
we'll need to assert that the exchange exists:

```python
import pika
import json

rabbit_conn = pika.BlockingConnection(
    pika.ConnectionParameters(host='localhost', port=5672)
channel = rabbit_conn.channel()
channel.exchange_declare('rethinkdb', exchange_type='topic', durable=False)
```

Unlike the script that pushes data into Rabbit, to listen we need to
create a _queue_. Queues are basically mailboxes. You go to an
exchange and sign up a queue for different topics from that exchange:

```python
queue = channel.queue_declare(exclusive=True).method.queue
```

You can give the queue a name if you want, but since we didn't pass a
name to `queue_declare` it'll create a randomly generated name for us.

Now we need to "bind" the queue to the topics we're interested
in. Other listeners can subscribe to the same topic, and Rabbit will
copy the message for every queue. Here, we'll just keep it simple and
bind to all events from "mytable":

```python
channel.queue_bind(queue, exchange='rethinkdb', routing_key='mytable.*')
```

Finally, to listen to the queue, we use the `channel.consume`
generator. Similar to the changefeed cursor from RethinkDB, `consume`
will block until a message arrives in the queue.

```python
for method, properties, payload in channel.consume(queue):
    change = json.loads(payload)
    tablename, change_type = method.routing_key.split('.')

    print tablename, 'got a change of type:', change_type
    print json.dumps(change, indent=True, sort_keys=True)
```

This will deserialize the change message, and pretty print it, along
with a short description of what kind of change it is.

# Further reading #

* [Full source code for this tutorial](http://github.com/rethinkdb/example-rabbitmq/tree/master/python)
* [In-depth description of the RabbitMQ model](https://www.rabbitmq.com/tutorials/amqp-concepts.html)
* [RethinkDB changefeeds](/docs/changefeeds)
