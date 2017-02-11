---
layout: documentation
title: Scaling, sharding and replication
docs_active: sharding-and-replication
permalink: docs/sharding-and-replication/
---

{% toctag %}

RethinkDB allows you to shard and replicate your cluster on a per-table basis. Settings can be controlled easily from the web administration console. In addition, ReQL commands for table configuration allow both scripting capability and more fine-grained control over replication, distributing replicas for individual tables across user-defined groups of servers using server tags.

<img alt="Sharding and Replication Illustration" class="api_command_illustration"
    src="/assets/images/docs/api_illustrations/shard-and-replicate.png" />

# Multi-datacenter setup #

To group servers together in data centers, RethinkDB uses [Server tags](#server-tags). Servers can be "tagged" with one or more group names on startup:

```
rethinkdb --server-tag data_center_1
```

Once a server has been given a tag, the tags may be used to assign table replicas to servers with the same tags using the `reconfigure` command. Read the section of this document on [Server tags](#server-tags) for more details.

# Running a proxy node #

Once you have several machines in a RethinkDB cluster, you can improve your cluster's efficiency by running a _proxy node_ on each application server and having the client application connect to the proxy on `localhost`.

A proxy node doesn't store any data; instead it acts as a query router. This offers some performance advantages:

* The proxy will send queries directly to the correct machines, reducing intracluster traffic.
* If you're using [changefeeds][cf], the proxy will de-duplicate changefeed messages sent from other cluster nodes, further reducing traffic.
* The proxy node can do some query processing itself, reducing CPU load on database servers.

To run a proxy node, simply use the `proxy` command line option on startup.

[cf]: /docs/changefeeds

```
rethinkdb proxy --join hostname:29015
```

# Sharding and replication via the web console #

When using the web UI, simply specify the number of shards you want, and based on the data available RethinkDB will determine the best split points to maintain balanced shards. To shard your data:

- Go to the table view (_Tables_ &rarr; _table name_).
- Click on the _Reconfigure_ button.
- Set the number of shards and replicas you would like.
- Click on the _Apply Configuration_ button.

![Shard with the web interface](/assets/images/docs/administration/shard.png)

A table may have up to 64 shards.

# Sharding and replication via ReQL #

There are three primary commands for changing sharding and replication in ReQL. In addition, there are lower-level values that can be changed by manipulating [system tables](/docs/system-tables/).


* The [table_create](/api/python/table_create) (or [tableCreate](/api/javascript/table_create)) command can specify initial values for `shards` and `replicas`.
* The [reconfigure](/api/python/reconfigure) command can change the values for `shards` and `replicas` for an existing table.
* The [rebalance](/api/python/rebalance) command will rebalance table shards.

For more information about administration via ReQL, consult the API documentation for the individual commands as well as the [Administration tools][at] documentation.

[at]: /docs/administration-tools/

{% infobox %}
__Note__: Currently, RethinkDB implements range shards, but will eventually be
switching to hash shards. Follow [Github issue #364][gh364] to track progress.

[gh364]: https://github.com/rethinkdb/rethinkdb/issues/364
{% endinfobox %}


# Advanced configuration #

These tasks cannot be performed through the web interface.

## Server tags ##

All of the servers in a RethinkDB cluster may be given zero or more _tags_ that can be used in table configurations to map replicas to servers specified by tag.

A server can be given tags with the `--server-tag` option on startup:

```
rethinkdb --server-tag us --server-tag us_west
```


While running, a server's configuration can be changed by writing to the `rethinkdb.server_config` [system table](/docs/system-tables/).

```py
# get server by UUID
r.db('rethinkdb').table('server_config').get(
    'd5211b11-9824-47b1-9f2e-516a999a6451').update(
    {tags: ['default', 'us', 'us_west']}).run(conn)
```

If no tags are specified on startup, the server will be started with one tag, `default`. Changing the sharding/replica information from the web UI or from ReQL commands that do not specify server tags will affect all servers with the `default` tag.

{% infobox alert %}
The web UI only affects servers with the `default` tag. If you remove the `default` tag from a server or start it without that tag, it will not be used for tables configured through the web UI.
{% endinfobox %}

When servers are tagged, you can use the tags in the [reconfigure](/api/python/reconfigure) command. To assign 3 replicas of the `users` table to `us_west` and 2 to `us_east`:

```py
r.table('users').reconfigure(shards=2, replicas={'us_west':3, 
    'us_east':2}, primary_replica_tag='us_east').run(conn)
```

If you remove *all* of a server's tags and then reconfigure all the cluster's tables, that server will be taken out of service.

```py
# decommission a server
r.db('rethinkdb').table('server_config').get(
    'd5211b11-9824-47b1-9f2e-516a999a6451').update(
    {tags: []}).run(conn)
r.db('database').reconfigure(shards=2, replicas=3).run(conn)
```

Note that tables are configured on creation and when the `reconfigure` command is called, but the configurations are *not* stored by the server otherwise. To reconfigure tables consistently&mdash;especially if your configuration uses server tags&mdash;you should save the configuration in a script. Read more about this in [Administration tools][at].

## Write acks and durability ##

Two settings for tables, write acknowledgements and write durability, cannot be set through either the web interface or the `reconfigure` command. They must be set by modifying the `table_config` table for individual tables.

The write acknowledgement setting for a table controls when the cluster acknowledges a write request as fulfilled. There are two possible settings:

* `majority`: The cluster sends the acknowledgement when the majority of replicas have acknowledged it. This is the default.
* `single`: The cluster sends the acknowledgement when any replica has acknowledged it.

To change these settings for a table:

```py
r.db('rethinkdb').table('table_config').get(
    '31c92680-f70c-4a4b-a49e-b238eb12c023').update(
        {"write_acks": "single"}).run(conn)
```

The `durability` setting for a table controls when writes are committed. In `hard` durability mode, writes are committed to disk before acknowledgements are sent; in `soft` mode, writes are acknowledged immediately upon receipt. The `soft` mode is faster but slightly less resilient to failure.
