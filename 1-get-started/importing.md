---
layout: documentation
title: Importing your data
active: docs
docs_active: importing
permalink: docs/importing/
---

The `rethinkdb` utility includes an `import` command to load existing data into RethinkD databases.

    rethinkdb import -f FILE --table DB.TABLE [-c HOST:PORT] [-a AUTH_KEY]
      [--force] [--clients NUM] [--format (csv | json)] [--pkey PRIMARY_KEY]
      [--delimiter CHARACTER] [--custom-header FIELD,FIELD... [--no-header]]

RethinkDB can import from either JSON files or CSV files. To import the file `users.json` into the table `test.users`, for example, you would use:

    rethinkdb import -f users.json --table test.users

If it were a CSV file, you would use:

    rethinkdb import -f users.csv --format csv --table test.users

By default, the import command will connect to `localhost` port `28015`. You can use the `-c` option to specify a machine and client port to connect to. (Note this is the driver port clients connect to, not the cluster port.)

    rethinkdb import -f crew.json --table discovery.crew -c hal:2001

If the cluster requires authorization, you can specify the authorization key with the `-a` option.

    rethinkdb import -f crew.json --table discovery.crew -c hal:2001 -a daisy

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

There are more options than what we've covered here. Run `rethinkdb help import` for a full list of parameters and examples.

{% infobox info %}

While `import` has the ability to import a directory full of files, those files are expected to be in the format and directory structure created by the `export` command.

{% endinfobox %}
