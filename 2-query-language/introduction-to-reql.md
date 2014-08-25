---
layout: documentation
title: Introduction to ReQL
active: docs
docs_active: introduction-to-reql
permalink: docs/introduction-to-reql/
---

ReQL is the RethinkDB query language. It offers a very powerful and
convenient way to manipulate JSON documents. This document is a gentle
introduction to ReQL concepts. You don't have to read it to be
productive with RethinkDB, but it helps to understand some basics.

{% infobox info %}
<strong>Want to write useful queries right away?</strong> Check out the [ten-minute guide](/docs/guide/javascript/).
{% endinfobox %}

<img src="/assets/images/docs/api_illustrations/introduction_to_reql.png" class="api_command_illustration" />

ReQL is different from other NoSQL query languages. It's built on
three key principles:

1. __ReQL embeds into your programming language.__ Queries are
   constructed by making function calls in the programming language
   you already know. You don't have to concatenate strings or
   construct specialized JSON objects to query the database.
2. __All ReQL queries are chainable__. You begin with a table and
   incrementally chain transformers to the end of the query using the
   `.` operator.
3. __All queries execute on the server__. While queries are
   constructed on the client in a familiar programming language, they
   execute entirely on the database server once you call the `run`
   command and pass it an active database connection.

Let's look at these concepts in more detail.

{% infobox info %}
<strong>Note:</strong> the following examples use the Python driver,
but most of them also apply to RethinkDB drivers for other languages.
{% endinfobox %}

# ReQL embeds into your programming language #

You start using ReQL in your program similarly to how you'd use other
databases:

```python
import rethinkdb as r  # import the RethinkDB package
conn = r.connect()       # connect to the server on localhost and default port
```

But this is where the similarity ends. Instead of constructing strings
and passing them to the database server, you access ReQL by using
methods from the `rethinkdb` package:

```python
r.table_create('users').run(conn)   # create a table `users`
r.table('users').run(conn)          # get an iterable cursor to the `users` table
```

Every ReQL query, from filters, to updates, to table joins is done by
calling appropriate methods.

{% infobox info %}
__This design has the following advantages:__

* You can use the same programming environment and tools you're
  already used to.
* Learning the language is no different from learning any other
  library.
* There is little to no chance of security issues that arise from
  string injection attacks.
{% endinfobox %}

# All ReQL queries are chainable #

In ReQL, you can chain commands at the end of other commands using the
`.` operator:

```python
# Get an iterable cursor to the `users` table (we've seen this above)
r.table('users').run(conn)

# Return only the `last_name` field of the documents
r.table('users').pluck('last_name').run(conn)

# Get all the distinct last names (remove duplicates)
r.table('users').pluck('last_name').distinct().run(conn)

# Count the number of distinct last names
r.table('users').pluck('last_name').distinct().count().run(conn)
```

Almost all ReQL operations are chainable. You can think of the `.`
operator similarly to how you'd think of a Unix pipe. You select the
data from the table and pipe it into a command that transforms it. You
can continue chaining transformers until your query is done. In ReQL,
data flows from left to right.

Even if you have a cluster of RethinkDB nodes, you can send your
queries to any node and the cluster will create and execute
distributed programs that get the data from relevant nodes, perform
the necessary computations, and present you with final results
without you ever worrying about it.

{% infobox info %}
__This design has the following advantages:__

* The language is easy to learn, read, and modify.
* It's a natural and convenient way to express queries.
* You can construct queries incrementally by chaining transformations
  and examining intermediary results.
{% endinfobox %}

# ReQL is efficient #

## Server-side execution ##

While queries are built up on the client, they're only sent to the
server once you call the `run` command. All processing happens on the
server &mdash; the queries don't run on the client, and don't require
intermediary network round trips between the client and the
server. For example, you can store queries in variables, and send them
to the server later:

```python
# Create the query to get distinct last names
distinct_lastnames_query = r.table('users').pluck('last_name').distinct()

# Send it to the server and execute
distinct_lastnames_query.run(conn)
```

{% infobox info %}
Read about [how this technology is implemented](/blog/lambda-functions/) for more details.
{% endinfobox %}

