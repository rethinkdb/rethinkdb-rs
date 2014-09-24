---
layout: documentation
title: Cookbook for JavaScript
active: docs
docs_active: cookbook
js: [faq_index, fancybox]
permalink: docs/cookbook/javascript/
switcher: true
language : JavaScript
---
{% include recipe-forms.html %}

<img src="/assets/images/docs/api_illustrations/cookbook.png" class="api_command_illustration" />

<div id="faqcontents"></div>
---
{% faqsection Basic commands %}

## Creating a database ##

You can use the `dbCreate` command as follows:

```javascript
r.dbCreate("blog").run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

Another way to create a database is through the web UI. You can reach
the web UI at `http://HOST:8080`. Click on the _Tables_ tab at the top
and then use the _Add Database_ button.

## Creating a table ##

You can select the database where you'd like to create the table with
the `db` command and use the `tableCreate` command as follows:

```javascript
r.db("blog").tableCreate("posts").run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

Note that you can omit the `db` command if you're creating a table in
the default database on your connection (set to `test` unless
specified in `connect`).

Another way to create a new table is to use the web UI. You can reach
the web UI at `http://HOST:8080`. Click on the _Tables_ tab at the top
of the page and then use the _Add Table_ button.

## Inserting documents ##

You can insert documents by calling the `insert` command on the
appropriate table:

```javascript
r.table("user").insert({
    name: "Michel",
    age: 26
}).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

You can insert multiple documents at once by passing an array of
documents to `insert` as follows:

```javascript
r.table("user").insert([
    {
        name: "Michel",
        age: 26
    },
    {
        name: "Slava",
        age: 30
    }
]).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

## Deleting documents ##

To delete documents, select the documents you'd like to delete and use
the `delete` command. For example, let's delete all posts with the
author "Michel":

```javascript
r.table("posts").filter(r.row("author").eq("Michel")).delete().run(conn,
    function(err, result) {
        if (err) throw err;
        console.log(result);
    }
);
```

Or, let's try to delete a single user:

```javascript
r.table("posts").get("7644aaf2-9928-4231-aa68-4e65e31bf219").delete().run(conn,
    function(err, result) {
        if (err) throw err;
        console.log(result);
    }
);
```

Here is how we'd delete all documents in a table:

```javascript
r.table("posts").delete().run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

{% endfaqsection %}

{% faqsection Filtering %}

## Filtering based on multiple fields ##

Suppose you'd like to select all posts where the author's name is
"Michel" and the category is "Geek". You can do it as follows:

```javascript
r.table("posts").filter({
    author: "Michel",
    category: "Geek",
}).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

Alternatively, you can build a predicate with the `and` command, and
pass it to `filter`:

```javascript
r.table("posts").filter(
    r.row("author").eq("Michel").and(r.row("category").eq("Geek"))
).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

You can also use the prefix notation (passing all arguments to
`r.and`), if that's what you prefer:

```javascript
r.table("posts").filter(
    r.and(r.row("author").eq("Michel"), r.row("category").eq("Geek"))
).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

Similarly, you can use the `r.or` command to filter based on one of
many conditions.

## Filtering based on the presence of a value in an array ##

Suppose we have a table `users` with documents of the following form:

```json
{
    name: "William Adama"
    emails: ["bill@bsg.com", "william@bsg.com"],
    ship: "Galactica"
}
```

If we want to retrieve all users that have the email address
`user@email.com`, we can write:

```javascript
r.table("user").filter(r.row("emails").contains("user@email.com"))
 .run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

If we want to retrieve all users on the Galactica and Pegasus, we can write:

```js
r.table("user").filter(function (user) {
    r(["Galactica", "Pegasus"]).contains(user("ship"))
}).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

## Filtering based on nested fields ##

In JavaScript you can use the operator `()` to get the value of a
field. This operator can be chained to retrieve values from nested
fields.

