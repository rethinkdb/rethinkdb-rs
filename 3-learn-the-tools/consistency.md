---
layout: documentation
title: Consistency guarantees
docs_active: consistency
permalink: docs/consistency/
---

Three settings control consistency and durability in RethinkDB: write acknowledgements and durability per table, and the query read mode.

# Settings

* *Write acknowledgements* are set per table with the `write_acks` setting, either using the [config][co] command or by writing to the `table_config` [system table][st]. The default is `majority`, meaning writes will be acknowledged when a majority of (voting) replicas have confirmed their writes. The other possible option is `single`, meaning writes will be acknowledged when a single replica acknowledges it.
* *Durability* is set per table with the `durability` setting, again using either `reconfigure` or writing to the `table_config` system table.  In `hard` durability mode, writes are committed to disk before acknowledgements are sent; in `soft` mode, writes are acknowledged immediately upon receipt. The `soft` mode is faster but slightly less resilient to failure. The default is `hard`.
* *Read mode* is set per query via an optional argument, `read_mode` (or `readMode`), to [table][tb]. It has three possible values:
    * `single` returns values that are in memory (but not necessarily written to disk) on the primary replica. This is the default.
    * `majority` will only return values that are safely committed on disk on a majority of replicas. This slows reads, as it requires sending a message to every replica on each read.
    * `outdated` will return values that are in memory on an arbitrarily-selected replica. This is the fastest but least consistent.

Note that [changefeeds][cf] will ignore the `read_mode` flag, and will always behave as if it is set to `single`.

[co]: /api/javascript/config/
[tb]: /api/javascript/table/
[st]: /docs/system-tables/
[cf]: /docs/changefeeds/

# Linearizability and atomicity guarantees

With the following settings, RethinkDB guarantees linearizability of individual atomic operations on individual documents:

* `write_acks`: `majority`
* `durability`: `hard`
* `read_mode`: `majority`

This means that every read will see every previous successful write, and no read will ever see a definitively failed write. (See note about definitively failed vs. indeterminate writes below.)

The linearizability guarantee is for *atomic operations,* not for *queries.* A single RethinkDB query will not necessarily execute as a single atomic operation. It's possible that the query:

```js
r.table("foo").get("bar").eq(r.table("foo").get("bar")).run(conn, callback);
```

could return `false`! Each individual `get` operation is atomic, but the query as a whole is not. To read and modify a document in a single atomic operation, use the `update` or `replace` commands.

```js
r.table("foo").get(id).update({hits: r.row("hits") + 1}).run(conn, callback);
```

This can also be used to implement a check-and-set register. The following query will atomically check whether the `check` field is equal to `old_value` and change it to `new_value` if so:

```js
r.table("foo").get(register_id).update({
    check: r.branch(r.row("check").eq(old_value), new_value, r.row("check"))
}).run(conn, callback);
```

{% infobox %}
RethinkDB operations are never atomic across multiple keys. For this reason, RethinkDB cannot be considered an ACID database.
{% endinfobox %}

Currently, `filter`, `get_all` and similar operations execute as separate operations from `update` and other mutation operations. (These operations are usually atomic, although not all `filter` operations are, depending on the predicate.) Therefore, the following is *not* a correct implementation of a check-and-set register, since `filter` and `update` will not execute in one atomic operation:

```js
r.table("foo").filter({
    id: register_id, foo: old_val
}).update({foo: new_val}).run(conn, callback);

table.filter({id: register_id, foo: old_val}).update({foo: new_val})
```

This behavior may change in the future. See [Github issue #3992][gh3992] to track the discussion.

[gh3992]: https://github.com/rethinkdb/rethinkdb/issues/3992 "Make table.filter.update atomic"

# Availability guarantees

Except for brief periods, a table will remain fully available as long as more than half of the voting replicas for each shard are available. If half or more of the voting replicas for a shard are lost, then read or write operations on that shard will fail.

Reconfiguring a table (changing the number of shards, shard boundaries, etc.) causes brief losses of availability at various points during the reconfiguration.

If the primary replica is lost but more than half of the voting replicas are still available, an arbitrary voting replica will be elected as primary. The new primary will appear in `table_status`, but the `primary_replica` field of `table_config` will not change. If the old primary ever becomes available again, the system will switch back. When the primary changes there will be a brief period of unavailability.

If half or more of the voting replicas of a shard are lost, the only way to recover availability is to run [config][co] with the `emergency_repair` option. Consult the documentation for `reconfigure` for more details.

Reads run in `"single"` mode may succeed even if the table is not available, but this is not guaranteed. Reads run in `"outdated"` mode will succeed as long as at least one replica for each of the relevant shards is available.

{% infobox %}
**Voting and non-voting?** All replicas are "voting" replicas by default, which simply means that they're counted in any operation that requires a majority of replicas to be available. However, the speed at which replicas "vote" is affected by network latency; if you have a faraway data center with higher latency, you might want to set its replicas to be non-voting to improve performance, at the cost of guaranteed availability in that data center. You can set a replica to be "non-voting" by changing its table configuration with `reconfigure`.
{% endinfobox %}

[fail]: /docs/failover/

# Balancing safety and performance

RethinkDB's default settings prioritize safety over performance, except in one case: `read_mode` defaults to `single` rather than `majority`. The `majority` read mode requires sending a query to all of the replicas and waiting for a majority to reply, which significantly degrades performance.

In normal operation, `single` read mode produces the same results as `majority` read mode during normal operation, but in the event of a network failure or crash, it might return outdated results. It's also possible that a read run in `single` mode could return results from an incomplete write that is later rolled back.

The same is true for `single` write mode and `soft` durability mode. In normal operation these produce the same results as `majority` and `hard`, but in the event of a network or server failure, recent write operations that were run using these modes might be lost.

Note that `write_acks` and `durability` don't actually affect how the write is performed; they only affect when the acknowledgement is sent back to the client.

Reads run in `"outdated"` mode will return outdated data even during normal operation, but the data will typically be less than a second out of date. In the event of a network or server failure, the data may be much more out of date. The advantage of running reads in `"outdated"` mode is that the latency and throughput are often better than in `"single"` mode, in addition to the availability differences described in the previous section.

# Notes

Using the `emergency_repair` option on a table will invalidate all the guarantees.

There are two ways a write operation can fail. If a write fails *definitively,* no read will ever see it, even in the weaker read modes. If it fails *indeterminately,* reads run in `single` or `outdated` modes might see it, but when the network failure or crash that caused the problem is resolved the write might or might not be rolled back. In general, writes will fail indeterminately if they were running at the exact moment when the network or server issue first happened. Both of these failures will generate errors, and you can examine the error message to see whether the failure was definitive or indeterminate.

RethinkDB's automatic failover has limitations in cases of non-transitive connectivity failure, i.e., server A can contact B and B can contact C, but A cannot contact C. Read the [Failover][fail] documentation for more details.
