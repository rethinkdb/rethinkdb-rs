---
layout: documentation
title: Importing your data
docs_active: importing
permalink: docs/importing/
---

<img alt="Importing Data Illustration" class="api_command_illustration"
    src="/assets/images/docs/api_illustrations/importing_data.png" />

The `rethinkdb` utility includes an `import` command to load existing data into RethinkDB databases. It can read JSON files, organized in one of two formats described below, or comma-separated value (CSV) files (including ones with other delimiters such as tab characters). The utility runs under the `admin` user account (see [Permissions and user accounts][pua]).

[pua]: /docs/permissions-and-accounts

When the option is available, you should choose the JSON file format. If you're exporting from a SQL database this might not be possible, but you might be able to write a separate script to transform CSV output into JSON, or use the `mysql2json` script available as part of [mysql2xxxx][m2x].

[m2x]: https://github.com/seamusabshere/mysql2xxxx

The full syntax for the `import` command is as follows:

_Import from a directory_

    rethinkdb import -d DIR [-c HOST:PORT] [--force] [-p]
      [--password-file FILE] [-i (DB | DB.TABLE)] [--clients NUM]
      [--shards NUM_SHARDS] [--replicas NUM_REPLICAS]

_Import from a file_

    rethinkdb import -f FILE --table DB.TABLE [-c HOST:PORT] [--force]
      [-p] [--password-file FILE] [--clients NUM] [--format (csv | json)]
      [--pkey PRIMARY_KEY] [--shards NUM_SHARDS] [--replicas NUM_REPLICAS]
      [--delimiter CHARACTER] [--custom-header FIELD,FIELD... [--no-header]]

Importing from a directory is only supported for directories created by the `rethinkdb export` command.

Options for imports include:

* `-f`: file to import from
* `--table`: table to import to
* `--format`: CSV or JSON (default JSON)
* `-c`: connect to the given IP address/host and port
* `-p`, `--password`: prompt for the admin password, if one has been set
* `--password-file`: read the admin password from a plain text file
* `--tls-cert`: specify a path to a TLS certificate to allow encrypted connections to the server (see [Securing the cluster][sec])
* `--clients`: the number of client connections to use at once (default 8)
* `--force`: import data even if a table already exists
* `--fields`: only import from the listed fields
* `--no-header`: indicate the first line of a CSV file is _not_ a header row
* `--custom-header`: supply a custom header row for a CSV file

[sec]: /docs/security/

(Some of these options only apply to file imports, and there are other options available. Type `rethinkdb help import` for a full list.)

To import the file `users.json` into the table `test.users`, you would use:

    rethinkdb import -f users.json --table test.users

If it were a CSV file, you would use:

    rethinkdb import -f users.csv --format csv --table test.users

By default, the import command will connect to `localhost` port `28015`. You can use the `-c` option to specify a server and client port to connect to. (Note this is the driver port clients connect to, not the cluster port.)

    rethinkdb import -f crew.json --table discovery.crew -c hal:2001

If the cluster requires authorization, you can prompt for the `admin` user account password with `-p`, or supply a `--password-file` to read the password from. (The password file is just a plain text file, with the password on the first and only line.)

    rethinkdb import -f crew.json --table discovery.crew -c hal:2001 -p

A primary key other than `id` can be specified with `--pkey`:

    rethinkdb import -f heroes.json --table marvel.heroes --pkey name

JSON files are preferred to CSV files, as JSON can represent RethinkDB documents fully. If you're importing from a CSV file, you should include a header row with the field names, or use the `--no-header` option with the `--custom-header` option to specify the names.

    rethinkdb import -f users.csv --format csv --table test.users --no-header \
        --custom-header id,username,email,password

The CSV delimiter defaults to the comma, but this can be overridden with the `--delimiter` option. Use `--delimiter '\t'` for a tab-delimited file.

Values in CSV imports will always be imported as strings. If you want to convert those fields after import to the `number` data type, run an `update` query that does the conversion. An example runnable in the Data Explorer:

```js
r.table('tablename').update(function(doc) {
    return doc.merge({
        field1: doc('field1').coerceTo('number'),
        field2: doc('field2').coerceTo('number')
    })
});
```

RethinkDB will accept two formats for JSON files:

* An array of JSON documents.

    ```js
    [ { field: "value" }, { field: "value"}, ... ]
    ```

* Whitespace-separated JSON rows.

    ```js
    { field: "value" }
    { field: "value" }
    ```

In both cases, each documents is a JSON object, bracketed with `{ }` characters. Only the first format is itself a valid JSON document, but RethinkDB will import documents properly either way.

There are more options than what we've covered here. Run `rethinkdb help import` for a full list of parameters and examples.

{% infobox alert %}

While `import` has the ability to import a directory full of files, those files are expected to be in the format and directory structure created by the `export` command.

{% endinfobox %}
