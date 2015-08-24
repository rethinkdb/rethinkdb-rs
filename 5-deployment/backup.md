---
layout: documentation
title: Back up your data
docs_active: backup
permalink: docs/backup/
---

RethinkDB ships with `dump` and `restore` commands that allow easily
doing hot backups on a live cluster. The dump and restore commands
operate on tar.gz archives of JSON documents (along with additional
table metadata). You can run `rethinkdb dump --help` and `rethinkdb
restore --help` for more information.

# Backup #

Back up your data as follows:

```
# Dump the data from a RethinkDB cluster (placed in a file
# rethinkdb_dump_DATE_TIME.tar.gz by default)
$ rethinkdb dump -c HOST:PORT
```

Since the backup process is using client drivers, it automatically
takes advantage of the MVCC functionality built into RethinkDB. It
will use some cluster resources, but will not lock out any of the
clients, so you can safely run it on a live cluster.

# Restore #

You can reimport the backup into a running cluster as follows:

```
# Reimport an earlier dump
$ rethinkdb restore -c HOST:PORT rethinkdb_dump_DATE_TIME.tar.gz
```


