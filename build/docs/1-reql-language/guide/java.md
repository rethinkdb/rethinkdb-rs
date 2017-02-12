---
layout: documentation
title: Ten-minute guide with RethinkDB and Java
docs_active: guide
permalink: docs/guide/java/
alias: docs/guides/ten-min/
switcher: true
language : Java
---

{% infobox %}
__Before you start:__

* Make sure you've [installed RethinkDB](/install)&mdash;it should only take a minute!
* Make also sure you've [installed the Java driver](/docs/install-drivers/java/).
* Read the [thirty-second quickstart](/docs/quickstart/).
{% endinfobox %}

<img src="/assets/images/docs/api_illustrations/10-minute-guide_javascript.png" class="api_command_illustration" />

# Start the server #

For a more detailed look, make sure to read the [quickstart](/docs/quickstart/).

## Linux and OS X ##

Start the server from a terminal window.

    $ rethinkdb

## Windows ##

Start the server from the Windows command prompt.

    C:\Users\Slava\RethinkDB\>rethinkdb.exe

# Import the driver #

You can use the drivers from Java like this:

```java
import com.rethinkdb.RethinkDB;
import com.rethinkdb.gen.exc.ReqlError;
import com.rethinkdb.gen.exc.ReqlQueryLogicError;
import com.rethinkdb.model.MapObject;
import com.rethinkdb.net.Connection;

public static final RethinkDB r = RethinkDB.r;
```

You can now access RethinkDB commands through the `r` class.

# Open a connection #

When you first start RethinkDB, the server opens a port for the client
drivers (`28015` by default). Let's open a connection:

```java
Connection conn = r.connection().hostname("localhost").port(28015).connect();
```

The variable `conn` is now initialized and we can run queries.

# Create a new table #

By default, RethinkDB creates a database `test`. Let's create a table
`authors` within this database:

```java
r.db("test").tableCreate("authors").run(conn);
```

The result will be (in JSON):

```json
{
    "config_changes": [
        <table configuration data>
    ],
    "tables_created": 1
}
```

(The `config_changes` field contains metadata about the newly created table; for more details, read about the [tableCreate](/api/java/table_create/) command.) There are a couple of things you should note about this query:

* First, we select the database `test` with the `db` command.
* Then, we add the `tableCreate` command to create the actual table.
* Lastly, we call `run(conn)` in order to send the query to the server.

All ReQL queries follow this general structure. Now that we've created
a table, let's insert some data!

# Insert data #

Let's insert three new documents into the `authors` table:

```java
r.table("authors").insert(r.array(
    r.hashMap("name", "William Adama")
     .with("tv_show", "Battlestar Galactica")
     .with("posts", r.array(
        r.hashMap("title", "Decommissioning speech")
         .with("content", "The Cylon War is long over..."),
        r.hashMap("title", "We are at war")
         .with("content", "Moments ago, this ship received..."),
        r.hashMap("title", "The new Earth")
         .with("content", "The discoveries of the past few days...")
        )
    ),
    r.hashMap("name", "Laura Roslin")
     .with("tv_show", "Battlestar Galactica")
     .with("posts", r.array(
        r.hashMap("title", "The oath of office")
         .with("content", "I, Laura Roslin, ..."),
        r.hashMap("title", "They look like us")
         .with("content", "The Cylons have the ability...")
        )
    ),
    r.hashMap("name", "Jean-Luc Picard")
     .with("tv_show", "Star Trek TNG")
     .with("posts", r.array(
        r.hashMap("title", "Civil rights")
         .with("content", "There are some words I've known since...")
        )
    )
)).run(conn);
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
the query `r.table("authors")`:

```java
Cursor cursor = r.table("authors").run(conn);
for (Object doc : cursor) {
    System.out.println(doc);
}
```

The result is an array of the three previously inserted documents,
along with the generated `id` values.

Since the table might contain a large number of documents, the
database returns a cursor object. As you iterate through the cursor,
the server will send documents to the client in batches as they are
requested. We only have three documents in our example, so we can
safely retrieve all the documents at once. The `toList` function
automatically iterates through the cursor and puts the documents into
a Java list.

## Filter documents based on a condition ##

Let's try to retrieve the document where the `name` attribute is set
to `William Adama`.  We can use a condition to filter the documents by
chaining a `filter` command to the end of the query:

```java
Cursor cursor = r.table("authors").filter(row -> row.g("name").eq("William Adama")).run(conn);
for (Object doc : cursor) {
    System.out.println(doc);
}
```

This query returns a cursor with one document&mdash;the record for
William Adama. The `filter` command evaluates the provided condition
for every row in the table, and returns only the relevant rows. Here's
the new commands we used to construct the condition above:

- `row` refers to the currently visited document.
- `row.g("name")` refers to the value of the field `name` of the visited
  document.
- The `eq` command returns `true` if two values are equal (in this case, the field `name` and the  string `William Adama`).

Let's use `filter` again to retrieve all authors who have more than
two posts:

```java
Cursor cursor = r.table("authors").filter(row -> row.g("posts").count().gt(2)).run(conn);
for (Object doc : cursor) {
    System.out.println(doc);
}
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

