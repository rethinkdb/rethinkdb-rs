---
layout: api-command
language: JavaScript
permalink: api/javascript/reduce/
command: reduce
io:
    -   - sequence
        - value
related_commands:
    map: map/
    concat_map: concat_map/
    groupedMapReduce: grouped_map_reduce/
    groupBy: group_by/
---

# Command syntax #

{% apibody %}
sequence.reduce(reductionFunction[, default]) &rarr; value
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


__Example:__ Return the number of documents in the table `posts`.

```js
r.table("posts").map(function(doc) {
    return 1;
}).reduce(function(left, right) {
    return left.add(right);
}, 0).run(conn, callback);
```

A shorter way to execute this query is to use [count](/api/javascript/count).



__Example:__ Suppose that each `post` has a field `comments` that is an array of
comments.  
Return the number of comments for all posts.

```js
r.table("posts").map(function(doc) {
    return doc("comments").count();
}).reduce(function(left, right) {
    return left.add(right);
}, 0).run(conn, callback);
```



__Example:__ Suppose that each `post` has a field `comments` that is an array of
comments.  
Return the maximum number comments per post.

```js
r.table("posts").map(function(doc) {
    return doc("comments").count();
}).reduce(function(left, right) {
    return r.branch(
        left.gt(right),
        left,
        right
    );
}, 0).run(conn, callback);
```
