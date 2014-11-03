---
layout: documentation
title: Sharding and replication
active: docs
docs_active: sharding-and-replication
permalink: docs/sharding-and-replication/
---

<img alt="Sharding and Replication Illustration" class="api_command_illustration"
    src="/assets/images/docs/api_illustrations/shard-and-replicate.png" />

RethinkDB allows you to shard and replicate your cluster on a per-table basis.

Sharding a table is as easy as typing the number of shards you'd like in the
web admin and clicking 'Rebalance'. RethinkDB rebalances the table, creates
copies in the cluster, and moves necessary data around behind the scenes
without any additional work from the user. Similarly, to change the number of
replicas you simply set the number in the web UI and hit 'Save'. 

Sharding and replication settings can also be
controlled from a powerful command-line interface: `rethinkdb admin`.

# Sharding #
{% infobox info %}
__Note__: Currently, RethinkDB implements range shards, but will eventually be
switching to hash shards (follow [Github
issue #364](https://github.com/rethinkdb/rethinkdb/issues/364) to track progress).
{% endinfobox %}

## Sharding via the web interface ##
 When using the web UI, simply  specify the number of shards you
 want, and based on the data available RethinkDB will determine the best split
 points to maintain balanced shards. To shard your data: 

- Go to the table view (_Tables_ &rarr; _table name_).
- Click on the _Edit_ button under shard settings.
- Set the number of shards you would like.
- Click on the _Rebalance_ button.

![Shard with the web interface](/assets/images/docs/administration/shard.png)

## Sharding via the command-line interface ##
Connect to your cluster via the command-line interface:

```
rethinkdb admin --join <host>:<port>
```

Shards are managed by specifying a set of split points. A split point is the
primary key upon which the table will be sharded. By adding and removing split
points, you can add or remove shards to a table.

- Find the UUID of the table you want to shard using `ls`.
- To list existing shards, use `ls <table_name>` or `ls <table_uuid>`.
- Add a new split point, creating a new shard, using `split shard <table_uuid>
  <split_point>`.
- Remove a split point, removing an existing shard, using `merge shard
  <table_uuid> <split_point>`.

# Replication #
There are two parameters that can be set when dealing with replicas in
RethinkDB:

- The number of _replicas_: the number of copies of your data.
- The number of _acknowledgements_ (also referred to as _acks_): the number of
  confirmations required before a write is acknowledged.  

These two parameters can be specified for each table on a per-datacenter basis
or for the whole cluster (which includes servers that are not assigned to any
datacenter).

The primary constraints on these parameters are:

- You cannot require more replicas than you have servers available in your
  cluster.
- The number of _acks_ has to be less than or equal to the number of
  _replicas_, or no write will ever be acknowledge.

## Replication via the web interface ##
To replicate your data through the web UI:

- Go to the table view (_Tables_ > _table name_).
- Click on the _Edit_ button under replication settings.
- Set the number of _replicas_ and _acks_ you would like.
- Click on the _Update_ button.

![Replica with the web interface](/assets/images/docs/administration/replica.png)

## Replication via the command-line interface ##
Connect to your cluster via the command-line interface:

```
rethinkdb admin --join <host>:<port>
```

You can change the number of _replicas_ and _acks_ using the following commands:

```
set acks <table> <num_acks> [<datacenter>]
set replicas <table> <num_replicas> [<datacenter>]
```

# Pinning masters to datacenters #
Because RethinkDB is immediately consistent, each shard has to be assigned to a
master (also called a primary server).  The web interface provides an easy way
to pin primaries to a datacenter, but does not let the user pin a primary per
shard or per server basis. If you need this level of control, you will have to
use the command-line interface instead.

## Choosing a primary using the web interface  ##
By default, the primary for a shard can be put anywhere in the cluster. That is
to say, there is no constraint that requires the primary to be in a particular
datacenter.  In order to set a certain datacenter to contain all the primaries
of your table, you will have to:

- Go to the table view (_Tables_ > _table name_).
- Click on the _Show multi-datacenter options_.
- Click on the toggle box to _Pin masters to a single datacenter_.
- Choose the datacenter and click the _Update_ button.


![Change primary with the web interface](/assets/images/docs/administration/primary.png)

## Choosing a primary using the command-line interface ##
Connect to your cluster via the command-line interface:

```
rethinkdb admin --join <host>:<port>
```

- Find the UUID of the table you want to shard using `ls`.
Once you find the UUID of your table using the `ls` command, you can pin all of
the primaries for a table to a particular datacenter with:
- To pin all of the primaries for a table to a particular datacenter, use `set primary <table> <datacenter>`.

The command line interface also provides a more precise way to pin data. You
can pin a shard (primary or secondary) to a particular server. The command to
do this is:

```
pin shard <table> <shard> [--master <server>] [--replicas <server>...]
```
