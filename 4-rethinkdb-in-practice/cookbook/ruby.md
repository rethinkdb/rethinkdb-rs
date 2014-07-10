---
layout: documentation
title: Cookbook for Ruby
active: docs
docs_active: cookbook
js: faq_index
permalink: docs/cookbook/ruby/
switcher: true
language : Ruby
---
{% include recipe-forms.html %}

<img src="/assets/images/docs/api_illustrations/cookbook.png" class="api_command_illustration" />

<div id="faqcontents"></div>
---
{% faqsection Basic commands %}

## Creating a database ##

You can use the `db_create` method as follows:

```ruby
r.db_create("blog").run
```

Another way to create a database is through the web UI. You can reach
the web UI at `http://HOST:8080`. Click on the _Tables_ tab at the top
and then use the _Add Database_ button.

## Creating a table ##

You can select the database where you'd like to create the table with
the `db` command and use the `table_create` command as follows:

```ruby
r.db("blog").table_create("posts").run
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

```ruby
r.table("user").insert({
    "name" => "Michel",
    "age" => 26
}).run
```

You can insert multiple documents at once by passing an array of
documents to `insert` as follows:

```ruby
r.table("user").insert([
    {
        "name" => "Michel",
        "age" => 26
    },
    {
        "name" => "Slava",
        "age" => 30
    }
]).run
```

## Deleting documents ##

To delete documents, select the documents you'd like to delete and use
the `delete` command. For example, let's delete all posts with the
author "Michel":

```ruby
r.table("posts").filter{|post| post["author"].eq("Michel")}.delete.run
```

Or, let's try to delete a single user:

```ruby
r.table("posts").get("7644aaf2-9928-4231-aa68-4e65e31bf219").delete.run
```

Here is how we'd delete all documents in a table:

```ruby
r.table("posts").delete.run
```

{% endfaqsection %}

{% faqsection Filtering %}

## Filtering based on multiple fields ##

Suppose you'd like to select all posts where the author's name is
"Michel" and the category is "Geek". You can do it as follows:

```ruby
r.table("posts").filter({
    "author" => "Michel",
    "category" => "Geek",
}).run
```

Alternatively, you can use the overloaded `&` operator to build a
predicate and pass it to `filter`:

```ruby
r.table("posts").filter{|post|
    (post["author"].eq("Michel")) & (post["category"].eq("Geek"))
}.run
```

__Note__: RethinkDB overloads `&` because Ruby doesn't allow
overloading the proper _and_ operator. Since `&` has high precedence,
make sure to wrap the conditions around it in parentheses.

You can also use the `r.and` command, if you prefer not using
overloaded `&`:

```ruby
r.table("posts").filter{|post|
    r.and(post["author"].eq("Michel"),
          post["category"].eq("Geek"))}.run
```

Similarly, you can use the overloaded `|` operator or the equivalent
`r.or` command to filter based on one of many conditions.

## Filtering based on the presence of a value in an array ##

Suppose we have a table `users` with documents of the following form:

```ruby
{
    "name" => "William Adama"
    "emails" => ["bill@bsg.com", "william@bsg.com"],
    "ship" => "Galactica"
}
```

If we want to retrieve all users that have the email address
`user@email.com`, we can write:

```ruby
r.table("user").filter{|user| user["emails"].contains("user@email.com")}.run
```

If we want to retrieve all users on the Galactica and Pegasus, we can write:

```rb
r.table("user").filter{ |user|
    r.expr(["Galactica", "Pegasus"]).contains(user["ship"])
}.run()

## Filtering based on nested fields ##

In Ruby you can use the operator `[]` to get the value of a
field. This operator can be chained to retrieve values from nested
fields.

Suppose we have a table `users` with documents of the following form:

```ruby
{
    "name" => "William Adama"
    "contact" => {
        "phone" => "555-5555"
        "email" => "bill@bsg.com"
    }
}
```

Let's filter based on the nested field `email`:

```ruby
r.table("user").filter{|user|
    user["contact"]["email"].eq("user@email.com")
}.run
```

For many ReQL commands, you can also use a JSON-style nested syntax that allows
considerably more flexibility. Read "[Accessing nested fields](/docs/nested-fields)"
for more information.

## Efficiently retrieving multiple documents by primary key ##

If you want to retrieve all the posts with the primary keys `1`, `2`,
or `3` you can use the `get_all` command:

```ruby
r.table("posts").get_all(1, 2, 3).run
```

## Efficiently retrieving multiple documents by secondary index ##

Suppose we have a table `posts` that links posts to authors via an
`author_id` field. If we've created a secondary index on `author_id`
and want to retrieve all the posts where `author_id` is `1`, `2`, or
`3`, we can use the `get_all` command to do it as follows:

```ruby
r.table("posts").get_all(1, 2, 3, :index=>'author_id').run
```

{% infobox info %}
Read about [creating secondary indexes in RethinkDB](/docs/secondary-indexes/).
{% endinfobox %}

