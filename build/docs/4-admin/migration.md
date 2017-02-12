---
layout: documentation
title: Migrating data from previous versions of RethinkDB
docs_active: migration
permalink: docs/migration/
---

The steps necessary for migrating data to current RethinkDB versions from previous ones depend on which version you're migrating from.

{% toctag %}

* **1.16 or higher:** Migration is handled automatically. (This is also true for upgrading from 1.14 onward to versions earlier than 2.2.) After migration, follow the "Rebuild indexes" directions.
* **1.13&ndash;1.15:** Upgrade to RethinkDB 2.0.5 *first,* rebuild the secondary indexes by following the "Rebuild indexes" directions, then upgrade to 2.1 or higher. (Migration from 2.0.5 to 2.1+ will be handled automatically.)
* **1.7&ndash;1.12:** Follow the "Migrating old data" directions.
* **1.6 or earlier:** Read the "Deprecated versions" section.

# Back up your data

While it's not strictly necessary to back up your data before upgrading, it's always a good idea. You should make a backup by using the `dump` command *before* updating RethinkDB to its new version! Databases that have been automatically upgraded are not backward-compatible (that is, a database from version 1.14 cannot be used with version 1.13).

Use the `dump` subcommand from the command line to create an archive of data from the cluster. This creates a **tar.gz** file consisting of JSON documents and additional table metadata.

    rethinkdb dump [options]

The `restore` subcommand will reload a backup an archive into your cluster.

    rethinkdb restore filename

Use `rethinkdb help <command>` for a list of options. For more details, see [Backing up your data][backup].

[backup]: /docs/backup/

{% infobox alert %}
__Note:__ The `dump` and `restore` commands require the [Python driver](/docs/install-drivers/python/) to be installed. Don't upgrade the Python driver until *after* you've dumped the data!

If you don't have the Python driver installed, you can install a previous version using `pip install rethinkdb==<version>`. (You can use the [Python Package Index](https://pypi.python.org/pypi/rethinkdb "PyPI > rethinkdb") to check on current and older versions.)
{% endinfobox %}

# Rebuild indexes

When you upgrade a major release (i.e., 2.1 to 2.2), you should rebuild outdated secondary indexes manually. This can be done easily from the command line:

    rethinkdb index-rebuild

This is *required* if you're upgrading from versions before 1.16; in those cases, you'll need to upgrade to version 2.0.5 first. (You can download 2.0.5 and other older versions at RethinkDB's [download archive](http://download.rethinkdb.com)). If you're upgrading from RethinkDB version 1.16 or later, you can move to 2.2 or higher directly.

Note that rebuilding indexes is *not* required if you're upgrading between minor releases (i.e., 2.2.0 to 2.2.1).

# Migrating old data

*These steps are only necessary if you're upgrading from RethinkDB version 1.7&ndash;1.12 to version 2.1 or higher.* If you're using a later version, see the sections above. (If you're using 1.6 or earlier, read the "[Deprecated versions](#deprecated-versions)" section below.)

Migration consists of three simple steps:

  - Export your data from the existing version of RethinkDB
  - Upgrade RethinkDB to a new version
  - Import the data into the new version of RethinkDB

{% infobox alert %}
You must export your data **before** you've upgraded RethinkDB to a new version! If you've already updated, you can find binaries for previous versions in the [download archive](http://download.rethinkdb.com).
{% endinfobox %}

## Exporting your data

To export your data, use `rethinkdb dump`:

    rethinkdb dump -c <host>:<port>

This command will export all your data to a `tar.gz` file named `rethinkdb_dump_<timestamp>.tar.gz` (this may vary depending on your platform).

Use `rethinkdb help <command>` for a list of options. For more details, see [Backing up your data][backup].

## Upgrading RethinkDB

First, upgrade the RethinkDB server and drivers to the latest version:

- See [server install instructions](/install) for your platform.
- See [driver install instructions](/docs/install-drivers/) for your language.

{% infobox %}
__Keep in mind__: 

- Don't upgrade the Python driver until *after* you've dumped the data.
- [Upgrade](/docs/install-drivers/python/) the Python driver for the `restore` step below. 
{% endinfobox %}

Then make sure to move or delete the old RethinkDB data directory
(`rethinkdb_data` by default), since the new version will not be able
to read the old file.

## Importing your data

To import your data, use `rethinkdb restore`:

    rethinkdb restore <exported_file> -c <host>:<port>

Use `rethinkdb restore --help` to see the complete list of options for importing your data. Again, for more details, see [Backing up your data][backup].

After importing your data, you'll need to rebuild your secondary indexes.

    rethinkdb index-rebuild

{% infobox alert %}
The cluster configuration is *not* exported in backup. After a full restore, it will need to be manually reconfigured.
{% endinfobox %}

# Deprecated versions

Upgrading from RethinkDB versions 1.6 or earlier has *not* been tested with RethinkDB 2.1 and higher. However, you should be able to use the deprecated [migration script][ms].

[ms]: https://github.com/rethinkdb/rethinkdb/tree/02b4f29e1e7f15b3edffcb68bf015578ec5783ab/scripts/migration

Follow the directions in the README file to perform the migration.
