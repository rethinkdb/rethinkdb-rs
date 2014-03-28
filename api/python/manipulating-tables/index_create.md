---
layout: api-command
language: Python
permalink: api/python/index_create/
command: index_create
related_commands:
    index_wait: index_wait/
    index_status: index_status/
    index_list: index_list/
    index_drop: index_drop/
---

# Command syntax #

{% apibody %}
table.index_create(index_name[, index_function][, multi=True]) &rarr; object
{% endapibody %}

# Description #

Create a new secondary index on this table.

RethinkDB supports different types of secondary indexes:

- Simple indexes based on the value of a single field.
- Compound indexes based on multiple fields.
- Multi indexes based on arrays of values.
- Indexes based on arbitrary expressions.

If you are not familiar with secondary indexes, read
[the article about secondary indexes](http://www.rethinkdb.com/docs/secondary-indexes/)
to learn more about them.

__Example:__ Create a simple index based on the field `post_id`.

```py
r.table('comments').index_create('post_id').run(conn)
```
__Example:__ Create a simple index based on the nested field `author > name`.


```py
r.table('comments').index_create('author_name', r.row["author"]["name"]).run(conn)
```


__Example:__ Create a compount index based on the fields `post_id` and `date`.

```py
r.table('comments').index_create('post_and_date', [r.row["post_id"], r.row["date"]]).run(conn)
```
__Example:__ Create a multi index based on the field `authors`.


__Example:__ A multi index can be created by passing an optional multi argument. Multi
index functions should return arrays and allow you to query based on whether a value
is present in the returned array. The example would allow us to get heroes who possess a
specific ability (the field 'abilities' is an array).

```py
r.table('posts').index_create('authors', multi=True).run(conn)
```

__Example:__ Create a multi index based on an arbitrary expression.

```py
r.table('posts').index_create('authors', lambda doc:
    r.branch(
        doc.has_fields("updated_at"),
        doc["updated_at"],
        doc["created_at"]
    )
).run(conn)
```
