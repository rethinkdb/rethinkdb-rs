---
layout: api-command
language: Ruby
permalink: api/ruby/reduce/
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
The reduction function can be called on:

- two elements of the sequence
- one element of the sequence and one result of a previous reduction
- two results of previous reductions

The reduction function can be called on the results of two previous reductions because the
`reduce` command is distributed and parallelized across shards and CPU cores. A common
mistaken when using the `reduce` command is to suppose that the reduction is executed
from left to right. Read the [map-reduce in RethinkDB](/docs/map-reduce/) article to
see an example.

The `default` value is returned only if you reduce an empty sequence.


__Example:__ Return the numbers of documents in the table `posts`.

```rb
r.table("posts").map{|doc| 1 }.reduce(0), { left, right: left+right }.run(conn)
```

A shorter way to execute this query is to use [count](/api/ruby/count).


__Example:__ Suppose that each `post` has a field `comments` that is an array of
comments.  
Return the number of comments for all posts.

```rb
r.table("posts").map{|doc|
    doc["comments"].count()
}.reduce(0), { left, right:
    left+right
}.run(conn)
```


__Example:__ Suppose that each `post` has a field `comments` that is an array of
comments.  
Return the maximum number comments per post.

```rb
r.table("posts").map{|doc|
    doc["comments"].count()
}.reduce(0) { left, right:
    r.branch(
        left > right,
        left,
        right
    )
}.run(conn)
```

