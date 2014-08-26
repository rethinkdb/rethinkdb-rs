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

The packages do not ship with a default config file, so if you install RethinkDB, it
will not automatically be run on system startup until you add a config file to
`/etc/rethinkdb/instances.d/`.

Copy the the example config file and restart the init.d script:

```bash
sudo cp /etc/rethinkdb/default.conf.sample /etc/rethinkdb/instances.d/instance1.conf
sudo vim /etc/rethinkdb/instances.d/instance1.conf # Edit some options if needed
sudo /etc/init.d/rethinkdb restart
```

The basic setup is complete &mdash; __you've now got a working server!__


## Multiple instances ##

The init.d script supports starting multiple instances on the same machine via
multiple `.conf` files in `/etc/rethinkdb/instances.d`.

## Installing from source ##

If you compiled from source, you can get the init.d script from
[here](https://github.com/rethinkdb/rethinkdb/blob/next/packaging/assets/init/rethinkdb)
on Github. You can get the sample config file on Github from
[here](https://github.com/rethinkdb/rethinkdb/blob/next/packaging/assets/config/default.conf.sample).

# Startup with systemd #

## Basic setup ##

Support for systemd is on its way &mdash; You can track progress on
[issue #2014](https://github.com/rethinkdb/rethinkdb/issues/2014)

Create the file `/usr/lib/tmpfiles.d/rethinkdb.conf` with the content:

```
d /run/rethinkdb 0755 rethinkdb rethinkdb -
```

Add one more file `/usr/lib/systemd/system/rethinkdb@.service`

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

The `chmod` for the two files should be 644 (`chmod 644 <file>`).


## Starting RethinkDB instances ##

First, create the RethinkDB data directory with the following command:

```
rethinkdb create -d /path/to/your/rethinkdb/directory
```

Download
[default.conf.sample](https://github.com/rethinkdb/rethinkdb/blob/next/packaging/assets/config/default.conf.sample),
in `/etc/rethinkdb/instances.d/<name_instance>.conf`, set your desired settings and run:

```
sudo systemctl enable rethinkdb@<name_instance>
sudo systemctl start rethinkdb@<name_instance>
```

__You've now got a working server!__


## Multiple instances ##

Since `systemd` supports multiple instances, starting a new instance
of RethinkDB only requires creating another `.conf` file.

# Configuration options #

The `.conf` file includes a number of options exclusively for the
init script. The rest of the options are exactly the same as the ones
that go on the command line to the RethinkDB server. For more details
about these options run `rethinkdb help`.

## Supported options ##

For some of the options below, the default value depends on `<name>`, the name of the
config file without the `.conf` extension.

* `runuser` and `rungroup` &mdash; specifies which
  user and group should be used launch the Rethinkdb process.   
  *Default*: `rethinkdb` and `rethinkdb`.

* `pid-file` &mdash; the location of the file with the RethinkDB instance process ID (used by the init script to communicate with
  the server process).   
  *Default*: `/var/run/rethinkdb/<name>/pid_file` 

* `directory` &mdash; the data directory where
  database tables will be stored. This location must be readable and
  writable by the user or group (or both) specified by `runuser`
  and `rungroup`.   
  _Note_: It is best to create the database manually via
  `rethinkdb create --directory ...` as `runuser` or `rungroup` before
  enabling auto-start.  
  *Default*: `/var/lib/rethinkdb/<name>/`

* `log-file` &mdash; path to the log file.  
  *Default*: `<directory>/log_file`

* `bind` &mdash; Address of local interfaces to listen on when accepting connections.
   May be 'all' or an IP address, loopback addresses are enabled by default.  
   *Default*: all local addresses

* `canonical-address` &mdash; Address that other rethinkdb instances will use to connect to this machine.
  It can be specified multiple times.

* `http-port`, `driver-port`, and `cluster-port` &mdash; the web UI
  port (default `8080`), the client driver port (default
  `28015`), and intracluster traffic port (default `29015`),
  respectively.

* `join` &mdash; The `host:port` of a node that Rethinkdb will connect to.
  It can be specified multiple times.

* `port-offset` All ports used locally will have this value added.  
  *Default*: 0
  
* `no-http-admin` &mdash; Disable web administration console.

* `cores` &mdash; Number of cores to use.  
  *Default*: Number of cores of the CPU.

* `cache-size` &mdash; Size of the cache in MB.  
  *Default*: Half of the available RAM on startup.

* `io-threads` &mdash; Number of simultaneous I/O operations can happen at the same time.  
  *Default*: 64

* `no-direct-io` &mdash; Disable direct I/O.

* `machine-name` &mdash; The name for this machine (as it will appear in the metadata).  
  *Default*: Randomly chosen from a short list of names.



# Troubleshooting #

{% infobox info %}
<strong>Seeing a 'received invalid clustering header' message?</strong>
{% include troubleshootingcluster.md %} 
{% endinfobox %}

