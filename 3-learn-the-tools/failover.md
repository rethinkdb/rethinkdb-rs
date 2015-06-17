---
layout: documentation
title: Failover
docs_active: failover
permalink: docs/failover/
---

When a server fails, it may be because of a network availability issue or something more serious, such as system failure. In a multi-server configuration, where tables have multiple replicas distributed among multiple physical machines, RethinkDB will be able to maintain availability automatically in many cases.

If the primary replica for a table fails, as long as more than half of the table's voting replicas and more than half of the voting replicas for each shard remain available, one of those voting replicas will be arbitrarily selected as the new primary. There will be a brief period of unavailability, but no data will be lost.

If half or more of the voting replicas of a shard are lost, an *emergency repair* will need to be performed. For more information on the emergency repair option, read the documentation for [reconfigure][rc].

[rc]: /api/javascript/reconfigure

{% infobox %}
**Voting and non-voting?** All replicas are "voting" replicas by default, which simply means that they're counted in any operation that requires a majority of replicas to be available. However, the speed at which replicas "vote" is affected by network latency; if you have a faraway data center with higher latency, you might want to set its replicas to be non-voting to improve performance, at the cost of guaranteed availability in that data center. You can set a replica to be "non-voting" by changing its table configuration with `reconfigure`.
{% endinfobox %}

## Limitations of automatic failover ##

In most circumstances, automatic failover can be performed as long as a majority of voting replicas are available. However, one circumstance in which it may not be performed is a non-transitive connectivity failure. Imagine a cluster with three servers: A, B, and C. Under normal network operations, all of the servers can connect to one another. If a network failure occurs such that A can connect to B and B can connect to C, but A cannot connect to C, the network failure is non-transitive. For a more in-depth description, as well as progress on a long-term solution, read [Github issue #4357][gh4357].

[gh4357]: https://github.com/rethinkdb/rethinkdb/issues/4357

Since automatic failover requires a majority of servers for a table to be available, it requires a minimum of three servers to be involved. If you have a two-machine cluster, automatic failover will never occur; if the primary replica for a table becomes unavailable, you must fix the problem manually using the emergency repair option of `reconfigure`.
