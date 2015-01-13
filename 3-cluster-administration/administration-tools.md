---
layout: documentation
title: Administration tools
active: docs
docs_active: administration-tools
permalink: docs/administration-tools/
alias: docs/guides/administration/
js: fancybox
---

RethinkDB provides a web interface which lets you manage your entire server cluster, from controlling sharding and replication to running ReQL queries (in JavaScript), with editing and history support. In addition, you can perform administration tasks using scriptable ReQL commands.

# The web interface #

<a class="screenshot-thumbnail" href="/assets/images/docs/administration/webui.png"><img src="/assets/images/docs/administration/thumbnails/webui.png" /></a>

Once RethinkDB is running, you can connect to it at <http://localhost:8080>, assuming you've kept the default port (8080) and it's running on your local machine.

By default, RethinkDB binds the web interface to `localhost` for security reasons. If you need to be able to access it from another server, use the `--bind all` parameter when starting RethinkDB. Read how to [Start a cluster][sc] or [Create a cluster on system startup][cc].

[sc]: /docs/start-a-cluster/
[cc]: /docs/cluster-on-startup/

# ReQL administration commands #

With the appropriate [client driver][cd] installed you can use a supported language to perform all administration tasks, either from the language's REPL or as a script. There are ReQL commands for [configuring sharding and replication](/api/python/reconfigure), [rebalancing shards](/api/python/rebalance) and more. In addition, you can query [system tables](/docs/system-tables/) to get information about your cluster and to change many of its operational characteristics.

[cd]: /docs/install-drivers/

These examples use Python, but there's equivalent functionality in Ruby, and any other scripting language with a RethinkDB driver updated for version 1.16 or later. Read the API documentation for more information on specific commands along with descriptions of their return values.

## Using a REPL ##

Load `python` (or [ipython](http://ipython.org)) and set up a connection to your database:

```py
import rethinkdb as r
r.connect('localhost', 28015).repl()
```

Now, you can use ReQL commands to query system tables and perform reconfiguration commands. To return the server status, you can query the `server_status` system table in the special `rethinkdb` database.

```py
list(r.db('rethinkdb').table('server_status').run())

[{u'status': u'available', u'network': {u'canonical_addresses': [{u'host':
u'127.0.0.1', u'port': 29015}, {u'host': u'::1', u'port': 29015}],
u'http_admin_port': 8080, u'hostname': u'rethinkdb.local', u'cluster_port':
29015, u'reql_port': 28015}, u'process': {u'version': u'rethinkdb
1.15.2-1425-gad513b (CLANG 6.0 (clang-600.0.56))', u'pid': 69596,
u'cache_size_mb': 100, u'argv': [u'./build/release_clang/rethinkdb'],
u'time_started': datetime.datetime(2014, 12, 12, 22, 43, 56, 651000,
tzinfo=<rethinkdb.ast.RqlTzinfo object at 0x10c13d1d0>)}, u'connection':
{u'time_connected': datetime.datetime(2014, 12, 12, 22, 43, 56, 654000,
tzinfo=<rethinkdb.ast.RqlTzinfo object at 0x10c13d250>), u'time_disconnected':
None}, u'id': u'6dbc31fe-8f78-4128-af76-cdac43bcc195', u'name':
u'rethinkdb_local_qpp'}]
```

To return the status on a specific table, you can use the [table_status](/api/python/table_status) command.

```py
list(r.table_status('superheroes').run())
```

And reconfiguring a table can be done the [reconfigure](/api/python/reconfigure) command.

```py
r.table('a').reconfigure(shards=2,replicas=2).run()

r.table('b').reconfigure(shards=2,replicas={'us_east':2, 'us_west':2,
    'london':2}).run()
```

The Data Explorer in the web administration UI is itself a JavaScript REPL, with syntax highlighting and history. (The article on [ReQL data exploration][rde] goes into some detail on how to use the Data Explorer.) The advantage of scripting languages with ReQL comes into play when writing administration scripts.

## Scripting ReQL ##

By using ReQL with a language like Python, it becomes easy to script administrative and configuration tasks with RethinkDB. If you have complex table configurations that might need to be repeated for new tables or tweaked for the whole database, you can store them in a script.

```py
import rethinkdb as r
conn = r.connect('localhost', 28015)

# Configure the entire database
r.db('database').reconfigure(shards=2, replicas=3).run(conn)

# Configure a set of specific tables
tables = ['users', 'posts', 'comments']
for table in tables:
    r.table(table).reconfigure(shards=3, replicas=2).run(conn)

# Configure all tables that are not related to logging
tables = [t for t in r.table_list().run() if 'log_' not in t]
for table in tables:
    r.table(table).reconfigure(shards=2, replicas=3).run(conn)

# Retrieve the current configuration of all the tables
# This uses the table_config system table
configs = r.db('rethinkdb').table('table_config')

# Restore the configuration of tables saved in 'configs'
for config in configs:
    r.db('rethinkdb').table('table_config').get(
    config['id']).update(config).run(conn)
```

Scripting is also the only way to access some advanced features such as server tags, which let you group servers together for replication purposes (such as associating them with physical data centers). For more information, read the "Advanced configuration" section of [Sharding and replication][sr].

[sr]: /docs/sharding-and-replication/