## Laziness ##

ReQL queries are executed lazily:

```python
# Get up to five user documents that have the `age` field defined
r.table('users').has_fields('age').limit(5).run(conn)
```

For this query RethinkDB will perform enough work to get the five
documents, and stop when the query is satisfied. Even if you don't
have a limit on the number of queries but use a cursor, RethinkDB will
do just enough work to allow you to read the data you request. This
allows queries to execute quickly without wasting CPU cycles, network
bandwidth, and disk IO.

Like most database systems, ReQL supports primary and secondary
indexes to allow efficient data access. You can also create compound
indexes and indexes based on arbitrary ReQL expressions to speed up
complex queries.

{% infobox info %}
Learn how to use [primary and secondary indexes](/docs/secondary-indexes/) in RethinkDB.
{% endinfobox %}

## Parallelism ##

All ReQL queries are automatically parallelized on the RethinkDB
server as much as possible. Whenever possible, query execution is
split across CPU cores, machines in the cluster, and even multiple
datacenters. If you have large, complicated queries that require
multiple stages of processing, RethinkDB will automatically break them
up into stages, execute each stage in parallel, and combine data to
return a complete result.

## Query optimization ##

While RethinkDB doesn't currently have a fully-featured query
optimizer, ReQL is designed with one in mind. For example, the server
has enough information to reorder the chain for efficiency, or to use
alternative implementation plans to improve performance. This feature
will be introduced into future versions of RethinkDB.

# ReQL queries are functional #

So far we've seen only simple queries without conditions. ReQL
supports a familiar syntax for building more advanced queries:

```python
# Get all users older than 30
r.table('users').filter(lambda user: user['age'] > 30).run(conn)

# If you'd like to avoid writing lambdas, RethinkDB supports an
# alternative syntax:
r.table('users').filter(r.row['age'] > 30).run(conn)
```

This query looks just like any other Python code you would normally
write. Note that RethinkDB will execute this query on the server, and
it doesn't execute native Python code.

The client drivers do a lot of work to inspect the code and convert it
to an efficient ReQL query that will be executed on the server:

* Whenever possible, the client drivers use operator overloading to
  support expressions such as `user['age'] > 30`.
* The `lambda` expression is executed only once on the
  client. Internally, the driver passes a special object to the
  `lambda` function which allows constructing a representation of the
  query. This representation is then sent to the server over the
  network and evaluated on the cluster.

{% infobox info %}
Read about [how this technology is implemented](/blog/lambda-functions/) for more details.
{% endinfobox %}

This technology has limitations. While most operations allow you to
write familiar code, you can't use native language's operations that
have side effects (such as `print`) or control blocks (such as `if`
and `for`). Instead, you have to use alternative ReQL commands:

```python
# WRONG: Get all users older than 30 using the `if` statement
r.table('users').filter(lambda user:
    print "Testing"      # WRONG: this will only execute once on the client
    if user['age'] > 30:
        True,
        False).run(conn)

# RIGHT: Get all users older than 30 using the `r.branch` command
r.table('users').filter(lambda user:
    r.branch(user['age'] > 30,
             True,
             False)).run(conn)
```

{% infobox info %}
__This design has the following advantages:__

* For most queries, you can write familiar, easy to learn code without
  learning special commands.
* The queries are efficiently transported to the server (via protocol
  buffers), and evaluated in the cluster.
* RethinkDB has access to the query structure, which allows for
  optimization techniques similar to those available in SQL. This
  feature will be added to RethinkDB in the future.

__This technology has the following limitation:__

* Native language's operations that have side effects or control
  blocks cannot be used within a `lambda`. Learn more about [how this
  design is implemented](/blog/lambda-functions/) for details.

{% endinfobox %}


# ReQL queries are composable #

You can combine multiple ReQL queries to build more complex
ones.

## Composing simple commands ##

Let's start with a simple example. RethinkDB supports
server-side Javascript evaluation using the embedded V8 engine
(sandboxed within outside processes, of course):

```python
# Evaluate a Javascript expression on the server and get the result
r.js('1 + 1').run(conn)
```

