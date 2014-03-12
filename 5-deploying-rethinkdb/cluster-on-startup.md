---
layout: documentation
title: Create a cluster on system startup
active: docs
docs_active: cluster-on-startup
permalink: docs/cluster-on-startup/
alias: docs/guides/startup/
---

# Startup with init.d #

On Linux, RethinkDB packages automatically install an init script at
`/etc/init.d/rethinkdb` and add default run-level entries.

## Quick setup ##

To get started, copy the the example config file from
`/etc/rethinkdb/default.conf.sample` into the instances directory
(`/etc/rethinkdb/instances.d/`) and restart the init.d script:

```bash
sudo cp /etc/rethinkdb/default.conf.sample /etc/rethinkdb/instances.d/instance1.conf
sudo vim /etc/rethinkdb/instances.d/instance1.conf # edit the options
sudo /etc/init.d/rethinkdb restart
```

The basic setup is complete &mdash; __you've now got a working server!__

The init.d script looks for filenames ending in `.conf` in
`/etc/rethinkdb/instances.d/`, and starts an instance of RethinkDB for
each config file found in this directory. The packages do not ship
with a default config file, so if you install RethinkDB, it will not
automatically be run on system startup until you add a config file to
`/etc/rethinkdb/instances.d/`.

## Multiple instances ##

The init.d script supports starting multiple instances on the same machine via
multiple `.conf` files in `/etc/rethinkdb/instances.d`. Note that the
init.d script produces a feedback line for each registered instance when
queried. This is not standard behavior for an init.d script, so if you
have a tool that depends upon standard init.d script output, you might
need to limit each machine to only one RethinkDB instance in
`/etc/rethinkdb/instances.d`.

The `http-port`, `driver-port` and `cluster-port` options are
mandatory when defining multiple instances.

## Installing from source ##

If you compiled from source, you can get the init.d script from
[here](https://github.com/rethinkdb/rethinkdb/blob/next/packaging/assets/init/rethinkdb)
on Github. You can get the sample config file on Github from
[here](https://github.com/rethinkdb/rethinkdb/blob/next/packaging/assets/config/default.conf.sample).

# Startup with systemd #

## Basic setup ##

Add the file `/usr/lib/tmpfiles.d/rethinkdb.conf` with the content:

```
d /run/rethinkdb 0755 rethinkdb rethinkdb -
```

Then add one more file `/usr/lib/systemd/system/rethinkdb@.service`

```
[Unit]
Description=RethinkDB database server for instance '%i'

[Service]
User=rethinkdb
Group=rethinkdb
ExecStart=/usr/bin/rethinkdb serve --config-file /etc/rethinkdb/instances.d/%i.conf
KillMode=process
PrivateTmp=true

[Install]
WantedBy=multi-user.target
```

The `chmod` for the two files should be 644.

## Starting RethinkDB instances ##

First, create the RethinkDB data directory with the following command:

```
rethinkdb create -d /path/to/your/rethinkdb/directory
```

Then, download the
[default.conf.sample](https://github.com/rethinkdb/rethinkdb/blob/next/packaging/assets/config/default.conf.sample),
and move it into `/etc/rethinkdb/instances.d/<name_instance>.conf`.

Finally, modify the .conf file with your desired settings and then
run:

```
sudo systemctl enable rethinkdb@<name_instance>
sudo systemctl start rethinkdb@<name_instance>
```

You've now got a working server!

## Multiple instances ##

Since `systemd` supports multiple instances, starting a new instance
of RethinkDB only requires creating another .conf file.

# Configuration options #

The `.conf` file includes a number of options for exclusive to the
init script. The rest of the options are exactly the same as the ones
that go on the command line to the RethinkDB server. For more details
about these options run `rethinkdb help`.

## Supported options ##

For some of the options below, the default value depends on `<name>`, the name of the
config file without the `.conf` extension.

* `runuser` and `rungroup` &mdash; specifies which
  user and group should be used launch the rethinkdb process. 
  **Defaults**: `rethinkdb` and `rethinkdb`.

* `pid-file` &mdash; the location of the file with the RethinkDB instance process ID (used by the init script to communicate with
  the server process).  
  **Default**: `/var/run/rethinkdb/<name>/pid_file` 

* `directory` &mdash; the data directory where
  database tables will be stored. This location must be readable and
  writable by the user or group (or both) specified by `runuser`
  and `rungroup`. Note, it is best to create the database manually via
  `rethinkdb create --directory ...` as `runuser` or `rungroup` before
  enabling auto-start.  
  **Default**: `/var/lib/rethinkdb/<name>/`

* `http-port`, `driver-port`, and `cluster-port` &mdash; the web UI
  port (default `8080`), the client driver port (default
  `28015`), and intracluster traffic port (default `29015`),
  respectively.
  
* `bind` &mdash; by default, the server process binds only to
  loop-back interfaces (`127.*.*.*`) and thus may not be accessible
  over the network. The `bind` option allows the server process to bind to additional interfaces. You can either specify IPv4 addresses of network interfaces or simply use `all` to
  bind to all interfaces.

* `join` &mdash; rethinkdb allows you to incrementally build a cluster by joining new
  nodes to others that are already running. The `join` option specifies which
  rethinkdb node to join via `host:port`. For example,
  `join=newton:29015` will join the node on host `newton` at
  intracluster port `29015`. You can also specify multiple `join` options
  in case some of your nodes are unreachable at the time of startup. 

# Troubleshooting #

{% infobox info %}
<strong>Seeing a 'received invalid clustering header' message?</strong>
{% include troubleshootingcluster.md %} 
{% endinfobox %}

