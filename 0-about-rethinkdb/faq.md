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
<strong>Want to learn more about RethinkDB?</strong>

* Read the <a href="/docs/guide/javascript/">ten-minute guide</a> to get started with RethinkDB.
* Browse the <a href="/docs/architecture/">architecture overview</a> for programmers familiar with distributed systems.
* Jump into the [cookbook](/docs/cookbook/javascript/) and see dozens of examples of common RethinkDB queries.
{% endinfobox %}

<img src="/assets/images/docs/api_illustrations/faq.png" class="api_command_illustration" />

<div id="faqcontents"></div>
---
{% faqsection RethinkDB overview %}

## What is RethinkDB? ##

RethinkDB is an open-source, distributed database built to store JSON
documents and scale to multiple machines with very little effort. It's
easy to set up and learn, and it has a pleasant query language that
supports really useful queries like table joins, groupings, and
aggregations.

{% infobox info %}
    <p><strong>RethinkDB in under two minutes:</strong> see the <a href="/videos/what-is-rethinkdb">highlights video</a>.</p>
{% endinfobox %}

## What are the main differences from other NoSQL databases? ##

Find out how RethinkDB compares to other NoSQL databases:

- [RethinkDB compared to MongoDB](/docs/comparison-tables/) &mdash; an unbiased technical comparison between RethinkDB and MongoDB.
- [RethinkDB vs today's NoSQL](/blog/mongodb-biased-comparison/) &mdash; our biased, but more personal take on what makes RethinkDB different.

## When is RethinkDB a good choice? ##

- RethinkDB is a great choice if you need flexible schemas, value ease of use,
  and are planning to run anywhere from a single node to a sixteen-node
  cluster.
- If you periodically copy your data into a separate system to do analytics
  (such as Hadoop), but your analytics are not incredibly computationally
  intensive, you can significantly simplify things by running your analytical
  queries in RethinkDB directly. RethinkDB will _not_ lock your database.
- Finally, if you are already running a database cluster and are overwhelmed by
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
  consistency, so if you absolutely need high write availability and do not
  mind dealing with conflicts, you may be better off with a Dynamo-style system
  like Riak.

{% endfaqsection %}

{% faqsection Practical considerations %}

## What languages can I use to work with RethinkDB? ##

You can use Ruby, Python, and Javascript/Node.js to write RethinkDB
queries. In addition, there are [community
supported](/docs/install-drivers/) client drivers for more than half a
dozen other languages.

If you already know Javascript, all RethinkDB queries can be freely
intermixed with Javascript code because the server supports native
Javascript execution using the V8 engine.

## What are the system requirements? ##

RethinkDB server is written in C++ and currently runs on 32-bit and
64-bit Linux systems, as well as OS X 10.7 and above. Ruby, Python,
Javascript, as well as [community supported](/docs/install-drivers/)
client drivers can run on any platform where these languages are
supported.

It's best to run RethinkDB on nodes with at least 2GB of RAM, but
there are no other strict hardware requirements. RethinkDB has a
custom caching engine and can run on low-memory nodes with large
amounts of on-disk data, Amazon EC2 instances, etc. It also has
specialized support for high-end hardware and does a great job on
high-memory nodes with many cores, solid-state storage, and
high-throughput network hardware.

## Does RethinkDB support SQL? ##

No, but RethinkDB supports a very powerful, expressive, and easy to
learn query language that can do almost anything SQL can do (and many
things SQL can't do, such as mixing queries with Javascript
expressions and Hadoop-style map/reduce).

## How do queries get routed in a RethinkDB cluster? ##

You can connect your clients to any node in the cluster, and all the
queries will automatically be routed to their destination. Advanced
queries (such as joins, filters, etc.) will be broken up and routed to
the appropriate machines, executed in parallel, the resultset will be
recombined, and streamed back to the client. The user never has to
worry about sending queries to specific nodes&mdash; everything happens
automatically behind the scenes. 

## How does RethinkDB handle write durability? ##

RethinkDB comes with strict write durability out of the box and is
identical to traditional database systems in this respect. By default,
no write is ever acknowledged until it's safely committed to disk.

{% infobox info %}
<strong>Want to speed up your write queries?</strong> Learn how to
[configure durability options](/docs/troubleshooting/#why-are-my-inserts-slow).
{% endinfobox %}

## How is RethinkDB licensed? ##

RethinkDB server is licensed under GNU AGPL v3.0. The client drivers
are licensed under Apache License v2.0.

We wanted to pick a license that balances the interests of three
parties &mdash; our end users, our company, and the software
development community at large. When picking a license, we decided
that these interests can be expressed via three simple goals:

- Allow anyone to download RethinkDB, examine the source code, and use it for
  free (as in speech and beer) for any purpose.
- Require users that choose to modify RethinkDB to fit their needs to release
  the patches to the software development community.
- Require users that are unwilling to release the patches to the software
  development community to purchase a commercial license.
- Given that an enormous amount of software is offered as a service via the
  network and isn't actually distributed in binary form, the most effective
  license to fulfill all three goals is GNU AGPL.

We chose to release the client drivers under Apache License v2.0 to
remove any ambiguity as to the extent of the server license &mdash;
you do not have to license any software that uses RethinkDB under AGPL
and are completely free to use any licensing mechanism of your choice.

{% endfaqsection %}

