---
layout: documentation
title: Start a RethinkDB server
active: docs
docs_active: start-a-server
permalink: docs/start-a-server/
alias: docs/start-a-cluster/
---

{% infobox info %}
**Want to start instances of RethinkDB on system startup?** See [Start RethinkDB at system startup](/docs/start-on-startup/) to learn how to set up RethinkDB with `init.d` or `systemd`.
{% endinfobox %}

<img src="/assets/images/docs/api_illustrations/cluster.png" class="api_command_illustration" />

# Starting the server

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

{% infobox info %}
For a complete list of options that can be passed to RethinkDB on the command line, read [RethinkDB command line options](/docs/cli-options) or type `rethinkdb --help` at the terminal prompt.
{% endinfobox %}

#  Multiple RethinkDB instances on a single machine #

Adding a node to a RethinkDB cluster is as easy as starting a new
RethinkDB process and pointing it to an existing node in the
cluster. Everything else is handled by the system without any
additional effort required from the user.

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

For a complete list of options that can be passed to RethinkDB on the command line, read [RethinkDB command line options](/docs/cli-options), or type `rethinkdb --help` at the terminal prompt.

{% infobox info %}
<strong>Having trouble accessing the web interface?</strong> Try restarting both of your RethinkDB instances with an additional `--bind all` parameter.
{% endinfobox %}

In production, you'd likely want to specify options via configuration files rather than command line options; read the [configuration file](/docs/config-file/) documentation for details on the format and available options. Also, you'd want your RethinkDB instances to come online at system startup. See [Start RethinkDB at system startup](/docs/start-on-startup/) to learn how to set up RethinkDB with `init.d` or `systemd`.

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
`localhost` in order to prevent unauthorized clients on the network
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

