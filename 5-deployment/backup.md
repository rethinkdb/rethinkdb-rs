---
layout: documentation
title: Backing up your data
docs_active: backup
permalink: docs/backup/
---

The RethinkDB command line utility allows you to easily take hot backups on a live cluster with the `dump` and `restore` subcommands. The utility runs under the `admin` user account (see [Permissions and user accounts][pua]).

[pua]: /docs/permissions-and-accounts

# Backup

Use the `dump` subcommand to create an archive of data from the cluster. This creates a **tar.gz** file consisting of JSON documents and additional table metadata.

    rethinkdb dump [options]

Options to `dump` let you specify cluster information and limit the archive to specific databases or tables.

* `-c`, `--connect`: host and client port of the node to connect to (default: `localhost:28015`)
* `-f`, `--file`: specify a filename for the archive (default: `rethinkdb_dump_<date>_<time>.tar.gz`)
* `-e`, `--export`: limit the dump to the given database or table (specified as `database.table`); may be specified multiple times for multiple databases/tables
* `-p`, `--password`: prompt for the admin password, if one has been set
* `--password-file`: read the admin password from a plain text file
* `--tls-cert`: specify a path to a TLS certificate to allow encrypted connections to the server (see [Securing the cluster][sec])
* `--clients`: number of tables to export simultaneously (default: `3`)
* `--temp-dir`: directory to use for intermediary results
* `-h`, `--help`: print help

[sec]: /docs/security/

Since the backup process uses client drivers, it takes advantage of RethinkDB's concurrency. While it will use some cluster resources, it won't lock out any clients, and it can be safely run on a live cluster.

## Examples

    rethinkdb dump -c fortress:39500

Connect to the cluster at host `fortress` with a client port at `39500`, saving to the default archive name.

    rethinkdb dump -e league.users -f backup.tar.gz --password-file pw.txt

Connect to the default cluster (`localhost:28015`) and archive the `users` table from the `league` database in `backup.tar.gz`. Read the `admin` user password from the file `pw.txt`.

{% infobox alert %}
**Note:** The `dump` command saves database and table contents and metadata, but does **not** save cluster configuration data.
{% endinfobox %}

# Restore #

The `restore` subcommand has most of the the same options and defaults as the `dump` command, although there are a few extra commands for controlling how data is imported.

    rethinkdb restore filename

(Note that you must specify the archive to restore from; there is no default.)

* `-c`, `--connect`: host and client port of the node to connect to (default: `localhost:28015`)
* `-p`, `--password`: prompt for the admin password, if one has been set
* `--password-file`: read the admin password from a plain text file
* `--tls-cert`: specify a path to a TLS certificate to allow encrypted connections to the server (see [Securing the cluster][sec])
* `-i`, `--import`: limit the restore to the given database or table (specified as `database.table`); may be specified multiple times for multiple databases/tables
* `--clients`: number of client connections to use (default: `8`)
* `--temp-dir`: directory to use for intermediary results
* `--hard-durability`: use hard durability writes (slower, but less memory consumption on the server)
* `--force`: import data even if a table already exists
* `--no-secondary-indexes`: do not create secondary indexes for the restored tables
* `-h`, `--help`: print help

## Examples

    rethinkdb restore rethinkdb_dump_2015-09-17T10:59:58.tar.gz

Restore to the default cluster (`localhost:28015`).

    rethinkdb restore backup.tar.gz -c fortress:39500

Restore `backup.tar.gz` to the cluster running on `fortress` at port `39500`.

    rethinkdb restore backup.tar.gz -i league.users --password-file pw.txt

Restore to the default cluster, only importing the table `users` to the database `league` from the archive `backup.tar.gz`. Read the `admin` user password from the file `pw.txt`. (This should be a plain text file with the password on the first and only line.)
