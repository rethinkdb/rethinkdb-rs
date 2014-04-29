---
layout: api-command
language: Ruby
permalink: api/ruby/order_by/
command: order_by
related_commands:
    skip: skip/
    limit: limit/
    '[]': slice/
---

# Command syntax #

{% apibody %}
table.order_by([key1...], :index => index_name) -> selection<stream>
selection.order_by(key1, [key2...]) -> selection<array>
sequence.order_by(key1, [key2...]) -> array
{% endapibody %}

# Description #

Sort the sequence by document values of the given key(s).

Sorting without an index is limited to 100,000 documents because it requires the server
to hold the whole sequence in memory. Sorting with an index can be done only on a table
or after a `between` command using the same index. The `order_by` command defaults to
ascending ordering. To explicitly specify the ordering, wrap the attribute with either
`r.asc` or `r.desc`.

__Example:__ Order all the posts using the index `date`.   

```rb
r.table('posts').order_by(:index => 'date').run(conn)
```

The index must have been previously created with [index_create](/api/ruby/index_create/).

```rb
r.table('posts').index_create('date').run(conn)
```


__Example:__ Because indexes can be created on arbitrary functions, you can efficiently
sort your data with arbitrary expressions.

```rb
r.table('posts').order_by(:index => 'votes').run(conn)
```

The index must have been previously created with [index_create](/api/ruby/index_create/).

```rb
r.table('posts').index_create('votes') {|post|
    post["upvotes"]-post["downvotes"]
}.run(conn)
```

__Example:__ If you have a sequence with less than 100.000 documents, you can sort it
without an index.   
Return the comments of the post with `id` of `1`, ordered by date.

```rb
r.table("posts").get(1)["comments"].order_by("date")
```

__Example:__ You can efficiently order using multiple fields by using a
[compound index](http://www.rethinkdb.com/docs/secondary-indexes/python/).  
Order by date and title.

```rb
r.table('posts').order_by(:index => 'date_and_title').run(conn)
```

The index must have been previously created with [index_create](/api/ruby/index_create/).

```rb
r.table('posts').index_create('date_and_title') {|post| [post["date"], post["title"]]}.run(conn)
```

__Example:__ Notice that an index ordering always has highest precedence.    
So the following query orders post by date, and if multiple posts were published on the
same date, they will be ordered by title.

```rb
r.table('post').order_by(:title, :index => 'date').run(conn)
```

__Example:__ You can also specify a descending order when using an index.

```rb
r.table('post').order_by(:index => r.desc('date')).run(conn)
```

_Note_: You cannot specify multiple orders in a compound index. See [issue #2306](https://github.com/rethinkdb/rethinkdb/issues/2306)
to track progress.


__Example:__ Ordering after a `between` command can be done as long as the same index is being used.

```rb
r.table("posts").between(r.time(2013, 1, 1, '+00:00'), r.time(2013, 1, 1, '+00:00'), :index => "date")
    .order_by(:index => "date").run(conn);
```

__Example:__ If you have a sequence with less than 100.000 documents, you can sort it with an arbitrary function.   
Return the comments of the post with `id` of `1`, ordered by the sum of `upvotes` minus the sum of `downvotes`.

```rb
r.table("posts").get(1)["comments"].order_by(lambda { |comment|
    comment["upvotes"]-comment["downvotes"]
});
```

__Example:__ Functions can also be used descendingly.

```rb
r.table("posts").get(1)["comments"].order_by(r.desc(lambda {|comment|
    comment["upvotes"]-comment["downvotes"]
}));
```


