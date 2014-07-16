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
table.index_create(index_name[, index_function][, :multi => true]) &rarr; object
{% endapibody %}

# Description #

Create a new secondary index on a table. Secondary indexes improve the speed of many read queries at the slight cost of increased storage space and decreased write performance. For more information about secondary indexes, read the article "[Using secondary indexes in RethinkDB](/docs/secondary-indexes/)."

RethinkDB supports different types of secondary indexes:

- *Simple indexes* based on the value of a single field.
- *Compound indexes* based on multiple fields.
- *Multi indexes* based on arrays of values.
- Indexes based on *arbitrary expressions*.

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

__Example:__ Create a multi index based on an arbitrary expression.

```rb
r.table('posts').index_create('authors'){ |doc|
    r.branch(
        doc.has_fields("updated_at"),
        doc["updated_at"],
        doc["created_at"]
    )
}.run(conn)
```
