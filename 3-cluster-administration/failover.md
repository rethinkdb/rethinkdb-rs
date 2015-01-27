---
layout: documentation
title: Failover
active: docs
docs_active: failover
permalink: docs/failover/
---
# About automatic failover #
{% infobox info %}
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

In this example, we have 4 servers in our cluster and one table requiring 4
replicas and 4 acks.

As soon as one server dies, the web interface reports an issue. This is what
you would see in the webUI:

![Issue on the web interface](/assets/images/docs/administration/failover1.png)

If we click on the _Resolve issues_ button, we should see more information
about the current issue.  In our case, the unreachable server is a replica
(aka secondary) and therefore we have not lost any write availability.

![Issue on the web interface](/assets/images/docs/administration/failover2.png)

In this case, if we don't want to wait for the server to come back online, we
can resolve the issue by permanently removing the server. This will delete the
server and all its data from the cluster.

{% infobox %}
__Warning__: Once we have permanently removed a server, all of its data will be
lost. Even if we later restart a RethinkDB instance with the same data
directory, we will not be able to reuse the data.
{% endinfobox %}

Once the server is removed, we have only three servers left in our
cluster. Since we are requiring four replicas and four acks, the system raises
a new error: _Unsatisfiable goals_. This error means that our replication
requirements are not possible with the current cluster.

![Issue on the web interface](/assets/images/docs/administration/failover3.png)

There are two ways for us to solve this issue:

- Connect a new server to the cluster
- Lower the number of replicas and acks required

If we decide to click on the _Lower replicas_ button, the system will lower the
replicas just enough to solve the issue (in our case 3 replicas and 3 acks).

{% infobox info %}
__Note__: There can sometimes be a situation where the system has the option to
lower the number of replicas in a specific datacenter or in the whole cluster.
In this case, there is no way for the system to know which option is preferred, so
it will require the user to reduce the number of replicas manually.
{% endinfobox %}

Once the number of replicas has been reduced, there are no remaining issues in
our cluster and the warning disappears.

![Issue on the web interface](/assets/images/docs/administration/failover4.png)

## Permanently removing a server with ReQL ##

Administration through ReQL is performed by querying [system tables](/docs/system-tables/). Server issues can be listed by querying the [current_issues](/docs/system-issues/) system table.

An unreachable server will be listed with a `server_disconnected` issue, with its UUID in the `disconnected_server` field. To permanently remove a server using ReQL administration commands, delete it by UUID from the `server_config` system table.

From the Data Explorer:

```js
r.db('rethinkdb').table('server_config').get(<UUID>).delete()
```

(The syntax is similar with other drivers.)