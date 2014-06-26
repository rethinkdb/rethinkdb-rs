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

Sort the sequence by document values of the given key(s). To specify
the ordering, wrap the attribute with either `r.asc` or `r.desc`
(defaults to ascending).

Sorting without an index requires the server to hold the sequence in
memory, and is limited to 100,000 documents. Sorting with an index can
be done on arbitrarily large tables, or after a `between` command
using the same index.

__Example:__ Order all the posts using the index `date`.   

```rb
r.table('posts').order_by(:index => 'date').run(conn)
```

The index must have been previously created with [index_create](/api/ruby/index_create/).

```rb
r.table('posts').index_create('date').run(conn)
```

You can also select a descending ordering:

```rb
r.table('posts').order_by(:index => r.desc('date')).run(conn, callback)
```


__Example:__ Order a sequence with less than 100,000 documents
without an index.

```rb
r.table('posts').get(1)['comments'].order_by('date')
```

You can also select a descending ordering:

```rb
r.table('posts').get(1)['comments'].order_by(r.desc('date'))
```

If you're doing ad-hoc analysis and know your table won't have more then 100,000
elements you can run `order_by` without an index:

```rb
r.table('small_table').order_by('date')
```

__Example:__ You can efficiently order using multiple fields by using a
[compound index](http://www.rethinkdb.com/docs/secondary-indexes/ruby/).  

Order by date and title.

```rb
r.table('posts').order_by(:index => 'date_and_title').run(conn)
```

The index must have been previously created with [index_create](/api/ruby/index_create/).

```rb
r.table('posts').index_create('date_and_title') {|post| [post["date"], post["title"]]}.run(conn)
```

_Note_: You cannot specify multiple orders in a compound index. See [issue #2306](https://github.com/rethinkdb/rethinkdb/issues/2306)
to track progress.

__Example:__ If you have a sequence with less than 100,000 documents, you can order it
by multiple fields without an index.

```rb
r.table('small_table').order_by('date', r.desc('title'))
```

__Example:__ Notice that an index ordering always has highest
precedence. The following query orders posts by date, and if multiple
posts were published on the same date, they will be ordered by title.

```rb
r.table('post').order_by(:title, :index => 'date').run(conn)
```

__Example:__ You can use [nested field](/docs/cookbook/ruby/#filtering-based-on-nested-fields) syntax to sort on fields from subdocuments. (You can also create indexes on nested fields using this syntax with `index_create`.)

```rb
r.table('user').order_by{ |user| user['group']['id'] }.run(conn)
```

__Example:__ You can efficiently order data on arbitrary expressions using indexes.

```rb
r.table('posts').order_by(:index => 'votes').run(conn)
```

The index must have been previously created with [index_create](/api/ruby/index_create/).

```rb
r.table('posts').index_create('votes') {|post|
    post["upvotes"]-post["downvotes"]
}.run(conn)
```

__Example:__ If you have a sequence with less than 100,000 documents, you can order it with an arbitrary function directly.

```rb
r.table('small_table').order_by(lambda { |doc|
    doc['upvotes']-doc['downvotes']
});
```

You can also select a descending ordering:

```js
r.table('small_table').order_by(r.desc(lambda { |doc|
    doc['upvotes']-doc['downvotes']
}));
```

__Example:__ Ordering after a `between` command can be done as long as the same index is being used.

```rb
r.table("posts").between(r.time(2013, 1, 1, '+00:00'), r.time(2013, 1, 1, '+00:00'), :index => 'date')
    .order_by(:index => 'date').run(conn);
```