Suppose we have a table `users` with documents of the following form:

```json
{
    name: "William Adama"
    contact: {
        phone: "555-5555"
        email: "bill@bsg.com"
    }
}
```

Let's filter based on the nested field `email`:

```javascript
r.table("user").filter(
    r.row("contact")("email").eq("user@email.com")
).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

For many ReQL commands, you can also use a JSON-style nested syntax that allows
considerably more flexibility. Read "[Accessing nested fields](/docs/nested-fields)"
for more information.

## Efficiently retrieving multiple documents by primary key ##

If you want to retrieve all the posts with the primary keys `1`, `2`,
or `3` you can use the `getAll` command:

```javascript
r.table("posts").getAll(1, 2, 3).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

## Efficiently retrieving multiple documents by secondary index ##

Suppose we have a table `posts` that links posts to authors via an
`author_id` field. If we've created a secondary index on `author_id`
and want to retrieve all the posts where `author_id` is `1`, `2`, or
`3`, we can use the `getAll` command to do it as follows:

```javascript
r.table("posts").getAll(1, 2, 3, {index: 'author_id'})
 .run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

{% infobox info %}
Read about [creating secondary indexes in RethinkDB](/docs/secondary-indexes/).
{% endinfobox %}

## Retrieving all the objects in a stream (cursor) as an array ##

If you're using a command that returns a stream and want to retrieve all of
its results at once in an array rather than iterating through them with the
cursor object, you can coerce it to an array using the
[toArray](/api/javascript/to_array/) command.

```js
r.table('posts').run(conn, function(err, cursor) {
    if (err) throw err;
    cursor.toArray(function(result) {
        console.log(result);
    });
});
```

See the [data type documentation](/docs/data-types/) for more detail about
streams.

## Returning specific fields of a document ##

If you need to retrieve only a few specific fields from your
documents, you can use the `pluck` command. For example, here is how
you'd return only the fields `name` and `age` from each row in table
`users`:

```javascript
r.table("users").pluck("name", "age").run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

This is equivalent to calling `SELECT name, age FROM users` in SQL.

The `pluck` command also supports selecting nested fields in a
document. For example, suppose we'd like to select the fields `phone`
and `email` from the following document:

```json
{
    name: "William Adama"
    contact: {
        phone: "555-5555"
        email: "bill@bsg.com"
    }
}
```

We can use the following syntax:

```javascript
r.table("users").pluck(
    {contact: {phone: true, email: true}}
).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```


## Filtering based on a date range ##

Suppose you want to retrieve all the posts whose date field is
between January 1st, 2012 (included) and January 1st, 2013 (excluded). You could do:

```js
r.table("posts").filter(function(post) {
    return post("date").during(r.time(2012, 1, 1, 'Z'), r.time(2013, 1, 1, 'Z'));
}).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

You can also manually compare dates:

```js
r.table("posts").filter(function(post) {
    return post("date").ge(r.time(2012, 1, 1, 'Z')).and(
        post("date").lt(r.time(2013, 1, 1, 'Z')));
}).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```


## Filtering with Regex ##

If you want to retrieve all users whose last name starts with "Ma", 
you can use `r.match` this way:

```js
// Will return Martin, Martinez, Marshall etc.
r.table("users").filter(function(user) {
    return user("lastName").match("^Ma");
}).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

If you want to retrieve all users whose last name ends with an "s", 
you can use `r.match` this way:

```js
// Will return Williams, Jones, Davis etc.
r.table("users").filter(function(user) {
    return user("lastName").match("s$");
}).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

If you want to retrieve all users whose last name contains "ll", 
you can use `r.match` this way:

```js
// Will return Williams, Miller, Allen etc.
r.table("users").filter(function(user) {
    return user("lastName").match("ll");
}).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

## Case insensitive filter ##

Retrieve all users whose name is "William" (case insensitive).

