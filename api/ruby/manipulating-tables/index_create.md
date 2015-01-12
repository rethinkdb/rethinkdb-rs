---
layout: api-command
language: Ruby
permalink: api/ruby/index_create/
command: index_create
related_commands:
    index_wait: index_wait/
    index_status: index_status/
    index_list: index_list/
    index_drop: index_drop/
---

# Command syntax #

{% apibody %}
table.index_create(index_name[, index_function][, :multi => false, :geo => false]) &rarr; object
{% endapibody %}

# Description #

Create a new secondary index on a table. Secondary indexes improve the speed of many read queries at the slight cost of increased storage space and decreased write performance. For more information about secondary indexes, read the article "[Using secondary indexes in RethinkDB](/docs/secondary-indexes/)."

RethinkDB supports different types of secondary indexes:

- *Simple indexes* based on the value of a single field.
- *Compound indexes* based on multiple fields.
- *Multi indexes* based on arrays of values.
- *Geospatial indexes* based on indexes of geometry objects, created when the `geo` optional argument is true.
- Indexes based on *arbitrary expressions*.

The `index_function` can be an anonymous function or a binary representation obtained from the `function` field of [index_status](/api/ruby/index_status).

If successful, `create_index` will return an object of the form `{:created => 1}`. If an index by that name already exists on the table, a `RqlRuntimeError` will be thrown.

__Example:__ Create a simple index based on the field `post_id`.

```rb
r.table('comments').index_create('post_id').run(conn)
```
__Example:__ Create a simple index based on the nested field `author > name`.


```rb
r.table('comments').index_create('author_name'){ |doc|
    doc["author"]["name"]
}.run(conn)
```

__Example:__ Create a geospatial index based on the field `location`.

```rb
r.table('places').index_create('location', :geo => true).run(conn)
```

A geospatial index field should contain only geometry objects. It will work with geometry ReQL terms ([get_intersecting](/api/ruby/get_intersecting/) and [get_nearest](/api/ruby/get_nearest/)) as well as index-specific terms ([index_status](/api/ruby/index_status), [index_wait](/api/ruby/index_wait), [index_drop](/api/ruby/index_drop) and [index_list](/api/ruby/index_list)). Using terms that rely on non-geometric ordering such as [get_all](/api/ruby/get_all/), [order_by](/api/ruby/order_by/) and [between](/api/ruby/order_by/) will result in an error.


__Example:__ Create a compound index based on the fields `post_id` and `date`.

```rb
r.table('comments').index_create('post_and_date'){ |doc|
    [doc["post_id"], doc["date"]]
}.run(conn)
```

__Example:__ Create a multi index based on the field `authors`.

```rb
r.table('posts').index_create('authors', :multi=>true).run(conn)
```

__Example:__ Create a geospatial multi index based on the field `towers`.

```rb
r.table('networks').index_create('towers', :geo=>true, :multi=>true).run(conn)
```

__Example:__ Create an index based on an arbitrary expression.

```rb
r.table('posts').index_create('authors'){ |doc|
    r.branch(
        doc.has_fields("updated_at"),
        doc["updated_at"],
        doc["created_at"]
    )
}.run(conn)
```

__Example:__ Create a new secondary index based on an existing one.

```rb
index = r.table('posts').index_status('authors').nth(0)['function'].run(conn)
r.table('new_posts').index_create('authors', index).run(conn)
```

__Example:__ Rebuild an outdated secondary index on a table.

```rb
old_index = r.table('posts').index_status('old_index').nth(0)['function'].run(conn)
r.table('posts').index_create('new_index', old_index).run(conn)
r.table('posts').index_wait('new_index').run(conn)
r.table('posts').index_rename('new_index', 'old_index', {:overwrite => true}).run(conn)
```

