---
layout: documentation
title: Migrating data from previous versions of RethinkDB
docs_active: migration
permalink: docs/migration/
---

Starting with version **1.13** of RethinkDB, manual migration of data when upgrading between major versions (1.x &rarr; 1.y) is no longer necessary. Data is migrated to the new version automatically. While it's not strictly necessary to back up your data before upgrading, it's always a good idea. You should make a backup by using the `dump` command *before* updating RethinkDB to its new version! Databases that have been automatically upgraded are not backward-compatible (that is, a database from version 1.14 cannot be used with version 1.13).

{% infobox %}
If you are upgrading from version 1.13 to a later version, you will not need to export and re-import your data. If you are upgrading from version 1.12.x or earlier to a new major version, you will need to follow these instructions.

If you're upgrading from a version before RethinkDB **1.7,** you'll need to use the deprecated [migration script](https://github.com/rethinkdb/rethinkdb/tree/02b4f29e1e7f15b3edffcb68bf015578ec5783ab/scripts/migration).
{% endinfobox %}

Migration consists of three simple steps:

  - Export your data from the existing version of RethinkDB
  - Upgrade RethinkDB to a new version
  - Import the data into the new version of RethinkDB

{% infobox alert %}
You must export your data **before** you've upgraded RethinkDB to a new version! If you've already updated, you can find binaries for previous versions in the [download archive](http://download.rethinkdb.com).
{% endinfobox %}

# Exporting your data

To export your data, use `rethinkdb dump`:

```
rethinkdb dump --connect <host>:<port> [--auth <auth_key>] 
```

where:

- `host` is the IP address of any server of your RethinkDB cluster
- `port` is the port for driver connections (by default 28015)
- `auth_key` is an optional [authentication key](/docs/security) to connect to the cluster

This command will export all your data to a `tar.gz` file named
`rethinkdb_dump_<timestamp>.tar.gz` (this may vary depending on your platform).

Use `rethinkdb dump --help` to see the complete list of options for dumping
your data.

{% infobox alert %}

__Note:__ The `dump` command requires the [Python driver](/docs/install-drivers/python/) to be installed. Don't upgrade the Python driver until *after* you've dumped the data!

If you don't have the Python driver installed, you can install the previous version using `pip install rethinkdb==1.12.0-2`. (You can use the [Python Package Index](https://pypi.python.org/pypi "PyPI") to check on current and older versions.)
{% endinfobox %}

# Upgrading RethinkDB

__First__, upgrade the RethinkDB server and drivers to the latest version:

- See [server install instructions](/install) for your platform.
- See [driver install instructions](/docs/install-drivers/) for your language.

{% infobox %}
__Keep in mind__: 

- Don't upgrade the Python driver until *after* you've dumped the data.
- [Upgrade](/docs/install-drivers/python/) the Python driver for the `restore` step below. 
{% endinfobox %}

__Then__, make sure to move or delete the old RethinkDB data directory
(`rethinkdb_data` by default), since the new version will not be able
to read the old file.

# Importing your data

To import your data, use `rethinkdb restore`:

```
rethinkdb restore <exported_file> --connect <host>:<port> [--auth <auth_key>] 
```

where:

- `exported_file` is the data file exported with `rethinkdb dump`: by default named `rethinkdb_dump_<timestamp>.tar.gz` (this may vary depending on your platform)
- `host` is the IP address of any server of your RethinkDB cluster
- `port` is the port for driver connections (by default 28015)
- `auth_key` is an optional [authentication key](/docs/security) to connect to the cluster

Use `rethinkdb restore --help` to see the complete list of options for importing your data.

# Limitations

The dump/restore command currently comes with some limitations:

- The cluster configuration cannot be exported. The cluster has to be manually
  reconfigured.
- Secondary indexes cannot be exported. You will have to manually recreate
  them.