```js
// Will return william, William, WILLIAM, wiLLiam etc.
r.table("users").filter(function(user) {
    return user("name").match("(?i)^william$");
}).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```


{% endfaqsection %}

{% faqsection Manipulating documents %}

## Adding/overwriting a field in a document ##

To add or overwrite a field, you can use the `update` command.  For
instance, if you would like to add the field `author` with the value
"Michel" for all of the documents in the table `posts`, you would use:

```javascript
r.table("posts").update({ author: "Michel" }).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

## Removing a field from a document ##

The `update` command lets you to overwrite fields, but not delete
them. If you want to delete a field, use the `replace` command. The
`replace` command replaces your entire document with the new document
you pass as an argument. For example, if you want to delete the field
`author` of the blog post with the id `1`, you would use:

```javascript
r.table("posts").get("1").replace(r.row.without('author'))
 .run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

## Atomically updating a document based on a condition ##

All modifications made via the `update` and `replace` commands are
always atomic with respect to a single document. For example, let's
say we'd like to atomically update a view count for a page if the
field `countable` is set to true, and get back the old and new results
in a single query. We can perform this operation as follows:

```javascript
r.table("pages").update(function(page) {
    return r.branch(page("countable").eq(true),  // if the page is countable
        { views: page("views").add(1) },         // increment the view count
        {}                                       // else do nothing
    );
}, {returnChanges: true}).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

## Storing timestamps and JSON date strings as Time data types ##

You can use the `epochTime` and `ISO8601` commands to convert Unix timestamps (in seconds) and JSON date-time strings (which are in ISO 8601 format) to the ReQL time type. The ReQL driver will also convert JavaScript Date objects into ReQL time.

```js
var theDate = new Date();
var timestamp = theDate.getTime();
var JSONDate = theDate.toJSON();
r.table("dates").insert({
    from_object: theDate,
    from_epoch: r.epochTime(timestamp/1000.0),
    from_iso: r.ISO8601(JSONDate)
}).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

Use the commands `toEpochTime` and `toISO8601` to convert back.

## Incrementing a field value ##

It's possible to increment a field value in a document&mdash;for example, a counter&mdash;in one step on the server.

```js
r.table('aggregated').get(id).update(
    { count: r.row('count').default(0).add(1) }
).run(conn, callback);
```

Use `default` to ensure that if the `count` field doesn't already exist in the document, it's added correctly, rather than letting `add` throw an error.

{% endfaqsection %}

{% faqsection Pagination %}

## Limiting the number of returned documents ##

You can limit the number of documents returned by your queries with
the `limit` command. Let's retrieve just the first 10 blog posts:

```javascript
r.table("posts").orderBy("date").limit(10).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

## Implementing pagination ##

To paginate results, you can use a combination of the `skip` and
`limit` commands. Let's retrieve posts 11-20 from our database:

```javascript
r.table("posts").orderBy("date").skip(10).limit(10).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

{% endfaqsection %}

{% faqsection Transformations %}

## Counting the number of documents in a table ##

You can count the number of documents with a `count` command:

```javascript
r.table("posts").count().run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

## Computing the average value of a field ##

You can compute the average value of a field with the `avg` command.

```javascript
r.table("posts").avg("num_comments").run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

## Using subqueries to return additional fields ##

Suppose we'd like to to retrieve all the posts in the table `post` and
also return an additional field, `comments`, which is an array of all
the comments for the relevant post retrieved from the `comments`
table. We could do this using a subquery:

```javascript
r.table("posts").merge(function(post) {
    return {
        comments: r.table("comments").filter(function(comment) {
            return comment("id_post").eq(post("id"))
        }).coerceTo("ARRAY")
    }
}).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

## Performing a pivot operation ##

Suppose the table `marks` stores the marks of every students per course:

```js
[
    {
        "name": "William Adama",
        "mark": 90,
        "id": 1,
        "course": "English"
    },
    {
        "name": "William Adama",
        "mark": 70,
        "id": 2,
        "course": "Mathematics"
    },
    {
        "name": "Laura Roslin",
        "mark": 80,
        "id": 3,
        "course": "English"
    },
    {
        "name": "Laura Roslin",
        "mark": 80,
        "id": 4,
        "course": "Mathematics"
    }
]
```

You may be interested in retrieving the results in this format

```js
[
    {
        "name": "Laura Roslin",
        "Mathematics": 80,
        "English": 80
    },
    {
        "name": "William Adama",
        "Mathematics": 70,
        "English": 90
    }
]
```

In this case, you can do a pivot operation with the `group` and
`coerceTo` commands.

```js
r.db('test').table('marks').group('name').map(function (row) {
    return [row('course'), row('mark')];
}).ungroup().map(function (res) {
    return r.expr({name: res('group')}).merge(res('reduction').coerceTo('object'));
}).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

