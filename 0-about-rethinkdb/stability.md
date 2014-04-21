---
layout: documentation
title: "RethinkDB stability information"
short_title: RethinkDB stability
active: docs
docs_active: stability
permalink: stability/
alias: docs/stability/
js: faq_index
---

<div id="faqcontents"></div>
---
{% faqsection Stability under specific scenarios %}

## RethinkDB on a single node ##

RethinkDB is an open-source, distributed database built to store JSON
documents and scale to multiple machines with very little effort. It's
easy to set up and learn, and it has a pleasant query language that
supports really useful queries like table joins, groupings, and
aggregations.

{% infobox info %}
    <p><strong>RethinkDB in under two minutes:</strong> see the <a href="/videos/what-is-rethinkdb">highlights video</a>.</p>
{% endinfobox %}

## RethinkDB in small clusters ##

Find out how RethinkDB compares to other NoSQL databases:

- [RethinkDB compared to MongoDB](/docs/comparison-tables/) &mdash; an unbiased technical comparison between RethinkDB and MongoDB.
- [RethinkDB vs today's NoSQL](/docs/rethinkdb-vs-mongodb/) &mdash; our biased, but more personal take on what makes RethinkDB different.

## RethinkDB in large clusters ##

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

{% endfaqsection %}

