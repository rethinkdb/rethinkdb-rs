---
layout: documentation
title: Migrating data from previous versions of RethinkDB
active: docs
docs_active: migration
permalink: docs/migration/
---

You must migrate your data __before__ updating RethinkDB, since file formats are
currently incompatible between versions. Migration consists of three simple steps:

  - Export your data from the existing version of RethinkDB
  - Upgrade RethinkDB to a new version
  - Import the data into the new version of RethinkDB

If you have already updated to a new version of RethinkDB, you can find
binaries for previous versions on the [download
archive](http://download.rethinkdb.com).

{% infobox info %}
__Note:__ RethinkDB file formats are currently not compatible between versions.
A better migration experience is on the roadmap (follow [Github issue #1010](https://github.com/rethinkdb/rethinkdb/issues/1010) to track progress).
{% endinfobox %}

# Exporting your data

To export your data, use `rethinkdb dump`:

```
rethinkdb dump --connect <host>:<port> [--auth <auth_key>] 
```
where:

- `host` is the IP address of any machine of your RethinkDB cluster
- `port` is the port for driver connections (by default 28015)
- `auth_key` is an optional [authentication key](/docs/security) to connect to the cluster

This command will export all your data to a `tar.gz` file named
`rethinkdb_dump_<timestamp>.tar.gz` (this may vary depending on your platform).

Use `rethinkdb dump --help` to see the complete list of options for dumping
your data.

{% infobox info %}
__Exporting from a version before RethinkDB 1.7?__ Use the deprecated
[migration script](https://github.com/rethinkdb/rethinkdb/tree/next/scripts/migration).
{% endinfobox %}

# Upgrading RethinkDB

__First__, upgrade the RethinkDB server and drivers to the latest version:

- See [server install instructions](/install) for your platform.
- See [driver install instructions](/docs/install-drivers/) for your language.

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
- `host` is the IP address of any machine of your RethinkDB cluster
- `port` is the port for driver connections (by default 28015)
- `auth_key` is an optional [authentication key](/docs/security) to connect to the cluster

Use `rethinkdb restore --help` to see the complete list of options for importing your data.

# Limitations

The dump/restore command currently comes with some limitations:

- The cluster configuration cannot be exported. The cluster has to be manually
  reconfigured.
- Secondary indexes cannot be exported. You will have to manually recreate
  them.
