---
layout: documentation
title: Start RethinkDB at system startup
docs_active: start-on-startup
permalink: docs/start-on-startup/
alias:
    - docs/guides/startup/
    - docs/cluster-on-startup/
---

This document explains how to set up RethinkDB to run as a system service on supported operating systems, automatically launching on boot. For general instructions on starting RethinkDB, see [Start a RethinkDB server][srv].

[srv]: /docs/start-a-server/

In general, you'll have to follow these steps:

* Install RethinkDB as a service for your operating system. (This document describes how to do that for both `init.d` and `systemd`-based Linux distributions, as well as OS X using `launchd`. Depending on how you've installed RethinkDB, this may already be done for you.)
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

The basic setup is complete&mdash;__you've now got a working server!__

## Multiple instances ##

The init.d script supports starting multiple instances on the same server via
multiple `.conf` files in `/etc/rethinkdb/instances.d`. This may be desirable for isolating databases for separate applications running on the same server, or for testing purposes. (There is no performance gain from running multiple nodes of the same cluster on the same physical machine.)

In each configuration file, set a different data directory, and include the `join` configuration option for each node with the IP address and port of another node in the cluster. If the instances are not running on the same machine, specify `bind=all` in the configuration file (or `--bind all` on the command line). Take care that each instance on the same machine specifies different values for `driver-port`, `cluster-port` and `http-port`.

{% infobox alert %}
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

First, create the RethinkDB data directory with the following command and set the ownership to the `rethinkdb` user:

```
rethinkdb create -d /path/to/your/rethinkdb/directory
sudo chown -R rethinkdb.rethinkdb /path/to/your/rethinkdb/directory
```

Then, copy the sample configuration file and use the [configuration file](/docs/config-file) documentation as a guide to customize it. (If you don't have the sample `.conf` file, you can download it [here][conf].)

```bash
sudo cp /etc/rethinkdb/default.conf.sample /etc/rethinkdb/instances.d/instance1.conf
sudo vim /etc/rethinkdb/instances.d/instance1.conf
```

While you may be able to leave many options at their defaults, you'll definitely need to change the `directory=` line in the configuration file to point to your RethinkDB data directory.

```
directory=/path/to/your/rethinkdb/directory
```

Then, enable the service:

```bash
sudo systemctl enable rethinkdb@<name_instance>
sudo systemctl start rethinkdb@<name_instance>
```

__You've now got a working server!__

## Multiple instances ##

As systemd supports multiple instances on the same server, you simply need to create multiple `.conf` files in `/etc/rethinkdb/instances.d`. This may be desirable for isolating databases for separate applications running on the same server, or for testing purposes. (There is no performance gain from running multiple nodes of the same cluster on the same physical machine.)

In each configuration file, set a different data directory, and include the `join` configuration option for each node with the IP address and port of another node in the cluster. If the instances are not running on the same machine, specify `bind=all` in the configuration file (or `--bind all` on the command line). Take care that each instance on the same machine specifies different values for `driver-port`, `cluster-port` and `http-port`.

{% infobox alert %}
The `bind=all` option is a security risk if your machine is open to the internet, and you should take steps to prevent unauthorized access. See the [security page](/docs/security/) for more details.
{% endinfobox %}

# Startup with launchd (OS X)

If you install RethinkDB using [Homebrew][], a `launchd` configuration file will be installed for you in `~/Library/LaunchAgents/`, although that file may need to be modified.

[Homebrew]: http://brew.sh

## Basic setup

If you didn't install using Homebrew, you'll need to create a launchd configuration file, and decide where you want to store your data files. These instructions assume the following locations:

* RethinkDB binary installed by the official package in `/usr/local/bin/rethinkdb`
* RethinkDB data directory will be `/Library/RethinkDB/data`
* RethinkDB log will be `/var/log/rethinkdb.log`

If you wish other locations, change the text in the file appropriately.

Create `/Library/LaunchDaemons/com.rethinkdb.server.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple Computer//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>com.rethinkdb.server</string>
  <key>ProgramArguments</key>
  <array>
      <string>/usr/local/bin/rethinkdb</string>
      <string>-d</string>
      <string>/Library/RethinkDB/data</string>
  </array>
  <key>StandardOutPath</key>
  <string>/var/log/rethinkdb.log</string>
  <key>StandardErrorPath</key>
  <string>/var/log/rethinkdb.log</string>
  <key>RunAtLoad</key>
  <true/>
  <key>KeepAlive</key>
  <true/>
  <key>LowPriorityIO</key>
  <false/>
</dict>
</plist>
```

Set this file to be owned by the `root` user:

```bash
sudo chown root:wheel /Library/LaunchDaemons/com.rethinkdb.server.plist
sudo chmod 644 /Library/LaunchDaemons/com.rethinkdb.server.plist
```

Then you'll need to create the RethinkDB data directory.

```bash
sudo mkdir -p /Library/RethinkDB
sudo rethinkdb create -d /Library/RethinkDB/data
```

## Using a RethinkDB configuration file

By default, neither Homebrew nor the example configuration file above will read options from a [configuration file](/docs/config-file). If you wish to use one, you'll need to do the following:

* Download the [sample configuration file][conf] and copy it to a new location.

```bash
cp default.conf.sample /etc/rethinkdb.conf
```

* Edit the configuration file in your favorite editor. While you may be able to leave many options at their defaults, you'll definitely need to change the `directory=` line in the file to point to your data directory.

```
sudo pico /etc/rethinkdb.conf
```

* Edit `/Library/LaunchDaemons/com.rethinkdb.server.plist` to change the `ProgramArguments` key so RethinkDB will use your configuration file.

```xml
<key>ProgramArguments</key>
<array>
    <string>/usr/local/bin/rethinkdb</string>
    <string>--config-file</string>
    <string>/etc/rethinkdb.conf</string>
</array>
```

## Starting RethinkDB instances

To start RethinkDB, use `launchctl`:

```bash
launchctl load /Library/LaunchDaemons/com.rethinkdb.server.plist
```

RethinkDB will automatically load on startup. To disable this behavior, change the `RunAtLoad` key to `<false/>` in the `plist` file.

## Multiple instances

Running multiple instances of RethinkDB on the same server may be desirable for isolating databases for separate applications running on the same server, or for testing purposes. (There is no performance gain from running multiple nodes of the same cluster on the same physical machine.)
 
You will need to create new copies of the `com.rethinkdb.server.plist` file with different names (e.g., `com.rethinkdb.server2.plist`), making the following changes:

* Set the `Label` key value to the name of the file (e.g., `com.rethinkdb.server2.plist`).
* Set the `ProgramArguments` key to a new configuration file (e.g., `/etc/rethinkdb2.conf`).
* Set the `StandardOutPath` and `StandardErrorPath` keys to a new log file.

In each configuration file, set a different data directory, and include the `join` configuration option for each node with the IP address and port of another node in the cluster. If the instances are not running on the same machine, specify `bind=all` in the configuration file (or `--bind all` on the command line). Take care that each instance on the same machine specifies different values for `driver-port`, `cluster-port` and `http-port`.

{% infobox alert %}
The `bind=all` option is a security risk if your machine is open to the internet, and you should take steps to prevent unauthorized access. See the [security page](/docs/security/) for more details.
{% endinfobox %}

# Troubleshooting #

{% infobox %}
<strong>Seeing a 'received invalid clustering header' message?</strong>
{% include docs/troubleshootingcluster.md %} 
{% endinfobox %}

