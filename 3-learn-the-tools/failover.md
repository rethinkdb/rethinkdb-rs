---
layout: documentation
title: Failover
docs_active: failover
permalink: docs/failover/
---
# About automatic failover #
{% infobox %}
__Note__: RethinkDB does not support fully automatic failover yet, but it is on
the roadmap. Follow [Github
issue #223](https://github.com/rethinkdb/rethinkdb/issues/223) for more information.
{% endinfobox %}

When a server fails, it may be because of a network availability issue or something more serious, such as system failure. If the server will not be able to reconnect to the cluster, you can permanently remove it.

When a server is permanently removed, the system will attempt to recover itself automatically. However, dropping a server from the cluster can affect table availability and require additional manual intervention in the following cases:

- If the server was acting as a primary replica for any shards, the corresponding tables will lose read and write availability (out-of-date reads remain possible as long as there are other replicas of the shards).

- If the server was acting as a replica for a given table and there are not
  enough replicas in the cluster to respect the write acknowledgement
  settings (_acks_), the table will lose write availability, but maintain read
  availability. 

If the server was acting as a replica for a given table and there are enough replicas to respect the user's write acknowledgement settings (_acks_), the system continues operating as normal and the affected tables will maintain both read and write access. The system will be able to recover itself without additional intervention.

## What to do when a server goes down ##

In general, when a server goes down, there are two possible solutions. The first option is to simply wait for the server to become reachable again. If the server comes back up, RethinkDB automatically performs the following actions without any user interaction:

1. Replicas on the server are brought up to date with the latest changes. 
2. Primary replicas on the server become active again. 
3. The cluster clears the reachability issue from web and command
line tools, availability is restored, and the cluster continues
operating as normal.

The second option is to permanently remove the server. If a server is permanently removed, it is absolved of all responsibilities, and one of the replicas is automatically elected as a new primary replica. After the server is removed, availability is quickly restored and the cluster begins operating normally. (If the server comes back up, it is rejected by the cluster as a "ghost," since it might have data conflicts.)

## Example failover scenario using the web interface ##

As soon as one server dies, the web interface reports an issue. If we click on the _Resolve issues_ button, we should see more information
about the current issue. In our case, the unreachable server is a secondary replica, and therefore we have not lost any write availability.

![Issue on the web interface](/assets/images/docs/administration/failover2.png)

In this case, if we don't want to wait for the server to come back online, we
can resolve the issue by permanently removing the server. This will delete the
server and all its data from the cluster.

{% infobox alert %}
__Warning__: Once we have permanently removed a server, all of its data will be
lost. Even if we later restart a RethinkDB instance with the same data
directory, we will not be able to reuse the data.
{% endinfobox %}

Once the server is removed, the problem is resolved&mdash;the tables that had replicas on that server will be reconfigured automatically. Note, however, that if you connect a new server, the table will not automatically be reconfigured to take advantage of it&mdash;you will need to reconfigure the table manually.

## Permanently removing a server with ReQL ##

Administration through ReQL is performed by querying [system tables](/docs/system-tables/). Server issues can be listed by querying the [current_issues](/docs/system-issues/) system table.

An unreachable server will be listed with a `server_disconnected` issue, with its UUID in the `disconnected_server` field. To permanently remove a server using ReQL administration commands, delete it by UUID from the `server_config` system table.

From the Data Explorer:

```js
r.db('rethinkdb').table('server_config').get(<UUID>).delete()
```

(The syntax is similar with other drivers.)
