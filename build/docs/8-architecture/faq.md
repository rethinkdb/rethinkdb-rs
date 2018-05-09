---
layout: documentation
title: Frequently asked questions
active: faq
docs_active: faq
permalink: faq/
alias: docs/faq/
js: faq_index
---
{% infobox %}
**RethinkDB in under two minutes:** see the [highlights video][hv].

[hv]: /videos/what-is-rethinkdb
{% endinfobox %}

## What is RethinkDB? ##

RethinkDB is the first open-source, scalable JSON database built from
the ground up for the realtime web. It inverts the traditional
database architecture by exposing an exciting new access model --
instead of polling for changes, the developer can tell RethinkDB to
continuously push updated query results to applications in
realtime. RethinkDB's realtime push architecture dramatically reduces
the time and effort necessary to build scalable realtime apps.

In addition to being designed from the ground up for realtime apps,
RethinkDB offers a flexible query language, intuitive operations and
monitoring APIs, and is easy to setup and learn.

See the post [Advancing the realtime web][] for more technical details
on RethinkDB's mission.

[Advancing the realtime web]: /blog/realtime-web/

## When is RethinkDB a good choice? ##

RethinkDB is a great choice when your applications could benefit from
realtime feeds to your data.

The query-response database access model works well on the web because
it maps directly to HTTP's request-response. However, modern
applications require sending data directly to the client in
realtime. Use cases where companies benefited from RethinkDB's
realtime push architecture include:

- Collaborative web and mobile apps
- Streaming analytics apps
- Multiplayer games
- Realtime marketplaces
- Connected devices

For example, when a user changes the position of a button in
a collaborative design app, the server has to notify other users that
are simultaneously working on the same project. Web browsers support
these use cases via WebSockets and long-lived HTTP connections, but
adapting database systems to realtime needs still presents a huge
engineering challenge.

RethinkDB is the first open-source, scalable database designed
specifically to push data to applications in realtime. It dramatically
reduces the time and effort necessary to build scalable realtime apps.

<a name="production-use"></a>

## Who is using RethinkDB in production? ##

RethinkDB is being used in production by hundreds of technology
startups, consulting studios, and Fortune 500 companies. Here are some
example use cases:

- [Jive Software][] and [Mediafly][] use RethinkDB to power reactive web and mobile apps
- [Pristine.io][] and [Narrative Clip][] use RethinkDB to power cloud infrastructure for connected devices
- [Platzi][] and [Workshape.io][] use RethinkDB to power realtime analytics
- [CMUNE][] and [NodeCraft][] use RethinkDB to power massively scalable multiplayer games

[Pristine.io]: https://pristine.io/
[Narrative Clip]: http://getnarrative.com/
[Jive Software]: https://www.jivesoftware.com/
[Mediafly]: http://www.mediafly.com/
[Platzi]: https://platzi.com
[Workshape.io]: https://www.workshape.io/
[CMUNE]: http://www.cmune.com/
[NodeCraft]: https://nodecraft.com/

RethinkDB has a vibrant community of over 100,000 developers, and hundreds of contributors from around the world.

## Is RethinkDB based on existing technology? ##

Implementing efficient realtime push architecture required redesigning
most database components, including the query execution engine, the
distributed system, the caching subsystem, and the storage
engine. Because the architecture affects every database component,
RethinkDB has been implemented in C++ from scratch. RethinkDB was built
over five years by a team of database experts with the help of
hundreds of contributors from around the world.

## How is RethinkDB different from realtime sync? ##

RethinkDB is fundamentally different from realtime sync APIs like
[Firebase], [PubNub], or [Pusher] in three important ways.

[Firebase]: https://www.firebase.com/
[PubNub]: https://www.pubnub.com/
[Pusher]: https://pusher.com/

Firstly, realtime sync APIs are cloud services and RethinkDB is an
open-source project. While RethinkDB is available in the cloud via our
partners at [Compose.io][] and [Amazon AWS][], it can also be deployed
in your own infrastructures without restrictions.

[Compose.io]: https://www.compose.io/
[Amazon AWS]: https://aws.amazon.com/marketplace/pp/B013R60Q8Y

Secondly, realtime sync APIs are limited to syncing documents, while
RethinkDB is a general purpose database system. In RethinkDB you can
run arbitrary queries including table joins, subqueries, geospatial
queries, aggregation, and map-reduce. Realtime sync services have much
more limited querying capabilities.

Finally, realtime sync APIs are designed to be accessed directly from
the browser. This makes it very easy to get basic apps up and running,
but limits the flexibility as the app expands. RethinkDB is designed
to be accessed from an application server, much like a traditional
database. This requires slightly more setup code, but allows a lot of
flexibility as the application becomes more sophisticated.

## How is RethinkDB different from MongoDB? ##

RethinkDB is based on a fundamentally different architecture from
MongoDB. Instead of polling for changes, the developer can tell
RethinkDB to continuously push updated query results in realtime. You
can also write applications on top of RethinkDB using traditional
query-response paradigm, and subscribe to realtime feeds later as you
start adding realtime functionality to your app.

For example, here is how you query RethinkDB for a document:

```js
r.table('users').get('coffeemug').run()
```

And here is how you subscribe to a stream of updates from RethinkDB
any time the document changes:

```js
r.table('users').get('coffeemug').changes().run()
```

RethinkDB's realtime architecture can be compared to MongoDB's oplog,
but offers a much higher level of abstraction. RethinkDB's feeds
integrate seamlessly with the query computation engine, and allow you
to subscribe to changes on query results, not just raw replication
data. This architecture dramatically reduces the time and effort
necessary to build scalable realtime apps.

In addition to the realtime push architecture, RethinkDB offers a
number of other advantages over MongoDB:

- An advanced query language that supports table joins, subqueries,
  and massively parallelized distributed computation.
- An elegant and powerful operations and monitoring API that
  integrates with the query language and makes scaling RethinkDB
  dramatically easier.
- A simple and beautiful administration UI that lets you shard and
  replicate in a few clicks, and offers online documentation and query
  language suggestions.

See a [technical comparison of RethinkDB and MongoDB][t1] for an
unbiased point-by-point overview. For a more conversational take, read
[@coffeemug][t2]'s biased but more personal take on [what makes
RethinkDB different][t3].

[t1]: /docs/comparison-tables/
[t2]: https://github.com/coffeemug
[t3]: /blog/mongodb-biased-comparison/

## When is RethinkDB not a good choice? ##

- RethinkDB is not a good choice if you need full ACID support or strong schema
  enforcement&mdash;in this case you are better off using a relational
  database such as MySQL or PostgreSQL.
- If you are doing deep, computationally-intensive analytics you are better off
  using a system like Hadoop or a column-oriented store like Vertica.
- In some cases RethinkDB trades off write availability in favor of data
  consistency. If high write availability is critical and you don't
  mind dealing with conflicts you may be better off with a Dynamo-style system
  like Riak.

{% infobox %}

**Want to learn more about RethinkDB?**

* Read the [ten-minute guide][tmg] to get started with RethinkDB.
* Browse the [architecture overview][ao] for programmers familiar with distributed systems. 
* Jump into the [cookbook][cb] and see dozens of examples of common RethinkDB queries.

[tmg]: /docs/guide/javascript/
[ao]:  /docs/architecture/
[cb]:  /docs/cookbook/javascript/

{% endinfobox %}

# Practical considerations

## What languages can I use to work with RethinkDB? ##

We provide official drivers for Ruby, Python, Java, and JavaScript/Node.js. [Community-supported drivers][csd] exist for more than a dozen other languages, including C#/.NET, Go, and PHP.

[csd]: /docs/install-drivers/

## How scalable are changefeeds? ##

The changefeeds architecture is designed to enable each client to open multiple realtime feeds. Since modern web and mobile applications often have tens of thousands of concurrent clients, RethinkDB's feeds are designed to be extremely scalable. You should be able to open thousands of concurrent active feeds on a single RethinkDB node, and scale to tens or hundreds of thousands of feeds across a RethinkDB cluster.

## What are the system requirements? ##

The RethinkDB server is written in C++ and runs on 32-bit and 64-bit Linux systems, as well as OS X 10.7 and above. Client drivers can run on any platform where their languages are supported.

We recommend RethinkDB servers have at least 2GB of RAM, but there are no other strict hardware requirements. RethinkDB has a custom caching engine and can run on low-memory nodes with large amounts of on-disk data, Amazon EC2 instances, etc. It also has specialized support for high-end hardware and does a great job on high-memory nodes with many cores, solid-state storage, and high-throughput network hardware.

## Does RethinkDB support SQL? ##

No. However, RethinkDB's query language can do nearly anything SQL can do, including table joins and aggregation functions, and it's powerful, expressive and easy to learn. ReQL can also do many things SQL *can't* do, including mixing queries with JavaScript expressions and [map-reduce][mr].

[mr]: http://en.wikipedia.org/wiki/MapReduce

## Are RethinkDB transactions atomic? ##

Most write operations involving a single document in RethinkDB are guaranteed to be atomic. Operations that are not deterministic cannot update documents in an atomic fashion (such as random values, or values obtained as the result of a subquery). In addition, multiple documents are not updated atomically.

## Can RethinkDB reads ever see stale data? ##

Reads run with the `read_mode` option set to `single` (the default) will normally never see stale data, but they may see changes from concurrent writes that have not been safely committed to disk yet. This is equivalent to SQL's `READ UNCOMMITTED` isolation level. Reads run with `read_mode` set to `outdated` may see stale data.

If your cluster experiences a netsplit, then the `single` read mode can no longer make this guarantee: you might receive a response from the old primary, even though a new primary has been elected on the other side of the netsplit. Setting `read_mode` to `majority` guarantees no stale reads in this case as well, although reads will be slower. Read the [Consistency guarantees][cg] documentation for more information.

[cg]: /docs/consistency/

## How do queries get routed in a RethinkDB cluster? ##

Users never have to worry about sending queries to specific nodes. Connect your clients to any node in a cluster, and queries will be routed to the proper destination. Advanced queries such as joins and filters will be executed in parallel, with results recombined and streamed back to the client transparently. Everything happens automatically behind the scenes.

## How does RethinkDB handle write durability? ##

RethinkDB comes with strict write durability out of the box and is identical to traditional database systems in this respect. By default, no write is ever acknowledged until it's safely committed to disk.

{% infobox %}

**Want to speed up your write queries?** Learn how to
[configure durability options][cdo].

[cdo]: /docs/troubleshooting/#why-are-my-inserts-slow

{% endinfobox %}

## What usage statistics does RethinkDB collect? ##

By default, RethinkDB will collect anonymous usage statistics and report them to RethinkDB HQ when it checks for new versions of the server. The data it transmits are:

* RethinkDB's version
* The number of servers in the cluster
* The operating system (Linux or OS X) and architecture (32 or 64 bit)
* The number of tables, rounded to the nearest order of magnitude: 2<sup>round(log<sub>2</sub>(<em>tables</em>)</sup>

If the RethinkDB server is started with the `no-update-check` option, these statistics will *not* be sent.

## How is RethinkDB licensed? ##

The RethinkDB server and client libraries are licensed under the
[Apache License v2.0][apl].

[apl]:  http://www.apache.org/licenses/LICENSE-2.0.html