## Retrieving all the objects in a stream (cursor) as an array ##

If you're using a command that returns a stream and want to retrieve all of
its results at once in an array rather than iterating through them with the
cursor object, you can coerce it to an array using `to_a`.

```rb
posts = r.table('posts').run(conn).to_a
```

See the [data type documentation](/docs/data-types/) for more detail about
streams.

## Returning specific fields of a document ##

If you need to retrieve only a few specific fields from your
documents, you can use the `pluck` command. For example, here is how
you'd return only the fields `name` and `age` from each row in table
`users`:

```ruby
r.table("posts").pluck("name", "age").run
```

This is equivalent to calling `SELECT name, age FROM users` in SQL.

The `pluck` command also supports selecting nested fields in a
document. For example, suppose we'd like to select the fields `phone`
and `email` from the following document:

```ruby
{
    "name" => "William Adama"
    "contact" => {
        "phone" => "555-5555"
        "email" => "bill@bsg.com"
    }
}
```

We can use the following syntax:

```ruby
r.table("users").pluck({"contact"=>{"phone"=>true, "email"=>true}}).run
```


## Filtering based on a date range ##
Suppose you want to retrieve all the posts whose date field is
between January 1st, 2012 (included) and January 1st, 2013 (excluded), you could do:

```rb
r.table("posts").filter{ |post|
    post.during(r.time(2012, 1, 1, 'Z'), r.time(2013, 1, 1, 'Z'))
}.run(conn)
```

You can also manually compare dates:

```rb
r.table("posts").filter{ |post|
    (post["date"] >= r.time(2012, 1, 1, 'Z')) &
    (post["date"] < r.time(2013, 1, 1, 'Z'))
}.run(conn)
```


## Filtering with regex ##

If you want to retrieve all users whose last name starts with "Ma", 
you can use `r.match` this way:

```rb
# Will return Martin, Martinez, Marshall etc.
r.table("users").filter{ |user|
    user["lastName"].match("^Ma")
}.run(conn)
```

If you want to retrieve all users whose last name ends with an "s", 
you can use `r.match` this way:

```rb
# Will return Williams, Jones, Davis etc.
r.table("users").filter{ |user|
    user["lastName"].match("s$")
}.run(conn)
```

If you want to retrieve all users whose last name contains "ll", 
you can use `r.match` this way:

```rb
# Will return Williams, Miller, Allen etc.
r.table("users").filter{ |user|
    user["lastName"].match("ll")
}.run(conn)
```

## Case insensitive filter ##

Retrieve all users whose name is "William" (case insensitive).

```rb
# Will return william, William, WILLIAM, wiLLiam etc.
r.table("users").filter{ |user|
    user["lastName"].match("(?i)^william$")
}.run(conn)

```


{% endfaqsection %}

{% faqsection Manipulating documents %}

## Adding/overwriting a field in a document ##

To add or overwrite a field, you can use the `update` command.  For
instance, if you would like to add the field `author` with the value
"Michel" for all of the documents in the table `posts`, you would use:

```ruby
r.table("posts").update({ "author" => "Michel" }).run
```

## Removing a field from a document ##

The `update` command lets you to overwrite fields, but not delete
them. If you want to delete a field, use the `replace` command. The
`replace` command replaces your entire document with the new document
you pass as an argument. For example, if you want to delete the field
`author` of the blog post with the id `1`, you would use:

```ruby
r.table("posts").get("1").replace{|doc| doc.without('author')}.run
```

## Atomically updating a document based on a condition ##

All modifications made via the `update` and `replace` commands are
always atomic with respect to a single document. For example, let's
say we'd like to atomically update a view count for a page if the
field `countable` is set to true, and get back the old and new results
in a single query. We can perform this operation as follows:

```ruby
r.table("pages").update{|page|
    r.branch(page["countable"].eq(true),         // if the page is countable
             { "views"=>page["views"] + 1 },     // increment the view count
             {}                                  // else do nothing
    )}, {"return_vals"=>true}).run()
```

## Storing timestamps and JSON date strings as Time data types ##

You can use the `epoch_time` and `iso8601` commands to convert Unix timestamps (in seconds) and JSON date-time strings (which are in ISO 8601 format) to the ReQL time type. The ReQL driver will also convert Ruby Time objects into ReQL time, but will not automatically convert Date or DateTime objects.

```rb
the_date = Time.now
timestamp = the_date.to_i
json_date = the_date.iso8601
r.table("dates").insert({
    :from_object => the_date,
    :from_epoch => r.epoch_time(timestamp),
    :from_iso => r.iso8601(json_date)
}).run(conn)
```

{% endfaqsection %}

{% faqsection Pagination %}

## Limiting the number of returned documents ##

You can limit the number of documents returned by your queries with
the `limit` command. Let's retrieve just the first 10 blog posts:

```ruby
r.table("posts").order_by("date")
    .limit(10)
    .run
```

## Implementing pagination ##

