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

* There is a hard limit of 32 shards. (See [Sharding and replication](/docs/sharding-and-replication/) for more information.)

## Table/document limits ##

* There is no hard limit on the number of tables per database or cluster.

* Each table requires a minimum of approximately 10MB disk space on each server in a cluster. (A completely empty table takes up 4MB.)

* Each table has an overhead of 8MB RAM on each server it's replicated on.

* While there is no hard limit on the size of a single document, there is a recommended limit of 16MB for memory performance reasons.

* The maximum size of a JSON query is 64M.

* RethinkDB requires data structures in RAM on each server proportional to the size of the data on that server's disk, usually around 1% of the size of the total data set. See [Understanding RethinkDB memory requirements](/docs/memory-usage) for more details.

## Key lengths ##

* Primary keys are limited to 127 characters.

* Secondary keys are indexed on their first 238&minus;*PK* bytes, where *PK* is the primary key length of that table. If a secondary index has keys whose first 238&minus;*PK* bytes are identical, performance using those keys will be degraded, as RethinkDB will fall back on a linear search.

* Secondary indexes do not store objects or `null` values. See [Using secondary indexes](/docs/secondary-indexes/) for more details.

## Data types ##

* Numbers are double precision [IEEE 754][fp] floating point. Integers from &minus;2<sup>53</sup> to 2<sup>53</sup> are stored precisely; integers outside that range may be rounded. RethinkDB does not allow `NaN` or infinite values.

* By default, arrays on the RethinkDB server have a size limit of 100,000 elements. This can be changed on a per-query basis with the `arrayLimit` (or `array_limit`) option to [run](/api/javascript/run).

[fp]: https://en.wikipedia.org/wiki/IEEE_floating_point

## Other notes ##

RethinkDB uses byte-wise ordering for indexes, `orderBy` and `between`. While this corresponds to codepoint ordering in UTF-8, RethinkDB does not support Unicode collations, and does not normalize for identical characters with multiple codepoints (i.e, `\u0065\u0301` and `\u00e9` both represent the character "&eacute;" but RethinkDB treats them, and sorts them as, distinct characters).

Some file systems, typically compressed or encrypted ones, may require the `--no-direct-io` option (see [Create a cluster on system startup](/docs/cluster-on-startup/) for more information on RethinkDB options).

There are currently issues with `btrfs`. Follow [issue #2781](https://github.com/rethinkdb/rethinkdb/issues/2781) for more information.

By default, RethinkDB can return data from concurrent writes that have not been committed to disk yet. The `read_mode` option to `table` allows control of the isolation level (starting in RethinkDB 2.1).
