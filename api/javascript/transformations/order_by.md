---
layout: api-command
language: JavaScript
permalink: api/javascript/order_by/
command: orderBy
io:
    -   - sequence
        - stream
    -   - array
        - array
related_commands:
    skip: skip/
    limit: limit/
    slice: slice/
---

# Command syntax #

{% apibody %}
table.orderBy([key1...], {index: index_name}) -> selection<stream>
selection.orderBy(key1, [key2...]) -> selection<array>
sequence.orderBy(key1, [key2...]) -> array
{% endapibody %}

# Description #

Sort the sequence by document values of the given key(s).

Sorting without an index is limited to 100,000 documents because it requires the server
to hold the whole sequence in memory. Sorting with an index can be done only on a table
or after a `between` command using the same index. The `orderBy` command defaults to
ascending ordering. To explicitly specify the ordering, wrap the attribute with either
`r.asc` or `r.desc`.



__Example:__ Order all the posts using the index `date`.   

```js
r.table('posts').orderBy({index: 'date'}).run(conn, callback)
```

The index must have been previously created with [indexCreate](/api/javascript/index_create/).

```js
r.table('posts').indexCreate('date').run(conn, callback)
```


__Example:__ Because indexes can be created on arbitrary functions, you can efficiently
sort your data with arbitrary expressions.

```js
r.table('posts').orderBy({index: 'votes'}).run(conn, callback)
```

The index must have been previously created with [indexCreate](/api/javascript/index_create/).

```js
r.table('posts').indexCreate('votes', function(post) {
    return post("upvotes").sub(post("downvotes"))
}).run(conn, callback)
```

__Example:__ If you have a sequence with less than 100.000 documents, you can sort it
without an index.  
Return the comments of the post with `id` of `1`, ordered by date.

```js
r.table("posts").get(1)("comments").orderBy("date")
```



__Example:__ You can efficiently order using multiple fields by using a
[compound index](http://www.rethinkdb.com/docs/secondary-indexes/javascript/).  
Order by date and title.

```js
r.table('posts').orderBy({index: 'dateAndTitle'}).run(conn, callback)
```

The index must have been previously created with [indexCreate](/api/javascript/index_create/).

```js
r.table('posts').indexCreate('dateAndTitle', [r.row("date"), r.row("title")]).run(conn, callback)
```

__Example:__ Notice that an index ordering always has highest precedence.    
So the following query orders post by date, and if multiple posts were published on the
same date, they will be ordered by title.

```js
r.table('post').orderBy('title', {index: 'date'}).run(conn, callback)
```

__Example:__ You can also specify a descending order when using an index.

```js
r.table('post').orderBy({index: r.desc('date')}).run(conn, callback)
```

_Note_: You cannot specify multiple orders in a compound index. See [issue #2306](https://github.com/rethinkdb/rethinkdb/issues/2306)
to track progress.


__Example:__ Ordering after a `between` command can be done as long as the same index is being used.

```js
r.table("posts").between(r.time(2013, 1, 1, '+00:00'), r.time(2013, 1, 1, '+00:00'), {index: "date"})
    .orderBy({index: "date"}).run(conn, callback);
```

__Example:__ If you have a sequence with less than 100.000 documents, you can sort it with an arbitrary function.   
Return the comments of the post with `id` of `1`, ordered by the sum of `upvotes` minus the sum of `downvotes`.

```js
r.table("posts").get(1)("comments").orderBy(function(comment) {
    return comment("upvotes").sub(comment("downvotes"))
});
```

__Example:__ Functions can also be used descendingly.

```js
r.table("posts").get(1)("comments").orderBy(r.desc(function(comment) {
    return comment("upvotes").sub(comment("downvotes"))
}));
```


