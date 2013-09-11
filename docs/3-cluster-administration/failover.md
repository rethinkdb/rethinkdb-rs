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

When a machine fails, it may be because of a network availability issue or
something more serious, such as system failure. If the machine will not be able
to reconnect to the cluster, you can __manually declare it dead__, which is a
simple one-click operation. 

When a machine is declared dead, the system will attempt to recover itself
automatically. However, dropping a machine from the cluster can affect table
availability and require additional manual intervention in the following cases:

- If the machine was acting as __primary for any shards__, the corresponding
  tables will lose read and write availability (out-of-date reads remain
  possible as long as there are other replicas of the shards).
- If the machine was acting as a replica for a given table and there are __not
  enough replicas in the cluster__ to respect the write acknowledgement
  settings (_acks_), the table will lose write availability, but maintains read
  availability. 

On the other hand, if the machine was acting as a replica for a given table and there are
enough replicas to respect the user's write acknowledgement settings (_acks_),
the system continues operating as normal and the affected tables will maintain
both read and write access. The system will be able to recover itself without
additional intervention.

## What to do when a machine goes down ##

In general, when a machine goes down, there are two possible solutions. The
first option is to simply wait for the machine to become reachable again. If
the machine comes back up, RethinkDB automatically performs the following
actions without any user interaction:

1. Replicas on the machine are brought up to date with the latest changes. 
2. Primaries on the machine become active again. 
3. The cluster clears the reachability issue from web and command
line tools, availability is restored, and the cluster continues
operating as normal.

The second option is to declare the machine dead. If a machine is
declared dead, it is absolved of all responsibilities, and one of the
replicas is automatically elected as a new primary. After the
machine is declared dead, availability is quickly restored and the
cluster begins operating normally. (If the dead machine comes back up,
it is rejected by the cluster as a "zombie" since it might have data conflicts).


## Example failover scenario using the web interface ##
In this example, we have 4 machines in our cluster and one table requiring 4
replicas and 4 acks.

As soon as one machine dies, the web interface reports an issue. This is what
you would see in the webUI:

![Issue on the web interface](/assets/images/docs/administration/failover1.png)

If we click on the _Resolve issues_ button, we should see more information
about the current issue.  In our case, the unreachable machine is a replica
(aka secondary) and therefore we have not lost any write availability.

![Issue on the web interface](/assets/images/docs/administration/failover2.png)

In this case, if we don't want to wait for the machine to come back online, we
can declare the machine dead. Declaring a machine dead means that we remove the
machine and all its data from the cluster.

{% infobox %}
__Warning__: Once we have declared a machine dead, all of its data will be
lost. Even if we later restart a RethinkDB instance with the same data
directory, we will not be able to reuse the data.
{% endinfobox %}

Once the machine is declared dead, we have only three machines left in our
cluster. Since we are requiring four replicas and four acks, the system raises
a new error: _Unsatisfiable goals_. This error means that our replication
requirements are not possible with the current cluster.

![Issue on the web interface](/assets/images/docs/administration/failover3.png)

There are two ways for us to solve this issue:

- Connect a new machine to the cluster
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



## Example failover scenario using the command-line interface ##
Connect to your cluster via the command-line interface:

```
rethinkdb admin --join <host>:<port>
```

In this example, a machine has already been declared dead so you should see
this warning:

```
There is 1 outstanding issue, run 'ls issues' for more information
```

You can resolve this issue with the following:

```
# List the issues first using `ls`:
localhost:29015> ls issues

Machine 9d1b4e33-346d-406b-a1e9-8ce1d47e0f28 is inaccessible.

# If we try to remove the machine with the `rm` command, we will get an error
# stating "unsatisfiable goals":
localhost:29015> rm machine 9d1b4e33-346d-406b-a1e9-8ce1d47e0f28

error: Namespace ecaf9874-5fe2-4627-97ee-69c7cafdd9a8 has unsatisfiable goals

# To solve this problem, we can lower the number of replicas:
localhost:29105> set acks ecaf9874-5fe2-4627-97ee-69c7cafdd9a8 3
localhost:29105> set replicas ecaf9874-5fe2-4627-97ee-69c7cafdd9a8 3

# The list of issues should now be empty:
localhost:29015> ls issues
```
