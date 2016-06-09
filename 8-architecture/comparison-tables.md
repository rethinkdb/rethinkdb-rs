---
layout: documentation
title: "Technical comparison: RethinkDB and MongoDB"
docs_active: comparison-tables
permalink: docs/comparison-tables/
alias: docs/comparisons/mongodb/
---

{% infobox %}
__Interested in a more personal perspective?__ Read our take on [what makes RethinkDB different](/docs/rethinkdb-vs-mongodb/).
{% endinfobox %}

RethinkDB is based on a fundamentally different architecture from
MongoDB. Instead of polling for changes, the developer can tell
RethinkDB to continuously push updated query results in realtime.

However, since RethinkDB is sometimes compared to MongoDB we wrote up
an unbiased technical comparison (for a more partisan view, take a
look at the [biased comparison](/blog/mongodb-biased-comparison/)
written by [@coffeemug](https://github.com/coffeemug)). We tried to be
spartan with our commentary to allow the reader to form their own
opinion. Whenever possible, we provide links to the original
documentation for further details.

# Development #

| | RethinkDB | MongoDB |  
| ------ | ------ | ------ |  
| Platforms | Linux, OS X, Windows | Linux, OS X, Windows, Solaris |  
| Data model | JSON documents | BSON documents |  
| Data access | Unified chainable dynamic query language | Dynamic rich query language |  
| JavaScript integration | V8 engine | Spidermonkey/V8 engine |  
| Access languages | JSON protocol<br>3 official libraries<br>Many community supported libraries | BSON protocol<br>13 official libraries<br>Many community supported libraries |  
| Index types | Primary key<br>Compound<br>Secondary<br>Geospatial<br>Arbitrarily computed | Unique (unsharded only)<br>Compound<br>Secondary<br>Geospatial<br>Sparse |  
| Cloud deployment | AWS, dotCloud, Compose.io | Many cloud platforms |  

## Platforms ##

MongoDB has [binary distributions for](http://www.mongodb.org/downloads):

*   Linux 32/64-bit 
*   Windows 32/64-bit 
*   OS X 64-bit
*   Solaris 64-bit

_Note: [MongoDB 32-bit builds are limited to around 2GB of data](http://www.mongodb.org/downloads#32-bit-limit)_.

RethinkDB has [binary packages](/docs/install/) available for:

*   Ubuntu 10.04+ 32/64-bit
*   OS X 10.7+ 64-bit
*   CentOS 6 32/64-bit
*   Debian Wheezy and Jessie 32/64-bit

## Data model ##

MongoDB uses [BSON](http://docs.mongodb.org/manual/core/document/) for storing
data. The BSON protocol, a custom extension of JSON, supports [additional data
types](http://docs.mongodb.org/manual/core/document/#bson-type-considerations)
(e.g. ObjectId, timestamp, datetime, etc.) that are not part of the JSON
specification.

RethinkDB stores JSON documents with a binary on disk serialization. The data
types supported by JSON and implicitly RethinkDB are: number (double precision
floating-point), string, boolean, array, object, null.

## Query language ##

Accessing data in MongoDB can be done using:

* [CRUD operations using BSON objects](http://docs.mongodb.org/manual/core/crud/) for inserting, bulk inserting, filtering, and updating documents
* [Aggregations](http://docs.mongodb.org/manual/aggregation/) including [map-reduce](http://docs.mongodb.org/manual/core/map-reduce/)

RethinkDB provides a [unified chainable query language](/api) supporting:

*   CRUD operations
*	Aggregations including [map-reduce & group-map-reduce](/docs/map-reduce/)
*	Joins
*	Full sub-queries
*   [Changefeeds](/docs/changefeeds/)


## JavaScript integration ##

MongoDB's query language allows JavaScript queries using the [$where
clause](http://docs.mongodb.org/manual/reference/operator/query/where/). MongoDB
[map-reduce functions](http://docs.mongodb.org/manual/core/map-reduce/)
are defined in JavaScript. 

RethinkDB allows embedding [JavaScript
expressions](http://www.rethinkdb.com/api/javascript/js/) anywhere
as part of the [query language](/api). RethinkDB uses a pool of out-of-process
V8 execution engines for isolation.

## Access languages ##

MongoDB has [10 official and many community supported
libraries](http://docs.mongodb.org/ecosystem/drivers/). MongoDB's [wire
protocol is TCP based and uses
BSON](http://docs.mongodb.org/meta-driver/latest/legacy/mongodb-wire-protocol/).

RethinkDB provides official libraries for JavaScript/Node.js, Python, Java, and Ruby.
In addition, there are [community supported client drivers](/docs/install-drivers/) for more than half a dozen other languages.
RethinkDB uses JSON over TCP for client-server communications.

## Indexing ##

MongoDB supports [unique, compound, secondary, sparse, and geospatial
indexes](http://docs.mongodb.org/manual/indexes/).
All MongoDB indexes use a B-tree data structure. Every MongoDB query, including
update operations, uses one and only one index.

RethinkDB supports [primary key, compound, secondary, geospatial, and arbitrarily
computed](/docs/secondary-indexes/)
indexes [stored as
B-trees](/docs/architecture/#how-does-rethinkdb-index-data).
Every RethinkDB query, including update operations, uses one and only one
index.

## Cloud deployment ##

MongoDB can be manually deployed on the majority of cloud platforms (AWS,
Joyent, Rackspace, etc.). MongoDB hosting is also available from a [wide range
of providers](http://docs.mongodb.org/ecosystem/#platforms-and-services)
either as a dedicated service or as an add-on on Platform-as-a-Service
solutions.

RethinkDB can be manually deployed on cloud platforms such as AWS.

# Administration #

|           | RethinkDB | MongoDB |
| :-------- | --------- | ------- |
| CLI tools | ReQL admin commands | JavaScript interactive shell |
| UI tools  | Web-based admin UI | Simple HTTP interface |
| Failover  | Auto primary re-election | Replica-sets with auto primary re-election |
| Backup    | `rethinkdb-dump` | `mongodump` or snapshotting |

## CLI Tools ##

MongoDB provides a [JavaScript interactive
shell](http://docs.mongodb.org/manual/reference/program/mongo/) that
can be used for inspecting data, testing queries, creating indexes, maintenance
scripts, and other administrative functions.

RethinkDB administration tasks, including fine-grained cluster and server
configuration, can be scripted in any language with a ReQL driver. These
commands can also be executed in the admin UI's Data Explorer.

## UI tools ##

MongoDB has a simple [HTTP
interface](http://docs.mongodb.org/ecosystem/tools/http-interfaces/#http-console)
that displays read-only information about a server.  MongoDB (the company)
offers a hosted monitoring solution called [MMS](https://mms.mongodb.com).

RethinkDB has a web-based admin UI accessible on every node of a cluster that
provides high level and guided support for operating the cluster. The admin UI
also includes the Data Explorer for experimenting, tuning, and manipulating
data.


## Failover ##

The 3 main components of a MongoDB cluster (`mongos`, `mongod`, and the 3
config servers) are [highly
available](http://docs.mongodb.org/manual/core/sharded-cluster-high-availability/).
For servers storing data, MongoDB allows setting up replica sets with automatic
primary election.

RethinkDB supports automatic primary re-election using the [Raft algorithm][ra].

[ra]: https://en.wikipedia.org/wiki/Raft_(computer_science)

## Backup ##

MongoDB provides different mechanisms for backing up data:

*   the [`mongodump`](http://docs.mongodb.org/manual/reference/program/mongodump/) utility can perform a live backup of data. 
*   [disk/block level snapshots](http://docs.mongodb.org/manual/tutorial/backup-with-filesystem-snapshots/) can be used to backup a MongoDB instance when journaling is enabled. When [journaling is disabled](http://docs.mongodb.org/manual/tutorial/backup-with-filesystem-snapshots/#create-backups-on-instances-that-do-not-have-journaling-enabled), snapshots are possible after flushing all writes to disk and locking the database.

RethinkDB supports [hot backup](/docs/backup/) on a live cluster via `dump` and `restore` commands.

# Scaling #

|             | RethinkDB | MongoDB |
| :---------- | --------- | ------- |
| Sharding    | Guided range-based sharding<br/>(supervised/guided/advised/trained) | Automatic range-based sharding |
| Replication | Sync and async replication | Replica-sets with log-shipping |
| Multi datacenter | Server grouping via tags with per-group replication and write acknowledgements | Supports different options for multi DC |
| Map-reduce  | Multiple map-reduce functions<br/>Executing ReQL or JavaScript operations | JavaScript-based map-reduce |
| Performance | No published results | No official results |
| Concurrency | Event-based and coroutines<br/>Asynchronous block-level MVCC | Threading<br/>Read-write locks | 

## Sharding ##

MongoDB supports automatic range-based sharding using a shard key. A sharded
MongoDB cluster requires [3 config servers and 1 or more `mongos`
instances](http://docs.mongodb.org/manual/core/sharded-cluster-architectures-production/).

RethinkDB supports 1-click sharding from the admin UI. Sharding can be
configured also from the CLI which also supports manual assignments of shards
to specific servers. Rebalancing the shards can be done through the admin UI.

## Replication ##

[MongoDB replication](http://docs.mongodb.org/manual/replication/) is based on
replica sets which use a master-slave log-shipping asynchronous approach.
MongoDB replica sets are configured using the interactive shell.

RethinkDB allows setting up replication using the 1-click admin web UI or from
the CLI. RethinkDB supports both sync and async replication by specifying the
per-table number of write acknowledgements. RethinkDB replication is based on
B-Tree diff algorithms and doesn't require log-shipping.


## Multi Datacenter Support ##

MongoDB can be configured to run in multiple datacenters via [different
mechanisms](http://docs.mongodb.org/manual/data-center-awareness/):

- assigning priorities to members of replica-sets
- support for nearby replication
- tagging (version 2.0+)

RethinkDB supports grouping servers together in any configuration via "server
tags" with per-group replication and write acknowledgement settings. RethinkDB
immediate consistency-based reads and writes do not require a special protocol.

## Map-reduce ##

MongoDB supports running [JavaScript-based map-reduce
tasks](http://docs.mongodb.org/manual/applications/map-reduce/) through the
`mapReduce` command or from the interactive shell. MongoDB map-reduce allows
pre-filtering and ordering the data for the map phase. It also allows storing
the results in a new collection. The various phases of the MongoDB map-reduce
implementation make uses of different locks.

RethinkDB supports map-reduce with the `map` and `reduce` commands, as
well as group-map-reduce with the `group` command.  Map-reduce queries
can process data using both ReQL and JavaScript. RethinkDB operations
are transparently and fully distributed. None of these operations
require any locks. RethinkDB map-reduce functions can be part of
chained queries, by preceding, following, or being sub-queries of
other operations.

Neither MongoDB nor RethinkDB support incremental map-reduce by default.

## Performance ##

MongoDB doesn't publish any official performance numbers.

Official performance numbers for RethinkDB have not been published yet. (We're still working on improving performance for a number of specialized workloads.)

## Concurrency ##

MongoDB uses [locks at various
levels](http://docs.mongodb.org/manual/faq/concurrency/#which-operations-lock-the-database)
for ensuring data consistency. In MongoDB, v2.2 writes and map-reduce require
write locks at the database level. MongoDB uses threads for handling client
connections.

RethinkDB implements [block-level multiversion concurrency
control](/docs/architecture/#how-does-rethinkdb-execute-queries)
. In case multiple writes are performed on documents that are close together in
the B-Tree, RethinkDB does take exclusive block-level locks, but reads can
still proceed.

# Architecture #

|                   | RethinkDB | MongoDB |
| :---------------- | --------- | ------- |
| Consistency model | Immediate/strong consistency with support for out of date reads | Immediate/strong consistency with support for reading from replicas |
| Atomicity         | Document level | Document level |
| Durability        | Durable | Durable only with journaling enabled |
| Storage engine    | Log-structured B-tree serialization<br/>with incremental, fully concurrent garbage compactor | Memory mapped files |
| Query distribution engine | Transparent routing, distributed and parallelized | Transparent routing requires additional `mongos` processes |
| Caching engine    | Custom per-table configurable B-tree aware caching | OS-level memory mapped files LRU |

## Consistency model ##

MongoDB has a strong consistency model where each document has a master server
at a given point in time. [Until
recently](http://blog.mongodb.org/post/36666163412/introducing-mongoclient)
MongoDB client libraries had by default a fire-and-forget behavior.

In RethinkDB data always remains immediately consistent and conflict-free, and
a read that follows a write is always guaranteed to see the write. This is
accomplished by always assigning every shard to a single authoritative primary replica.
All reads and writes to any key in a given shard always get routed to its
respective primary where they're ordered and evaluated.

Both MongoDB and RethinkDB allow out-of-date reads.

_Note_: While MongoDB and RethinkDB docs refer to their consistency models as
strong and respectively immediate, we think the behavior of the two databases
is equivalent.

## Atomicity ##

MongoDB supports atomic document-level updates that add, remove, or set
attributes to constant values.

RethinkDB supports advanced atomic document-level updates that can add, remove,
or modify attributes with evaluated expressions that can also read current
attribute values. There are 2 cases where atomic updates cannot be guaranteed:
1) updates reading data from other documents; 2) updates involving JavaScript
evaluations.

## Durability ##

MongoDB supports [write-ahead journaling of
operations](http://docs.mongodb.org/manual/tutorial/manage-journaling/) to facilitate fast
crash recovery and durability in the storage engine. MongoDB offers a [recovery
procedure](http://docs.mongodb.org/manual/tutorial/recover-data-following-unexpected-shutdown/)
when journaling is not enabled. 

RethinkDB comes with strict write durability out of the box inspired by [BTRFS
inline journal](https://btrfs.wiki.kernel.org/index.php/Btrfs_design) and is
identical to traditional database systems in this respect. No write is ever
acknowledged until it's safely committed to disk.

## Storage engine ##

MongoDB uses [memory mapped files](http://docs.mongodb.org/manual/faq/storage/)
where the OS is in control of flushing writes and paging data in and out.

In RethinkDB, data is organized into B-Trees and stored on disk using [a
log-structured storage engine](/docs/architecture/#how-does-rethinkdb-index-data)
built specifically for RethinkDB and inspired by the architecture of BTRFS.
RethinkDB's engine includes an incremental, fully concurrent garbage compactor
and offers full data consistency in case of failures.


## Query distribution engine ##

MongoDB clients connect to a cluster through separate `mongos` processes which
are responsible for transparently routing the queries within the cluster.

RethinkDB clients can connect to any node in the cluster and queries will be
automatically routed internally.  Both simple (such as filters, joins) and
composed queries (chained operations) will be broken down, routed to the
appropriate servers, and executed in parallel. The results will be recombined
and streamed back to the client.

## Caching engine ##

MongoDB's storage engine uses memory mapped files which also function as an
[OS-level LRU caching system](http://docs.mongodb.org/manual/faq/storage/).
MongoDB can use all free memory on the server for cache space automatically
without any configuration of a cache size.

RethinkDB implements a custom B-tree aware caching mechanism. The cache size
can be configured on a per-server basis.
