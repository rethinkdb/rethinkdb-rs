---
layout: documentation
title: Map-reduce in RethinkDB
active: docs
docs_active: map-reduce
permalink: docs/map-reduce/
---

Map/reduce is a way to process distributed data at scale.
It was originally designed by
[Google](http://research.google.com/archive/mapreduce.html) and later
implemented in many systems
(e.g. [Apache Hadoop](http://hadoop.apache.org/)).

In RethinkDB, map/reduce queries operate on sequences, and are
composed of three parts:

* An optional __group__ operation which partitions the elements of the
  sequence into multiple groups.
* A __map__ operation which maps the elements of the sequence to a
  desired value.
* A __reduce__ operation which aggregates the values produced by
  __map__ into a single value.

RethinkDB implements efficient, distributed map/reduce over
tables. Many ReQL commands automatically compile to map/reduce
queries. You can also write your own map/reduce queries with the
`map`, `reduce`, and `group` commands.

{% infobox info%}
__Want to learn more about map/reduce?__ Read the [Wikipedia article](http://en.wikipedia.org/wiki/MapReduce).
{% endinfobox%}

# An example #

Let's suppose you are running a blog and would like to retrieve the
number of posts per category. A map/reduce query to perform this
operation would consist of the following steps:

* A __group__ step that groups the posts based on their category.
* A __map__ step that transforms each post into the number `1` (since
  we're counting each post once).
* A __reduce__ step that sums the number of posts for each group.

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

{% infobox info%}
__Note__: Hadoop combines the grouping stage and the mapping stage
into a single, slightly more complicated mapping function. For ease of
use, RethinkDB breaks up the complex mapping function into two simpler
operations.
{% endinfobox %}

Let's first compute the number of posts per category by explicit calls
to `group`, `map`, and `reduce`:

```python
r.table('posts')                        \
  .group(lambda post: post['category']) \
  .map(lambda post: 1)                  \
  .reduce(lambda a,b: a+b)              \
  .run(conn)
```

This works, but the query is unwieldy.  Fortunately, RethinkDB offers
several shortcuts.  First, grouping by a field is very common, so if
you provide `group` with the name of a field rather than a function,
it will group by that field:

```python
r.table('posts')                        \
  .group('category')                    \
  .map(lambda post: 1)                  \
  .reduce(lambda a,b: a+b)              \
  .run(conn)
```

Second, counting is so common that there is a specialized command
`count` for it.  So we can simply write:

```python
r.table('posts').group('category').count().run(conn)
```

RethinkDB has built-in commands for five common map/reduce operations:
`count`, `sum`, `avg`, `min`, and `max`.  If you want to do something
more complicated than any of those, you can use `map` and `reduce`
explicitly.

# How map/reduce queries are executed #

RethinkDB's map/reduce queries are distributed and parallelized across
shards and CPU cores whenever possible.  This allows map/reduce
queries to execute efficiently, but leads to a common mistake.  The
function provided to `reduce` is __not__ called successively on the
elements of the stream from left to right -- it's called on either the
elements of the stream, in any order, or on the output of previous
calls to the function.

Here is an example of an __incorrect__ way to write the previous
grouped map/reduce query:

```python
r.table('posts')                        \
  .group(lambda post: post['category']) \
  .map(lambda post: 1)                  \
  .reduce(lambda a,b: a+1)              \ # INCORRECT
  .run(conn)
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
may not be necessary. If you don't need a grouping function, you can
just leave off the `group` command.

For example, to count the number of posts in a table you can run
either of the following queries:

```python
r.table('posts').map(lambda row: 1).reduce(lambda a,b: a+b).run(conn)
r.table('posts').count().run(conn)
```

# Read more #
- [The API documentation](/api/python/group/) for the `group` command.
- [The API documentation](/api/python/ungroup/) for the `ungroup` command.
- [The API documentation](/api/python/map/) for the `map` command.
- [The API documentation](/api/python/reduce/) for the `reduce` command.
- [The Wikipedia article](http://en.wikipedia.org/wiki/MapReduce) on map/reduce.
- The [Hadoop tutorial](http://hadoop.apache.org/docs/r1.2.1/mapred_tutorial.html) for map/reduce.