_Note:_ A nicer syntax will eventually be added. See the
[Github issue 838](https://github.com/rethinkdb/rethinkdb/issues/838) to track
progress.


## Performing an unpivot operation ##

Doing an unpivot operation to "cancel" a pivot one can be done with the `concatMap`,
`map` and `keys` commands:

```js
r.table("pivotedMarks").concatMap(function (doc) {
    return doc.without("id", "name").keys().map(function (course) {
        return {
            name: doc("name"),
            course: course,
            mark: doc(course)
        };
    });
}).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

_Note:_ A nicer syntax will eventually be added. See the
[Github issue 838](https://github.com/rethinkdb/rethinkdb/issues/838) to track
progress.


## Renaming a field when retrieving documents ##

Suppose we want to rename the field `id` to `idUser` when retrieving
documents from the table `users`. In the subquery, we can use `merge` to add
a new field with the existing field's value, then `without` to delete the old
field:

```js
r.table("users").map(
    r.row.merge({ idUser: r.row("id") }).without("id")
).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```
## Grouping query results by date/time periods ##

ReQL has commands for extracting parts of [dates and times](/docs/dates-and-times/), including [year](/api/javascript/year), [month](/api/javascript/month), [day](/api/javascript/day), [dayOfWeek](/api/javascript/day_of_week) and more. You can use these with [group](/api/javascript/group) to group by various intervals. Suppose you had a table of invoices and wanted to retrieve them in groups ordered by year and month:

```js
r.table("invoices")
    .group([r.row("date").year(), r.row("date").month()])
    .ungroup()
    .merge({invoices: r.row('reduction'), month: r.row('group')})
    .without('reduction', 'group')
    .orderBy('month')
.run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

(We also use the technique for renaming a field, described above, to give the names "reduction" and "group" more useful names of "invoices" and "month.") You could use any combination of the ReQL date/time interval commands in the group, or work with the date/time as a native object.

Currently, ReQL has a default limit of 100,000 elements in an array, and the implementation of `group` requires the total number of documents grouped to fit within that boundary, so you are limited to 100,000 invoices. This can be changed, however, by passing the `arrayLimit` option to [run](/api/javascript/run). (Also note that `ungroup` always returns an array, although this may change in a future version. Follow issue [#2719](https://github.com/rethinkdb/rethinkdb/issues/2719) for progress on this.)

You can also use this approach with a [compound index](/docs/secondary-indexes/) on the intervals you want to group:

```js
r.table('invoices').indexCreate(
    'byDay', [r.row('date').year(), r.row('date').month(), r.row('date').day()]
).run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

Then you can use that index in the `group` function. This query would return the highest-value invoice for each day.

```js
r.table("invoices")
    .group({index: 'byDay'})
    .max('price')
.run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

{% endfaqsection %}

{% faqsection Miscellaneous %}

## Generating monotonically increasing primary key values ##

Efficiently generating monotonically increasing IDs in a distributed
system is a surprisingly difficult problem. If an inserted document is
missing a primary key, RethinkDB currently generates a random UUID. We
will be supporting additional autogeneration schemes in the future
(see [https://github.com/rethinkdb/rethinkdb/issues/117](https://github.com/rethinkdb/rethinkdb/issues/117)), but in the meantime, you can use one of the available open-source
libraries for distributed id generation (e.g. [twitter snowflake](https://github.com/twitter/snowflake)).

## Parsing RethinkDB's response to a write query ##

When you issue a write query (`insert`, `delete`, `update`, or
`replace`), RethinkDB returns a summary object that looks like this:

```javascript
{deleted:0, replaced:0, unchanged:0, errors:0, skipped:0, inserted:1}
```

The most important field of this object is `errors`.  Generally
speaking, if no exceptions are thrown and `errors` is 0 then the write
did what it was supposed to.  (RethinkDB throws an exception when it
isn't even able to access the table; it sets the `errors` field if it
can access the table but an error occurs during the write.  This
convention exists so that batched writes don't abort halfway through
when they encounter an error.)

The following fields are always present in this object:

* `inserted` -- Number of new documents added to the database.
* `deleted` -- Number of documents deleted from the database.
* `replaced` -- Number of documents that were modified.
* `unchanged` -- Number of documents that would have been modified, except that the new value was the same as the old value.
* `skipped` -- Number of documents that were unmodified in a write operation, because the document is not available to be deleted or updated. The document might have been deleted by a different operation happening concurrently, or in the case of a `get` operation the key might not exist.
* `errors` -- Number of documents that were left unmodified due to an error.

In addition, the following two fields are set as circumstances dictate:

* `generated_keys` -- If you issue an insert query where some or all of the rows lack primary keys, the server will generate primary keys for you and return an array of those keys in this field.  (The order of this array will match the order of the rows in your insert query.)
* `first_error` -- If `errors` is positive, the text of the first error message encountered will be in this field.  This is a very useful debugging aid.  (We don't return all of the errors because a single typo can result in millions of errors when operating on a large database.)

## Using dynamic keys in ReQL commands ##

Sometimes you may want to specify a key in a ReQL document dynamically. The easiest way to do that is with the `object` command, which takes a list of keys and values and returns an object from them.

Suppose you wanted to retrieve a list of distinct first names from a user table, but the table is too large to be used with the `distinct` command. You can use use `map` and `object` to turn the names into keys, then use `reduce` to remove duplicates:

```js
r.table('users').map(function (user) {
    return r.object(user('firstName'), true);
}).reduce(function (left, right) {
    return left.merge(right);
}).keys().run(conn, function(err, result) {
    if (err) throw err;
    console.log(result);
});
```

## Returning a ReQL query as a string ##

For testing or logging purposes, you might want to capture a created ReQL query as a string. (You can see an example of this in ReQL error messages.) While there is no ReQL command to do this, you can simply use the `toString()` method at the end of a query chain, rather than `run()`:

```js
r.table('users').filter(r.row('groups').contains('operators')).toString()
```

## Building ReQL queries on multiple lines ##

It's a common pattern in some query interfaces to "build" queries programmatically by instantiating a query object, calling it several times in succession to add query commands, then calling the execution command. This lets you dynamically change the query based on conditions at runtime. You might expect to do this in ReQL like so:

```js
var query = r.table('posts');
if (request.filter !== undefined) {
    query.filter(request.filter);
}
query.orderBy('date');
query.run(conn, callback);
```

However, that won't work! The reason is that the query object doesn't store state. Each of the commands after the first one is simply running on the *original* value of `query` (in this case, the `posts` table). You can solve this by explicitly assigning the output of each new command to the `query` variable:

```js
var query = r.table('posts');
if (request.filter !== undefined) {
    query = query.filter(request.filter);
}
query = query.orderBy('date');
query.run(conn, callback);
```

{% endfaqsection %}