Because ReQL is composable you can combine the `r.js` command with any
other query. For example, let's use it as an alternative to get all
users older than 30:

```python
# Get all users older than 30 (we've seen this above)
r.table('users').filter(lambda user: user['age'] > 30).run(conn)

# Get all users older than 30 using server-side JavaScript
r.table('users').filter(r.js('(function (user) { return user.age > 30; })')).run(conn)
```

RethinkDB will seamlessly evaluate the `js` command by calling into
the V8 engine during the evaluation of the `filter` query. You can
combine most queries this way into progressively more complex ones.

## Subqueries ##

Let's say we have another table `authors`, and we'd like to get a list
of authors whose last names are also in the `users` table we've seen
before. We can do it by combining two queries:

```python
# Find all authors whose last names are also in the `users` table
r.table('authors').filter(lambda author:
    r.table('users').pluck('last_name').contains(author.pluck('last_name'))).
    run(conn)
```

Here, we use the `r.table('users').pluck('last_name')` query as the
inner query in `filter`, combining the two queries to build a more
sophisticated one. Even if you have a cluster of machines and both the
`authors` table and the `users` table are sharded, RethinkDB will do
the right thing and evaluate relevant parts of the query above on the
appropriate shards, combine bits of data as necessary, and return the
complete result.

{% infobox info %}
__A few things to note about this query:__

* We compose the query on the client and call `run` only
  once. Remember to call `run` only once on the complex query when
  you're ready for it to be executed.
* You can also perform this query using the
  [inner_join](/api/python/inner_join/) command.
{% endinfobox %}

## Expressions ##

Composing queries isn't limited to simple commands and inner
queries. You can also use expressions to perform complex
operations. For example, suppose we'd like to find all users whose
salary and bonus don't exceed $90,000, and increase their salary by
10%:

```python
r.table('users').filter(lambda user: user['salary'] + user['bonus'] < 90000)
 .update(lambda user: {'salary': user['salary'] + user['salary'] * 0.1})
```

## Rich command-set ##

In addition to commands described here, ReQL supports a number of
sophisticated commands that are composable similarly to the commands
described here. See the following documentation for more details:

* Learn how to use [map-reduce](/docs/map-reduce/) in RethinkDB.
* Learn how to use [table joins](/docs/table-joins/) in RethinkDB.
* Browse the [API reference](/api) for more commands.

{% infobox info %}
__This design has the following advantages:__

* Unlike most NoSQL languages, you can use ReQL to build queries of
  arbitrary complexity.
* There is no new syntax or new commands for complex queries. Once you
  understand the composition principle you can write new queries
  without learning anything else.
* Subqueries can be abstracted in variables, which allows for modular
  programming in the same way as done by most other modern programming
  languages.
{% endinfobox %}

# And just for kicks, ReQL can do math! #

Just in case you needed another calculator, ReQL can do that too!

```python
# Add two plus two
(r.expr(2) + r.expr(2)).run(conn)

# You only need to specify `r.expr` once for the driver to work
(r.expr(2) + 2).run(conn)

# More algebra
(r.expr(2) + 2 / 2).run(conn)

# Logic
(r.expr(2) > 3).run(conn)

# Branches
r.branch(r.expr(2) > 3,
         1,  # if True, return 1
         2   # otherwise, return 2
  ).run(conn)

# Compute the Fibonacci sequence
r.table_create('fib').run(conn)
r.table('fib').insert([{'id': 0, 'value': 0}, {'id': 1, 'value': 1}]).run(conn)
r.expr([2, 3, 4, 5, 6, 7, 8, 9, 10, 11]).for_each(lambda x:
  r.table('fib').insert({'id': x,
                         'value': (r.table('fib').order_by('id').nth(x - 1)['value'] +
                                   r.table('fib').order_by('id').nth(x - 2)['value'])
                        })).run(conn)
r.table('fib').order_by('id')['value'].run(conn)
```

# Read More #

Browse the following resources to learn more about ReQL:

- [Lambda functions in RethinkDB](/blog/lambda-functions/)
- [Introduction to map-reduce](/docs/map-reduce/)
- [Introduction to Joins](/docs/table-joins/)
- [API Reference](/api/)
