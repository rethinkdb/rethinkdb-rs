---
layout: documentation
title: Start a RethinkDB cluster
active: docs
docs_active: start-a-cluster
permalink: docs/start-a-cluster/
---

{% infobox info %}
<strong>Want to set up a production cluster?</strong> See the [production cluster setup page](/docs/cluster-on-startup/) to learn how to set up RethinkDB with `init.d` or `systemd`.
{% endinfobox %}

Adding a node to a RethinkDB cluster is as easy as starting a new
RethinkDB process and pointing it to an existing node in the
cluster. Everything else is handled by the system without any
additional effort required from the user.

You can set up multiple RethinkDB instances on a single machine, or
use multiple physical machines or VMs to run a distributed cluster.

#  Multiple RethinkDB instances on a single machine #

To start the first RethinkDB instance, run this command in your
terminal:

```
$ rethinkdb
info: Creating directory /home/user/rethinkdb_data
info: Listening for intracluster connections on port 29015
info: Listening for client driver connections on port 28015
info: Listening for administrative HTTP connections on port 8080
info: Server ready
```

Note the port numbers you can use to access RethinkDB:

* Use the intracluster port (`29015` by default) to connect other nodes in the cluster to this node.
* Point your browser to the HTTP connections port (`8080` by default) to access the web interface.

Now start the second RethinkDB instance on the same machine:

```
$ rethinkdb --port-offset 1 --directory rethinkdb_data2 --join localhost:29015
info: Creating directory /home/user/rethinkdb_data2
info: Listening for intracluster connections on port 29016
info: Attempting connection to 1 peer...
info: Connected to server "Chaosknight" e6bfec5c-861e-4a8c-8eed-604cc124b714
info: Listening for client driver connections on port 28016
info: Listening for administrative HTTP connections on port 8081
info: Server ready
```

__You now have a RethinkDB cluster!__ Try pointing your browser to
`localhost:8080` or `localhost:8081` to access the web interface. If
you click on the "Servers" tab at the top, you should see both servers
in the cluster.

You can also point the client drivers to `localhost:28015` or
`localhost:28016` to start running queries (it doesn't matter which
node you use &mdash; the cluster will automatically route all commands to
appropriate nodes).

Note the command line parameters we used to start the second node:

- `--port-offset` &mdash; increment all ports by 1 so the two nodes don't try to use the same ports on one machine.
- `--directory` &mdash; use a different data directory so the two nodes don't try to access the same files.
- `--join` &mdash; tell our new RethinkDB instance to connect to another instance (in this case, `localhost:29015`).

{% infobox info %}
<strong>Having trouble accessing the web interface?</strong> Try restarting both of your RethinkDB instances with an additional `--bind all` parameter.
{% endinfobox %}

{% infobox info %}
<strong>Want to connect a third node?</strong> You can join it with either of the two existing nodes in the cluster.
{% endinfobox %}

# A RethinkDB cluster using multiple machines #

Starting a cluster on multiple machines or VMs is even easier than
starting it on a single machine, because you don't have to worry about
port and directory conflicts.

First, start RethinkDB on the first machine:

```
$ rethinkdb --bind all
```

Then start RethinkDB on the second machine:

```
$ rethinkdb --join IP_OF_FIRST_MACHINE:29015 --bind all
```

__You now have a RethinkDB cluster!__

Note that by default, RethinkDB only opens connections bound to
`localhost` in order to prevent unathorized clients on the network
from connecting to the server. The `--bind all` option allows
connections from anywhere on the network. It works well if the network
is protected.

If your network is open to the internet, you might have to take
additional precautions. See the [security page](/docs/security/) for
more details.

# Troubleshooting #

{% infobox info %}
<strong>Seeing a 'received invalid clustering header' message?</strong>
{% include troubleshootingcluster.md %} 
{% endinfobox %}

