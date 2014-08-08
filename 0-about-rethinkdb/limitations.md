---
layout: documentation
title: Limitations in RethinkDB
active: limitations
docs_active: limitations
permalink: limitations/
alias: docs/limitations/
---

RethinkDB has a few hard limitations, as well as some soft limitations that are dependent on your server configurations.

## Cluster/shard limits ##

* There is no hard limit on the number of databases that can be created. 

* There is no hard limit on the number of shards, but the web interface will only accept a number of 32 or less. (You can use the command-line interface to add more shards; see [Sharding and replication](/docs/sharding-and-replication/) for more information.)

## Table/document limits ##

* There is no hard limit on the number of tables per database or cluster. However, performance degrades based on the total number of shards across all tables on a cluster. The current recommendation is to keep this number below 300 (i.e., 300 tables with one shard each, or 50 tables with six shards each).

* Each table requires a minimum of approximately 10MB disk space on each server in a cluster. (A completely empty table takes up 4MB.)

* While there is no hard limit on the size of a single document, there is a recommended limit of 16MB for memory performance reasons.

* The maximum size of a JSON query is 64M.

## Key lengths ##

* Primary keys are limited to 127 characters.

* Secondary keys are indexed on their first 238&minus;*PK* bytes, where *PK* is the primary key length of that table. If a secondary index has keys whose first 238&minus;*PK* bytes are identical, performance using those keys will be degraded, as RethinkDB will fall back on a linear search.

* Secondary indexes do not store objects or `null` values. See [Using secondary indexes](/docs/secondary-indexes/) for more details.

## Other notes ##

Some file systems, typically compressed or encrypted ones, may require the `--no-direct-io` option (see [Create a cluster on system startup](/docs/cluster-on-startup/) for more information on RethinkDB options).

There are currently issues with `btrfs`. Follow [issue #2781](https://github.com/rethinkdb/rethinkdb/issues/2781) for more information.
