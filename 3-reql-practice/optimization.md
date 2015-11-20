---
layout: documentation
title: Optimizing query performance
docs_active: optimization
permalink: /docs/optimization/
---

Understanding how RethinkDB parallelizes queries can improve the performance of your applications&mdash;sometimes significantly.

{% toctag %}

# Sharding

The basic rule is:

**Processing happens where the data is until an operation needs to combine it.**

In other words, ReQL queries that involve multiple shards will be processed on those shards whenever possible.

Let's follow the processing of a simple query. (This example uses JavaScript, but the commands are virtually identical in other languages.)

```js
r.table('users').filter({role: 'admin'}).run(conn, callback);
```

RethinkDB will process this query with the following steps:

1. The query is sent to a server for execution.
2. The [filter][] operation is performed in parallel on each shard of the `users` table.
3. The result of the `filter` is sent from the shards to the query server and combined.
4. The result is returned to the client.

[filter]: /api/javascript/filter/

However, an [orderBy][] query will be executed differently.

[orderBy]: /api/javascript/order_by/

```js
r.table('users').orderBy('username').run(conn, callback);
```

1. The query is sent to a server for execution.
2. Data is sent from the shards to the query server and combined.
3. The `orderBy` operation is performed on the query server.
4. The result is returned to the client.

An `orderBy` operation (without an index) can't be distributed across the shards for parallel execution&mdash;it needs all the data in the table to perform a sort.

The following commands can be distributed across shards:

* Selections: `between`, `get_all`, `filter`
* [Map-reduce][mr] operations: `map`, `concat_map`, `reduce`
* `group`
* Derived terms: `pluck`, `with_field`, `count`, `eq_join`
* `order_by` *with* indexes

[mr]: /docs/map-reduce/

The order in which you chain ReQL commands can affect performance. For an example, imagine combining the previous two queries to return an ordered list of names of admin users. The `filter` operation can be distributed across shards, but the `orderBy` operation cannot. So this query:

```js
r.table('users').filter({role: 'admin'}).orderBy('name').run(conn, callback);
```

Is preferable to this query:

```js
r.table('users').orderBy('name').filter({role: 'admin'}).run(conn, callback);
```

Commands that stop subsequent commands from being parallelized include:

* `reduce`
* `order_by` (with or without indexes)
* `distinct`
* `limit`
* `max`, `min`, `avg`

Any command that requires the results from the shards to be combined on the server executing the query will finish executing on that server rather than being distributed. Optimize your queries by putting commands that can execute in parallel *before* commands that combine the result set whenever possible.

# Replication

RethinkDB's defaults tend to prioritize safety over performance. One of those defaults is that queries will be sent to the primary replicas for shards, which will always have current data (although that data may be returned to a query before it's been committed to disk).

You can increase the performance of a query by using the `outdated` read mode, which allows the cluster to return values from memory on arbitrarily-selected replicas.

```js
r.table('users', {readMode: 'outdated'}).
  filter({role: 'admin'}).run(conn, callback);
```

While `outdated` reads are faster, they are the least consistent. For more information on this option, read "Balancing safety and performance" in the [Consistency guarantees][cg] documentation.

[cg]: /docs/consistency/#balancing-safety-and-performance

# Proxy nodes

Starting RethinkDB with the `proxy` command turns a server into a *proxy node,* which acts as a query router. This increases cluster performance by reducing intracluster traffic and, if you're using changefeeds, de-duplicating feed messages.

For more information about proxy nodes, read "Running a proxy node" under [Scaling, sharding and replication][ssr].

[ssr]: /docs/sharding-and-replication/#running-a-proxy-node