```java
r.db("test").table("authors").get("7644aaf2-9928-4231-aa68-4e65e31bf219").run(conn);
```

Since primary keys are unique, the `get` command returns a single
document. This way we can retrieve the document directly without
converting a cursor to an array.

{% infobox %}
Learn more about how RethinkDB can efficiently retrieve documents with
[secondary indexes](/docs/secondary-indexes/).
{% endinfobox %}

# Realtime feeds #

{% infobox %}
Feel free to skip this section if you don't want to learn about
realtime feeds yet. You can always go back and start a feed later.
{% endinfobox %}

RethinkDB inverts the traditional database architecture by exposing an
exciting new access model -- instead of polling for changes, the
developer can tell RethinkDB to continuously push updated query
results to applications in realtime.

To start a feed, compile and run the following query in a terminal window:

```java
import com.rethinkdb.RethinkDB;
import com.rethinkdb.net.Connection;
import com.rethinkdb.net.Cursor;

public static final RethinkDB r = RethinkDB.r;

Connection conn = r.connection().hostname("localhost").port(28015).connect();

Cursor cursor = r.table("authors").changes().run(conn);
for (Object doc : cursor) {
    System.out.println(doc);
}
```
The code above will start with the following messages:

```json
{
  "new_val": {
    "id": "1d854219-85c6-4e6c-8259-dbda0ab386d4",
    "name": "Laura Roslin",
    "posts": [...],
    "tv_show": "Battlestar Galactica",
    "type": "fictional"
  },
  "old_val": {
    "id": "1d854219-85c6-4e6c-8259-dbda0ab386d4",
    "name": "Laura Roslin",
    "posts": [...],
    "tv_show": "Battlestar Galactica"
  }
}
```

RethinkDB will notify your program of all changes in the `authors`
table and will include the old value and the new value of each
modified document. See the [changefeeds][] documentation entry for
more details on how to use realtime feeds in RethinkDB.

[changefeeds]: /docs/changefeeds

We'll be updating and deleting some documents in the next two sections.
To do this in realtime, we're going to use JavaScript in the
[Data Explorer][de]. Open the RethinkDB Administration Console in your
web browser and click "Data Explorer."

[de]: /docs/reql-data-exploration/

# Update documents #

Let's update all documents in the `authors` table and add a `type`
field to note that every author so far is fictional:

```java
r.table("authors").update(r.hashMap("type", "fictional")).run(conn);
```

Since we changed three documents, the result should look like this:

```json
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

```java
r.table("authors").filter(
    row -> row.g("name").eq("William Adama")
).update(r.hashMap("rank", "Admiral")).run(conn);
```

Since we only updated one document, we get back this object:

```json
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

```java
r.table("authors").filter(
    row -> row.g("name").eq("Jean-Luc Picard")
).update(
    r.hashMap("posts", row -> row.g("posts")
     .append(r.hashMap("title", "Shakespeare")
     .with("content", "What a piece of work is man...")))
).run();
```

After processing this query, RethinkDB will add an additional post to
Jean-Luc Picard's document.

{% infobox %}
Browse the [API reference](/api/java/) for many more array operations available in RethinkDB.
{% endinfobox %}

# Delete documents #

Suppose we'd like to trim down our database and delete every document
with less than three posts (sorry Laura and Jean-Luc):

```java
r.table("authors").filter(row -> row.g("posts").count().lt(3)).delete().run(conn);
```

Since we have two authors with less than two posts, the result
is:

```json
{
    "unchanged": 0,
    "skipped": 0,
    "replaced": 0,
    "inserted": 0,
    "errors": 0,
    "deleted": 2
}
```

{% include docs/quickstart-footer.md %}
