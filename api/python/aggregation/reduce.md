---
layout: api-command
language: Python
permalink: api/python/reduce/
command: reduce
related_commands:
    map: map/
    concat_map: concat_map/
    grouped_map_reduce: grouped_map_reduce/
    group_by: group_by/
---

# Command syntax #

{% apibody %}
sequence.reduce(reduction_function[, default]) &rarr; value
{% endapibody %}

# Description #

Produce a single value from a sequence through repeated application of a reduction
function.

The `reduce` method is distributed and parallelized across shards and CPU cores.
This allows map/reduce queries to execute efficiently, but is a source of a common
mistake: assuming an incorrect reduction order.  
Read the [map-reduce in RethinkDB](/docs/map-reduce/) article if you are not familiar with
map/reduce.

The `default` value is returned only if you reduce an empty sequence.


__Example:__ Return the number of documents in the table `posts.

```py
r.table("posts").map(lambda doc:
    1
).reduce(lambda left, right:
    left+right , 0).run(conn);
```


A shorter way to execute this query is to use [count](/api/python/count).


__Example:__ Suppose that each `post` has a field `comments` that is an array of
comments.  
Return the number of comments for all posts.

```py
r.table("posts").map(lambda doc:
    doc["comments"].count()
).reduce(lambda left, right:
    left+right, 0).run(conn);
```


__Example:__ Suppose that each `post` has a field `comments` that is an array of
comments.  
Return the maximum number comments per post.

```py
r.table("posts").map(lambda doc:
    doc["comments"].count()
).reduce(lambda left, right:
    r.branch(
        left > right,
        left,
        right
    ), 0).run(conn);
```

