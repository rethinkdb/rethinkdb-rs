---
layout: documentation
title: Frequently asked questions
active: faq
docs_active: faq
permalink: faq/
alias: docs/faq/
js: faq_index
---
{% infobox info %}

**Want to learn more about RethinkDB?**

* Read the [ten-minute guide][tmg] to get started with RethinkDB.
* Browse the [architecture overview][ao] for programmers familiar with distributed systems. 
* Jump into the [cookbook][cb] and see dozens of examples of common RethinkDB queries.

[tmg]: /docs/guide/javascript/
[ao]:  /docs/architecture/
[cb]:  /docs/cookbook/javascript/

{% endinfobox %}

<img src="/assets/images/docs/api_illustrations/faq.png" class="api_command_illustration" />

<div id="faqcontents"></div>

---

{% faqsection RethinkDB overview %}

## What is RethinkDB? ##

RethinkDB is an open-source, distributed database built to store JSON documents and effortlessly scale to multiple machines. It's easy to set up and learn and features a simple but powerful query language that supports table joins, groupings, aggregations, and functions.

{% infobox info %}

**RethinkDB in under two minutes:** see the [highlights video][hv].

[hv]: /videos/what-is-rethinkdb

{% endinfobox %}

## What are the main differences from other NoSQL databases? ##

We've prepared a [technical comparison of RethinkDB and MongoDB][t1] for an unbiased point-by-point overview comparing us to MongoDB.

For a more conversational take, read "[RethinkDB compared to MongoDB][t2]" as well as [@coffeemug][t3]'s biased but more personal take on what makes RethinkDB different, "[RethinkDB vs today's NoSQL][t4]."

[t1]: /docs/comparison-tables/
[t2]: /docs/rethinkdb-vs-mongodb
[t3]: https://github.com/coffeemug
[t4]: /blog/mongodb-biased-comparison/

## When is RethinkDB a good choice? ##

- RethinkDB is a great choice if you need flexible schemas, value ease of use,
  and are planning to run anywhere from a single node to a sixteen-node
  cluster.
- If you periodically copy your data into a separate system to do analytics
  (such as Hadoop) but your analytics are not incredibly computationally
  intensive, you can significantly simplify things by running your analytical
  queries in RethinkDB directly. RethinkDB will _not_ lock your database.
- Finally, if you are already running a database cluster and feel overwhelmed by
  cluster administration and the complexities of sharding, replication, and
  failover, you will love RethinkDB. Sharding and replication can be done in a
  few clicks in the Web UI or on the command line.


## When is RethinkDB not a good choice? ##

- RethinkDB is not a good choice if you need full ACID support or strong schema
  enforcement &mdash; in this case you are better off using a relational
  database such as MySQL or PostgreSQL.
- If you are doing deep, computationally-intensive analytics you are better off
  using a system like Hadoop or a column-oriented store like Vertica.
- In some cases RethinkDB trades off write availability in favor of data
  consistency. If high write availability is critical and you don't
  mind dealing with conflicts you may be better off with a Dynamo-style system
  like Riak.

{% endfaqsection %}

{% faqsection Practical considerations %}

## What languages can I use to work with RethinkDB? ##

We provide official drivers for Ruby, Python, and JavaScript/Node.js. [Community-supported drivers][csd] exist for more than a dozen other languages, including C#/.NET, Go, and PHP.

[csd]: /docs/install-drivers/

## What are the system requirements? ##

The RethinkDB server is written in C++ and runs on 32-bit and 64-bit Linux systems, as well as OS X 10.7 and above. Client drivers can run on any platform where their languages are supported.

We recommend RethinkDB servers have at least 2GB of RAM, but there are no other strict hardware requirements. RethinkDB has a custom caching engine and can run on low-memory nodes with large amounts of on-disk data, Amazon EC2 instances, etc. It also has specialized support for high-end hardware and does a great job on high-memory nodes with many cores, solid-state storage, and high-throughput network hardware.

## Does RethinkDB support SQL? ##

No. However, RethinkDB's query language can do nearly anything SQL can do, including table joins and aggregation functions, and it's powerful, expressive and easy to learn. ReQL can also do many things SQL *can't* do, including mixing queries with JavaScript expressions and [map-reduce][mr].

[mr]: http://en.wikipedia.org/wiki/MapReduce

## Are RethinkDB transactions atomic? ##

For single document transactions, yes&mdash;in those cases, changes to documents are recorded atomically (along with changes to relevant indexes). For multiple document transactions, RethinkDB favors data consistency over high write availability. While RethinkDB is always CID, it is not always A.

## How do queries get routed in a RethinkDB cluster? ##

Users never have to worry about sending queries to specific nodes. Connect your clients to any node in a cluster, and queries will be routed to the proper destination. Advanced queries such as joins and filters will be executed in parallel, with results recombined and streamed back to the client transparently. Everything happens automatically behind the scenes.

## How does RethinkDB handle write durability? ##

RethinkDB comes with strict write durability out of the box and is identical to traditional database systems in this respect. By default, no write is ever acknowledged until it's safely committed to disk.

{% infobox info %}

**Want to speed up your write queries?** Learn how to
[configure durability options][cdo].

[cdo]: /docs/troubleshooting/#why-are-my-inserts-slow

{% endinfobox %}

## How is RethinkDB licensed? ##

The RethinkDB server is licensed under the [GNU Affero General Public License v3.0][agpl]. The client drivers are licensed under the [Apache License v2.0][apl].

[agpl]: http://www.gnu.org/licenses/agpl-3.0.html
[apl]:  http://www.apache.org/licenses/LICENSE-2.0.html

We wanted to pick a license that balances the interests of three parties &mdash; our end users, our company, and the software development community at large. When picking a license, we decided on three simple goals:

- Allow anyone to download RethinkDB, examine the source code, and use it for free (as in speech and beer) for any purpose.
- Require users who choose to modify RethinkDB to fit their needs to release the patches to the software development community.
- Require users who are unwilling to release the patches to the software development community to purchase a commercial license.

Given that an enormous amount of software is offered as a service via the network rather than being distributed in binary form, the most effective license to fulfill all three goals is the GNU AGPL.

We chose to release the client drivers under the Apache License v2.0 to remove any ambiguity as to the extent of the server license. You do not have to license any software that uses RethinkDB under AGPL, and are free to use any licensing mechanism of your choice.

{% endfaqsection %}
