---
layout: documentation
title: Publish-Subscribe with RethinkDB
active: docs
docs_active: publish-subscribe
permalink: docs/publish-subscribe/ruby/
switcher: true
language: Ruby
---

The
[publish-subscribe pattern](http://en.wikipedia.org/wiki/Publish-subscribe)
is a powerful way to decouple applications that need to
communicate. RethinkDB [changefeeds](/docs/changefeeds) allow us to
implement publish-subscribe with the database acting as a message
exchange. We've built a small example library called
[repubsub](https://github.com/rethinkdb/example-pubsub/tree/master/ruby)
implementing the pattern for use in Ruby applications.

This article will explain how to use repubsub, as well as describe how
it's implemented on top of changefeeds. If your application needs
asynchronous broadcast notifications, this may be a good fit.

# Publish-Subscribe #

There are different publish-subscribe variations, so here we'll
describe the type using a central topic exchange. In this model,
publishers connect to the central exchange and broadcast a message
with a given topic. When subscribers connect, they notify the exchange
about what kinds of messages they're interested in. The exchange is
then responsible for filtering messages.

Because of this decoupled interaction, publishers are free to
disconnect whenever they want. There may even be more than one
publisher. Likewise, if no subscribers are currently listening for
messages with a certain topic, the exchange is free to simply delete
them.

# Using repubsub #

Repubsub implements a simple abstraction on top of RethinkDB to enable
publish-subscribe. It uses ReQL as the filtering mechanism, so the
full power of the language is at your disposal. This gives a lot more
flexibility than traditional message queues.

The repubsub library has three classes:

* An `Exchange` is created by both publishers and
  subscribers. Publishers put messages into the exchange, and
  subscribers listen to messages on the exchange.
* A `Topic` is used by publishers. It contains some key that contains
  meta-data about the messages.
* A `Queue` is used by consumers. It has two purposes:
   1. To buffer messages that the subscriber hasn't consumed yet (this
      buffering is actually done in the database server)
   2. To filter messages from the `Exchange` by their `Topic` (again,
      the server does this filtering)

To import repubsub and create a connection to an exchange:

```ruby
require 'repubsub'

exchange = Repubsub::Exchange.new(:pubsub_demo,
    :db => 'repubsub', :host => 'localhost', :port => 28015)
```

## Subscribing to topics using regex ##

The simplest case is publishing a message with a string for a
topic. This lends itself to using regexes for filtering.

To publish a message to the exchange, create a topic:

```ruby
topic = exchange.topic('fights.superheroes.batman')
```

Now we can publish any arbitrary JSON document to the topic:

```ruby
message = {:opponent => 'Joker', :victory => True}
topic.publish(payload)
```

In the subscribing application we need to create a queue to receive
and buffer messages. The queue takes a ReQL filtering function as an
argument. This is similar to what you would pass to
[filter](/api/ruby/filter). Here we'll subscribe to all messages
about superhero fights:

```ruby
queue = exchange.queue{|topic| topic.match('fights\.superheroes.*')}
```

Then, to listen to messages, just iterate over the `.subscription`
method on the queue:

```ruby
queue.subscription.each do |topic,payload|
  puts "I got the topic: #{topic}"
  puts "With the message: #{payload}"
end
```

## Subscribing to topics using tags ##

You can also filter messages by tags. We could put the tags into a
string and build a regex to match messages with the tags we want, but
luckily we have the full power of ReQL at our disposal. Instead, we
can make the topic an actual JSON array, and use ReQL's
[contains](/api/ruby/contains) method to do the filtering.

So, for example, if we wanted to send a notification that Batman and
the Joker had a fight, we might publish with the tags `#superhero`,
`#fight`, and `#supervillain`:

```ruby
topic = exchange.topic(['superhero', 'fight', 'supervillain'])
message = {:interaction_type => 'tussle', :participants => ['Batman', 'Joker']}
topic.publish(message)
```

Then, subscribers could listen for messages with any combination of tags:

```ruby
filter_func = lambda {|tags| tags.contains('fight', 'superhero')}

exchange.queue(filter_func).subscription.each do |tags,payload|
  fighter1, fighter2 = payload['participants']
  puts "#{fighter1} got in a fight with #{fighter2}"
end
```

In this case, we would only receive notifications of fights involving
a superhero. Fights between supervillains would be ignored.

## Subscribing to hierarchical topics ##

As a final example, we'll use an object as the topic. Using an object
as the topic allows us a richer hierarchical structure, rather than
keeping them in a flat structure like an array. This provides us with
maximum flexibility in message routing.

Let's say we want to publish the teaming up between Batman, Superman
and the Joker:

```ruby
topic_hash = {
  :teamup => {
    :superheroes => ['Batman', 'Superman'],
    :supervillains => ['Joker'],
  },
  :suprising => true
}

topic = exchange.topic(topic_hash)
topic.publish('Today Batman, Superman and the Joker teamed up '
              'in a suprising turn of events...')
```

There are multiple subscriptions we could have set up that would receive this news:

```ruby
# Get all surprising messages
surprising_filter = lambda {|topic| topic['surprising']}

# Get all messages involving a teamup or a fight
teamup_filter = lambda {|topic| topic['teamup'] | topic['fight']}

# Get all messages talking about a teamup with Batman
batman_query = lambda {|topic| topic['teamup']['superheroes'].contains('Batman')}
```


## Try out the repubsub demo ##

The example documentation includes a
[demo script](https://github.com/rethinkdb/example-pubsub/blob/master/ruby/demo.rb')
that shows off the three topic patterns described above. The script
implements both a publisher and a subscriber with each pattern
type. You can use this script to try out multiple publishers and
multiple subscribers to test out how it works.

Run the publisher and corresponding subscribers in different terminal
windows, so the output doesn't run together. For example, to run the
publisher for the regex demo:

```bash
$ ./demo.rb regex publish
```

and in another window run:

```bash
$ ./demo.rb regex subscribe
```

You can run the `tags` and `hierarchy` demos the same way.

# How the library is implemented #

As mentioned above, the repubsub library is built using RethinkDB
changefeeds. Briefly, here's how it works:

* Each exchange is a single RethinkDB table
* Each document in the table has 4 keys: `id`, `topic`, `payload`, and
  `updated_on`.
    * For every message sent, repubsub sets the `updated_on` key
      with `r.now` to get the current time
* When posting a message to a topic, first repubsub attempts to
  overwrite a document with the exact same topic. If the exact topic
  isn't found, it creates a new document with the topic.
* Subscribers create a changefeed on the `Exchange`'s table, filtering
  for changes that mention documents matching their topic queries.

A key point to notice is that we don't actually care about the
document being stored in the table. We only create and update
documents because that forces RethinkDB to create a change
notification. These change notifications are the messages we want to
send to subscribers. Ultimately, the table ends up with lots of
documents that have whatever the last message happened to be inside
them. But at no point do we read those documents directly as a
subscriber. This is also why we update the `updated_on` field, so that
even if the document's payload hasn't changed, the document as a whole
will change and a notification will be generated.

The entire  query on the exchange is:

```ruby
# @table is the Exchange's underlying table
# filter_func is the function passed in by the subscriber
@table.changes[:new_val].filter{|row| filter_func.call(row[:topic])}
```

This query pulls out `new_val` from the changefeed, and passes just
the topic field from the new value down to the subscriber's function.

```ruby
full_query(filter_func).run(@conn).lazy.map do |message|
  [message['topic'], message['payload']]
end
```

We use a lazy map since the changes stream is potentially infinite.
