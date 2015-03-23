---
layout: documentation
title: Ten-minute guide with RethinkDB and JavaScript
docs_active: guide
permalink: docs/guide/javascript/
alias: docs/guides/ten-min/
switcher: true
language : JavaScript
---
{% infobox %}
__Before you start:__

* Make sure you've [installed RethinkDB](/install)&mdash;it should only take a minute!
* Make also sure you've [installed the JavaScript driver](/docs/install-drivers/javascript/).
* Read the [thirty-second quickstart](/docs/quickstart/).
{% endinfobox %}

<img src="/assets/images/docs/api_illustrations/10-minute-guide_javascript.png" class="api_command_illustration" />

# Import the driver #

First, start Node.js:

```bash
$ node
```

Then, import the RethinkDB driver:

```javascript
r = require('rethinkdb');
```

You can now access RethinkDB commands through the `r` module.

# Open a connection #

When you first start RethinkDB, the server opens a port for the client
drivers (`28015` by default). Let's open a connection:

```javascript
var connection = null;
r.connect( {host: 'localhost', port: 28015}, function(err, conn) {
    if (err) throw err;
    connection = conn;
})
```

The variable `connection` is now initialized and we can run queries.

# Create a new table #

By default, RethinkDB creates a database `test`. Let's create a table
`authors` within this database:

```javascript
r.db('test').tableCreate('authors').run(connection, function(err, result) {
    if (err) throw err;
    console.log(JSON.stringify(result, null, 2));
})
```

The result will be:

```json
{
    "config_changes": [
        <table configuration data>
    ],
    "tables_created": 1
}
```

(The `config_changes` field contains metadata about the newly created table; for more details, read about the [tableCreate](/api/javascript/table_create/) command.) There are a couple of things you should note about this query:

* First, we select the database `test` with the `db` command.
* Then, we add the `tableCreate` command to create the actual table.
* Lastly, we call `run(connection, callback)` in order to send the query to the server.

All ReQL queries follow this general structure. Now that we've created
a table, let's insert some data!

# Insert data #

Let's insert three new documents into the `authors` table:

```javascript
r.table('authors').insert([
    { name: "William Adama", tv_show: "Battlestar Galactica",
      posts: [
        {title: "Decommissioning speech", content: "The Cylon War is long over..."},
        {title: "We are at war", content: "Moments ago, this ship received word..."},
        {title: "The new Earth", content: "The discoveries of the past few days..."}
      ]
    },
    { name: "Laura Roslin", tv_show: "Battlestar Galactica",
      posts: [
        {title: "The oath of office", content: "I, Laura Roslin, ..."},
        {title: "They look like us", content: "The Cylons have the ability..."}
      ]
    },
    { name: "Jean-Luc Picard", tv_show: "Star Trek TNG",
      posts: [
        {title: "Civil rights", content: "There are some words I've known since..."}
      ]
    }
]).run(connection, function(err, result) {
    if (err) throw err;
    console.log(JSON.stringify(result, null, 2));
})
```

We should get back an object that looks like this:

```json
{
    "unchanged": 0,
    "skipped": 0,
    "replaced": 0,
    "inserted": 3,
    "generated_keys": [
        "7644aaf2-9928-4231-aa68-4e65e31bf219",
        "064058b6-cea9-4117-b92d-c911027a725a",
        "543ad9c8-1744-4001-bb5e-450b2565d02c"
    ],
    "errors": 0,
    "deleted": 0
}
```

The server should return an object with zero errors and three inserted
documents. We didn't specify any primary keys (by default, each table
uses the `id` attribute for primary keys), so RethinkDB generated them
for us. The generated keys are returned via the `generated_keys`
attribute.

There are a couple of things to note about this query:

* Each connection sets a default database to use during its lifetime
  (if you don't specify one in `connect`, the default database is set
  to `test`). This way we can omit the `db('test')` command in our
  query. We won't specify the database explicitly from now on, but if
  you want to prepend your queries with the `db` command, it won't
  hurt.
* The `insert` command accepts a single document or an array of
  documents if you want to batch inserts. We use an array in this
  query instead of running three separate `insert` commands for each
  document.

# Retrieve documents #

Now that we inserted some data, let's see how we can query the
database!

## All documents in a table ##

To retrieve all documents from the table `authors`, we can simply run
the query `r.table('authors')`:

```javascript
r.table('authors').run(connection, function(err, cursor) {
    if (err) throw err;
    cursor.toArray(function(err, result) {
        if (err) throw err;
        console.log(JSON.stringify(result, null, 2));
    });
});
```

The result is an array of the three previously inserted documents,
along with the generated `id` values.

Since the table might contain a large number of documents, the
database returns a cursor object. As you iterate through the cursor,
the server will send documents to the client in batches as they are
requested. We only have three documents in our example, so we can
safely retrieve all the documents at once. The `toArray` function
automatically iterates through the cursor and puts the documents into
a JavaScript array.