To paginate results, you can use a combination of the `skip` and
`limit` commands. Let's retrieve posts 11-20 from our database:

```ruby
r.table("posts").order_by("date").
  skip(10).
  limit(10).run
```

{% endfaqsection %}

{% faqsection Transformations %}

## Counting the number of documents in a table ##

You can count the number of documents with a `count` command:

```ruby
r.table("posts").count.run
```

## Computing the average value of a field ##

You can compute the average value of a field with the `avg` command.

```ruby
r.table("posts").avg("num_comments").run
```

## Using subqueries to return additional fields ##

Suppose we'd like to to retrieve all the posts in the table `post` and
also return an additional field, `comments`, which is an array of all
the comments for the relevant post retrieved from the `comments`
table. We could do this using a subquery:

```ruby
r.table("posts").merge{ |post|
    {
        "comments" => r.table("comments").filter{ |comment|
            comment["id_post"].eq(post["id"])
        }.coerce_to("ARRAY")
    }
}.run
```

## Performing a pivot operation ##

Suppose the table `marks` stores the marks of every students per course:

```rb
[
    {
        :name => "William Adama",
        :mark => 90,
        :id => 1,
        :course => "English"
    },
    {
        :name => "William Adama",
        :mark => 70,
        :id => 2,
        :course => "Mathematics"
    },
    {
        :name => "Laura Roslin",
        :mark => 80,
        :id => 3,
        :course => "English"
    },
    {
        :name => "Laura Roslin",
        :mark => 80,
        :id => 4,
        :course => "Mathematics"
    }
]
```

You may be interested in retrieving the results in this format

```rb
[
    {
        :name => "Laura Roslin",
        :Mathematics => 80,
        :English => 80
    },
    {
        :name => "William Adama",
        :Mathematics => 70,
        :English => 90
    }
]
```

In this case, you can do a pivot operation with the `group` and
`coerce_to` commands.


```rb
r.db('test').table('marks').group('name').map { |row|
    [row['course'], row['mark']]
}.ungroup().map { |res|
    r.expr({name: res['group']}).merge(res['reduction'].coerce_to('object'))
}.run(conn)
```

_Note:_ A nicer syntax will eventually be added. See the
[Github issue 838](https://github.com/rethinkdb/rethinkdb/issues/838) to track
progress.


## Performing an unpivot operation ##

Doing an unpivot operation to "cancel" a pivot one can be done with the `concat_map`,
`map` and `keys` commands:

```rb
r.table("pivoted_marks").concat_map { |doc|
    doc.without("id", "name").keys().map { |course|
    	{
            :name => doc["name"],
            :course => course,
            :mark => doc[course]
        }
    }
)
```

_Note:_ A nicer syntax will eventually be added. See the
[Github issue 838](https://github.com/rethinkdb/rethinkdb/issues/838) to track
progress.


## Renaming a field when retrieving documents ##

Suppose we want to rename the field `id` to `id_user` when retrieving
documents from the table `users`. We could do:

```rb
r.table("users").map{ |user|
    # Add the field id_user that is equal to the id one
    user.merge({
        "id_user" => user["id"]
    })
    .without("id") # Remove the field id
}
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

```ruby
{"deleted"=>0, "replaced"=>0, "unchanged"=>0, "errors"=>0, "skipped"=>0, "inserted"=>1}
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
* `skipped` -- Number of documents that were left unmodified because there was nothing to do.  (For example, if you delete a row that has already been deleted, that row will be "skipped".  This field is sometimes positive even when operating on a selection, because a concurrent write might get to the value first.)
* `errors` -- Number of documents that were left unmodified due to an error.

In addition, the following two fields are set as circumstances dictate:

* `generated_keys` -- If you issue an insert query where some or all of the rows lack primary keys, the server will generate primary keys for you and return an array of those keys in this field.  (The order of this array will match the order of the rows in your insert query.)
* `first_error` -- If `errors` is positive, the text of the first error message encountered will be in this field.  This is a very useful debugging aid.  (We don't return all of the errors because a single typo can result in millions of errors when operating on a large database.)

## Using dynamic keys in ReQL commands ##

Sometimes you may want to specify a key in a ReQL document dynamically. The easiest way to do that is with the `object` command, which takes a list of keys and values and returns an object from them.

Suppose you wanted to retrieve a list of distinct first names from a user table, but the table is too large to be used with the `distinct` command. You can use use `map` and `object` to turn the names into keys, then use `reduce` to remove duplicates:

```rb
r.table('users').map { |user|
    r.object(user['first_name'], true)
}.reduce { |left, right|
    left.merge(right)
}.keys().run(conn)
```

## Returning a ReQL query as a string ##

For testing or logging purposes, you might want to capture a created ReQL query as a string. (You can see an example of this in ReQL error messages.) While there is no ReQL command to do this, you can simply use the `inspect()` method at the end of a query chain, rather than `run()`:

```rb
r.table('users').filter{ |user| user['groups'].contains('operators')}.inspect()
```

{% endfaqsection %}
