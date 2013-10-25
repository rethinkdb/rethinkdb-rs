---
layout: documentation
title: Map-reduce in RethinkDB
active: docs
docs_active: map-reduce
permalink: docs/map-reduce/
---

Map/reduce is an operation on a sequence of documents that allows
performing distributed data processing at scale. Originally designed
by [Google](http://research.google.com/archive/mapreduce.html) and
later implemented in systems like [Apache
Hadoop](http://hadoop.apache.org/), a map/reduce query is composed of
two parts:

- A __map__ operation &mdash; each document is mapped to a key/value pair.
- A __reduce__ operation &mdash; an aggregation operation (such as counting or summation) that operates on values grouped by key.

RethinkDB implements an efficient, distributed map/reduce
infrastructure. Many ReQL operations automatically compile to a
map/reduce query. However, you can also use map/reduce directly via
the [grouped\_map\_reduce](/api/python/grouped_map_reduce/)
command.

{% infobox info%}
__Want to learn more about map/reduce?__ Read the [Wikipedia article](http://en.wikipedia.org/wiki/MapReduce).
{% endinfobox%}

# An example #

Let's suppose you are running a blog and would like to retrieve the
number of posts per category. A map/reduce query to perform this
operation would consist of the following steps:

- A __map__ step that returns a key/value pair for each post, where
  the key is the category of the post and the value is `1` (since each
  post needs to counted once).
- A __reduce__ step that sums the values for each category.

# Map/reduce in RethinkDB #

For our blog, we have a table `posts` that contains blog posts. Here's
an example document from the table:

```python
{
    "id": "7644aaf2-9928-4231-aa68-4e65e31bf219"
    "title": "The line must be drawn here"
    "content": "This far, no further! ..."
    "category": "Fiction"
}
```

Let's compute the number of posts per category. The
`grouped_map_reduce` command requires three arguments: a grouping
function, a mapping function, and a reduction function.

- __A grouping function__ must return a group of a given document.
- __A map function__ must return a value for a given document that will be aggregated.
- __A reduce function__ specifies how to reduce all the values.

{% infobox info%}
__Note__: Hadoop combines the grouping stage and the mapping stage
into a single, slightly more complicated mapping function. For ease of
use, RethinkDB breaks up the complex mapping function into two simpler
operations.
{% endinfobox %}

To compute the number of posts per category, we would write:

```python
r.table("post").grouped_map_reduce(
    lambda post: post["category"],   # Returns the group (category) for each post
    lambda post: 1,                  # Each post will counted once
    lambda x, y: x + y               # Sum two values
).run()
```

This map/reduce query is equivalent to a simpler ReQL command:

```python
r.table("post").grouped_by("category", r.count).run()
```

{% infobox info%}
__Note__: a more user friendly map/reduce syntax will be available soon &mdash;
see [Github issue #1096](https://github.com/rethinkdb/rethinkdb/issues/1096).
{% endinfobox %}

# How map/reduce queries are executed #

One important aspect of the `grouped_map_reduce` is that it is
distributed and parallelized across shards and CPU cores. This allows
map/reduce queries to execute efficiently, but is a source of a common
mistake: assuming an __incorrect__ reduction order.

Since the reduction step is performed in parallel, the summation
operation above may be performed in the following way:

```python
(1 + 1) + (1 + 1)
```

In other words, the reduction is __not__ always performed from left to
right. Here is an example of an incorrect way to write the
`grouped_map_reduce` query:

```python
r.table("post").grouped_map_reduce(
    lambda post: post["category"],
    lambda post: 1,
    lambda x, y: x + 1 # Will not work! `x` is not an aggregator!
).run()
```

If we have four documents in a single category in a sharded table,
here is a possible execution path for the command above:

1. Two of the documents are located on shard 1, the other two documents
   are on shard 2. RethinkDB runs the reduction on both shards in
   parallel.
2. The number of documents is computed on shard 1 &mdash; the query
   returns the value `2` for the shard.
3. The number of documents is computed on shard 2 &mdash; the query
   returns the value `2` for the shard.
4. However, the final reduction step (combining the values of two
   shards) doesn't work. Instead of computing `2 + 2` the query above
   will compute `2 + 1` instead.

{% infobox %}
__Be careful!__ Make sure your reduction function doesn't assume the
reduction step executes from left to right!
{% endinfobox %}

# Simplified map/reduce #

In cases where you need to do simpler computations, the grouping stage
may not be necessary. If you don't need a grouping function you can
use the simpler `map` and `reduce` commands directly:

For example, to count the number of posts in a table you can run the
following query:

```python
r.db("blog").table("posts").map(lambda row: 1).reduce(lambda x, y: x + y).run()
```

An equivalent full map/reduce query would look like this:

```python
r.table("post").grouped_map_reduce(
    lambda post: 1,       # We only have one group
    lambda post: 1,       # Each post will counted once
    lambda x, y: x + y    # Sum two values
).run()
```

Both of these queries are equivalent to the following ReQL query:

```python
r.db("blog").table("posts").count().run()
```

# Read more #
- [The API documentation](/api/python/grouped_map_reduce/) for the `grouped_map_reduce` command.
- [The Wikipedia article](http://en.wikipedia.org/wiki/MapReduce) on map/reduce.
- The [Hadoop tutorial](http://hadoop.apache.org/docs/stable/mapred_tutorial.html) for map/reduce.