## Filter documents based on a condition ##

Let's try to retrieve the document where the `name` attribute is set
to `William Adama`.  We can use a condition to filter the documents by
chaining a `filter` command to the end of the query:

```javascript
r.table('authors').filter(r.row('name').eq("William Adama")).
    run(connection, function(err, cursor) {
        if (err) throw err;
        cursor.toArray(function(err, result) {
            if (err) throw err;
            console.log(JSON.stringify(result, null, 2));
        });
    });
```

This query returns a cursor with one document&mdash;the record for
William Adama. The `filter` command evaluates the provided condition
for every row in the table, and returns only the relevant rows. Here's
the new commands we used to construct the condition above:

- `r.row` refers to the currently visited document.
- `r.row('name')` refers to the value of the field `name` of the visited
  document.
- The `eq` command returns `true` if two values are equal (in this case, the field `name` and the  string `William Adama`).

Let's use `filter` again to retrieve all authors who have more than
two posts:

```javascript
r.table('authors').filter(r.row('posts').count().gt(2)).
    run(connection, function(err, cursor) {
        if (err) throw err;
        cursor.toArray(function(err, result) {
            if (err) throw err;
            console.log(JSON.stringify(result, null, 2));
        });
    });
```

In this case, we're using a predicate that returns `true` only if the
length of the array in the field `posts` is greater than two. This
predicate contains two commands we haven't seen before:

- The `count` command returns the size of the array.
- The `gt` command returns `true` if a value is greater than the
  specified value (in this case, if the number of posts is greater than two).

## Retrieve documents by primary key ##

We can also efficiently retrieve documents by their primary key using
the `get` command. We can use one of the ids generated in the
previous example:

```javascript
r.table('authors').get('7644aaf2-9928-4231-aa68-4e65e31bf219').
    run(connection, function(err, result) {
        if (err) throw err;
        console.log(JSON.stringify(result, null, 2));
    });
```

Since primary keys are unique, the `get` command returns a single
document. This way we can retrieve the document directly without
converting a cursor to an array.

{% infobox %}
Learn more about how RethinkDB can efficiently retrieve documents with
[secondary indexes](/docs/secondary-indexes/).
{% endinfobox %}

# Update documents #

Let's update all documents in the `authors` table and add a `type`
field to note that every author so far is fictional:

```javascript
r.table('authors').update({type: "fictional"}).
    run(connection, function(err, result) {
        if (err) throw err;
        console.log(JSON.stringify(result, null, 2));
    });
```

Since we changed three documents, the result should look like this:

```javascript
{
    "unchanged": 0,
    "skipped": 0,
    "replaced": 3,
    "inserted": 0,
    "errors": 0,
    "deleted":0
}
```

Note that we first selected every author in the table, and then
chained the `update` command to the end of the query. We could also
update a subset of documents by filtering the table first. Let's
update William Adama's record to note that he has the rank of Admiral:

```javascript
r.table('authors').
    filter(r.row("name").eq("William Adama")).
    update({rank: "Admiral"}).
    run(connection, function(err, result) {
        if (err) throw err;
        console.log(JSON.stringify(result, null, 2));
    });
```

Since we only updated one document, we get back this object:

```javascript
{
    "unchanged": 0,
    "skipped": 0,
    "replaced": 1,
    "inserted": 0,
    "errors": 0,
    "deleted": 0
}
```

The `update` command allows changing existing fields in the document,
as well as values inside of arrays. Let's suppose Star Trek
archaeologists unearthed a new speech by Jean-Luc Picard that we'd like
to add to his posts:

```javascript
r.table('authors').filter(r.row("name").eq("Jean-Luc Picard")).
    update({posts: r.row("posts").append({
        title: "Shakespeare",
        content: "What a piece of work is man..."})
    }).run(connection, function(err, result) {
        if (err) throw err;
        console.log(JSON.stringify(result, null, 2));
    });
```

After processing this query, RethinkDB will add an additional post to
Jean-Luc Picard's document.

{% infobox %}
Browse the [API reference](/api/javascript/) for many more array operations available in RethinkDB.
{% endinfobox %}

# Delete documents #

Suppose we'd like to trim down our database and delete every document
with less than three posts (sorry Laura and Jean-Luc):

```javascript
r.table('authors').
    filter(r.row('posts').count().lt(3)).
    delete().
    run(connection, function(err, result) {
        if (err) throw err;
        console.log(JSON.stringify(result, null, 2));
    });
```

Since we have two authors with less than two posts, the result
is:

```javascript
{
    "unchanged": 0,
    "skipped": 0,
    "replaced": 0,
    "inserted": 0,
    "errors": 0,
    "deleted": 2
}
```

{% include quickstart-footer.md %} 
