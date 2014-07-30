---
layout: documentation
title: MapReduce in RethinkDB
active: docs
docs_active: map-reduce
permalink: docs/map-reduce/
---

<img src="/assets/images/docs/api_illustrations/map-reduce.png"
     alt="MapReduce Illustration"
     class="api_command_illustration" />

[MapReduce][wp] is a way to summarize and run aggregation functions on large data sets, potentially stored across many machines, in an efficient fashion. It works by processing the data on each server in parallel and then combining those results into one set. It was originally designed by [Google][g] and later implemented in database systems such as [Apache Hadoop][ah] and [MongoDB][md].

[wp]: http://en.wikipedia.org/wiki/MapReduce
[g]: http://research.google.com/archive/mapreduce.html
[ah]: http;//hadoop.apache.org/
[md]: http://www.mongodb.org/

In RethinkDB, MapReduce queries operate on sequences and are composed of two or three parts:

* An optional __group__ operation which partitions the elements of the sequence into multiple groups.
* A __map__ operation which filters and/or transforms the elements in the sequence (or each group) into a new sequence (or grouped sequences).
* A __reduce__ operation which aggregates the values produced by __map__ into a single value (or a single value for each group).

Some other MapReduce implementations, like Hadoop's, use the mapping step to perform grouping as well; RethinkDB's implementation explicitly separates them. This is sometimes referred to as "GroupMapReduce," or GMR. RethinkDB distributes GMR queries over tables and shards efficiently. You write GMR queries with the [group][], [map][] and [reduce][] commands, although as we'll see in our examples, many ReQL commands compile to GMR queries behind the scenes--many common MapReduce cases can be accomplished in one or two lines of ReQL.

# A simple example #

Suppose you are running a blog and would like to retrieve the number of posts. A MapReduce query to perform this operation would consist of the following steps:

* A __map__ step that transforms each post into the number `1` (since we're counting each post once).
* A __reduce__ step that sums the number of posts.

We won't need a __group__ step for this example.

For our blog, we have a table `posts` that contains blog posts. Here's an example document from the table. (We'll use Python for this example, but other ReQL drivers are very similar.)

```python
{
    "id": "7644aaf2-9928-4231-aa68-4e65e31bf219"
    "title": "The line must be drawn here"
    "content": "This far, no further! ..."
    "category": "Fiction"
}
```

First, we'll map each post to the number `1`:

```py
r.table('posts').map(lambda post: 1)
```

And sum the posts with `reduce`:

```py
r.table('posts').map(lambda post: 1).reduce(lambda a, b: a + b).run(conn)
```

For many cases where a GMR query might be used, ReQL provides even simpler aggregation functions. This example is really more easily written using [count](/api/python/count):

```py
r.table('posts').count().run(conn)
```

RethinkDB has shortcuts for five common aggregation operations: `count`, `sum`, `avg`, `min`, and `max`. In practice, you'll often be able to use these with `group` these rather than writing your own `map` and `reduce` functions.


# An example with group #

Suppose on the blog in the last example, you'd like to retrieve the number of posts _per category._ A MapReduce query to perform this operation would consist of the following steps:

* A __group__ step that groups the posts based on their category.
* The __map__ step from above.
* A __reduce__ step that sums the number of posts for each group.

First, we'll `group` the posts:

```py
r.table('posts').group(lambda post: post['category'])
```

Then as before, we map each post to the number `1`. Commands after the `group` command will be applied to each grouped set.

```py
r.table('posts').group(lambda post: post['category']).map(
    lambda post: 1)
```

And again, we sum the posts with `reduce`, which produces totals for each group this time:

```py
r.table('posts').group(lambda post: post['category']).map(
    lambda post: 1).reduce(lambda a, b: a + b).run(conn)
```

And, of course, we can use `count` to shorten that. We can actually shorten it even more: ReQL will let you provide `group` with the name of the field rather than a lambda function. So the simplified function is:

```py
r.table('posts').group('category').count().run(conn)
```

# A more complex example #

This is based on an example from [MongoDB][me]. Imagine a table of orders, with each document in the table structured like this:

[me]: http://docs.mongodb.org/manual/tutorial/map-reduce-examples/

```py
{
    "customer_id":  "cs11072",
    "date": r.time(2014, 27, 2, 12, 13, 09, '-07:00'),
    "id": 103,
    "items": [
        {
            "price": 91,
            "quantity": 1,
            "item_id":  "sku10491"
        } ,
        {
            "price": 9,
            "quantity": 3,
            "item_id":  "sku14667"
        } ,
        {
            "price": 37 ,
            "quantity": 3,
            "item_id":  "sku16857"
        }
    ],
    "total": 229
}
```

