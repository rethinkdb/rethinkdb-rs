---
layout: documentation
title: Start RethinkDB at system startup
active: docs
docs_active: start-on-startup
permalink: docs/start-on-startup/
alias: docs/guides/startup/
---

This document explains how to set up RethinkDB to run as a system service on supported operating systems, automatically launching on boot. For general instructions on starting RethinkDB, see [Start a RethinkDB server][srv].

[srv]: /docs/start-a-server/

In general, you'll have to follow these steps:

* Install RethinkDB as a service for your operating system. (This document describes how to do that for both `init.d` and `systemd`-based Linux distributions. Depending on how you've installed RethinkDB, this may already be done for you.)
* Create a RethinkDB configuration file for each RethinkDB instance running on this physical server.

# Startup with init.d #

RethinkDB packages automatically install an init script at `/etc/init.d/rethinkdb` and add default run-level entries. For RethinkDB to automatically run on system startup, you'll need to add a config file to `/etc/rethinkdb/instances.d/`.

## Quick setup ##

Copy the sample configuration file and use the [configuration file](/docs/config-file) documentation as a guide to customize it. (If you don't have the sample `.conf` file, you can download it [here][conf].)

[conf]: https://github.com/rethinkdb/rethinkdb/blob/next/packaging/assets/config/default.conf.sample

```bash
sudo cp /etc/rethinkdb/default.conf.sample /etc/rethinkdb/instances.d/instance1.conf
sudo vim /etc/rethinkdb/instances.d/instance1.conf
```

Then, restart the service:

```bash
sudo /etc/init.d/rethinkdb restart
```

The basic setup is complete &mdash; __you've now got a working server!__

## Multiple instances ##

The init.d script supports starting multiple instances on the same server via
multiple `.conf` files in `/etc/rethinkdb/instances.d`.

Include the `join` configuration option for each node with the IP address and port of another node in the cluster. If the instances are not running on the same machine, specify `bind=all` in the configuration file (or `--bind all` on the command line). Take care that each instance on the same machine specifies different values for `driver-port`, `cluster-port` and `http-port`.

{% infobox %}
The `bind=all` option is a security risk if your machine is open to the internet, and you should take steps to prevent unauthorized access. See the [security page](/docs/security/) for more details.
{% endinfobox %}

## Installing from source ##

If you compiled from source, get the `init.d` script from
[here](https://github.com/rethinkdb/rethinkdb/blob/next/packaging/assets/init/rethinkdb).
Get the sample `.conf` file [here][conf].

# Startup with systemd #

Full support for systemd is planned&mdash;you can track progress on [issue 2014](https://github.com/rethinkdb/rethinkdb/issues/2014). For now, you'll have to create a couple configuration files manually.

## Basic setup ##

Create the file `/usr/lib/tmpfiles.d/rethinkdb.conf` with the content:

```
d /run/rethinkdb 0755 rethinkdb rethinkdb -
```

And create the service file, `/usr/lib/systemd/system/rethinkdb@.service`:

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

Then, copy the sample configuration file and use the [configuration file](/docs/config-file) documentation as a guide to customize it. (If you don't have the sample `.conf` file, you can download it [here][conf].)

```bash
sudo cp /etc/rethinkdb/default.conf.sample /etc/rethinkdb/instances.d/instance1.conf
sudo vim /etc/rethinkdb/instances.d/instance1.conf
```

Then, enable the service:

```bash
sudo systemctl enable rethinkdb@<name_instance>
sudo systemctl start rethinkdb@<name_instance>
```

__You've now got a working server!__

## Multiple instances ##

As systemd supports multiple instances on the same server, you simply need to create multiple `.conf` files in `/etc/rethinkdb/instances.d`.

Include the `join` configuration option for each node with the IP address and port of another node in the cluster. If the instances are not running on the same machine, specify `bind=all` in the configuration file (or `--bind all` on the command line). Take care that each instance on the same machine specifies different values for `driver-port`, `cluster-port` and `http-port`.

{% infobox %}
The `bind=all` option is a security risk if your machine is open to the internet, and you should take steps to prevent unauthorized access. See the [security page](/docs/security/) for more details.
{% endinfobox %}


# Troubleshooting #

{% infobox info %}
<strong>Seeing a 'received invalid clustering header' message?</strong>
{% include troubleshootingcluster.md %} 
{% endinfobox %}