First, let's return the total price per customer. Since this is pre-computed per order in the `total` field, this is easily done with one of RethinkDB's aggregation functions.

```py
r.table('orders').group('customer_id').sum('total').run(conn)
```

Now for something more complicated: calculating the total and average quantities sold per item. For this, we'll use the [concat_map][] function, which combines mapping and concatenation together. In this case, we want to produce a sequence of all the items sold throughout all the orders with their item IDs and quantities. We'll also add a "count" field set to `1`; we'll use this the same way we used the mapping of each post in the blog example.

```py
r.table('orders').concat_map(lambda order:
    order['items'].map(lambda item:
        {'item_id': item['item_id'], 'quantity': item['quantity']. 'count': 1}
    ))
```

The inner `map` function is just being used to iterate through the items in each order. At this point, our query will return a list of objects, each object with three fields: `item_id`, `quantity` and `count`.

Now, we'll `group` by the `item_id` field and use a custom `reduce` function to sum the quantities and counts.

```py
r.table('orders').concat_map(lambda order:
    order['items'].map(lambda item:
        {'item_id': item['item_id'], 'quantity': item['quantity']. 'count': 1}
    )).group('item_id').reduce(lambda left, right: {
        'item_id': left['item_id'],
        'quantity': left['quantity'] + right['quantity'],
        'count': left['count'] + right['count']
    })
```

Finally, we'll use [ungroup][] to turn this grouped data into an array of objects with `group` and `reduction` keys. The `group` field will be the item ID for each group; the `reduction` field will have all the items from the `concat_map` function that belong to each group. Then we'll use `map` once more to iterate through that array, computing the average on this pass.

```py
r.table('orders').concat_map(lambda order:
    order['items'].map(lambda item:
        {'item_id': item['item_id'], 'quantity': item['quantity']. 'count': 1}
    )).group('item_id').reduce(lambda left, right: {
        'item_id': left['item_id'],
        'quantity': left['quantity'] + right['quantity'],
        'count': left['count'] + right['count']
    }).ungroup().map(lambda group: {
        'item_id': group['group'],
        'quantity': group['reduction']['quantity'],
        'avg': group['reduction']['quantity'] / group['reduction']['count']
    }).run(conn)
```

The output will be in this format:

```py
[
    {
        "avg": 3.3333333333333,
        "quantity": 20,
        "item_id": "sku10023"
    },
    {
        "avg": 2.2142857142857,
        "quantity": 31,
        "item_id": "sku10042"
    },
    ...
]
```

(Note that Javascript, or another language where `+` and `/` operators aren't overridden to work with ReQL, will require you to use [div](/api/javascript/div) and [add](/api/javascript/add).)

# How GMR queries are executed #

RethinkDB's GMR queries are distributed and parallelized across shards and CPU cores whenever possible. While this allows them to execute efficiently, it's important to keep in mind that the `reduce` function is *not* called on the elements of its input stream from left to right. It's called on either the elements of the stream *in any order* or on the output of previous calls to the function.

Here is an example of an __incorrect__ way to write the previous grouped map/reduce query, simply incrementing the first value passed to the reduction function:

```py
# Incorrect!
r.table('posts').group(lambda post: post['category']).map(
    lambda post: 1).reduce(lambda a, b: a + 1).run(conn)
```

Suppose we have ten documents in a single category in a sharded table. Four of the documents are on shard 1; six are on shard 2. When the incorrect query is executed, this is its path:

1. The number of documents on shard 1 is computed. The query returns the value `4` for the shard.
2. The number of documents on shard 2 is computed. The query returns the value `6` for the shard.
3. The final reduction step is executed to combine the values of the two shards. Instead of computing `4 + 6`, the query executes `4 + 1`.

{% infobox %}

__Be careful!__ Make sure your reduction function doesn't assume the
reduction step executes from left to right!

{% endinfobox %}

# Read more #

For more information about MapReduce in general, read the [Wikipedia article][wp]. For more information about RethinkDB's implementation, browse our API documentation.

* [group][]
* [map][]
* [reduce][]
* [ungroup][]
* [concat_map][]

[group]: /api/python/group/
[map]: /api/python/map/
[reduce]: /api/python/reduce/
[ungroup]: /api/python/ungroup/
[concat_map]: /api/python/concat_map
