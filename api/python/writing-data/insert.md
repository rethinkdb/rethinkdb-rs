---
layout: api-command
language: Python
permalink: api/python/insert/
command: insert
related_commands:
    update: update/
    replace: replace/
    delete: delete/
---


# Command syntax #

{% apibody %}
table.insert(json | [json][, durability="hard", return_vals=False, upsert=False])
    &rarr; object
{% endapibody %}

# Description #

<img src="/assets/images/docs/api_illustrations/insert_python.png" class="api_command_illustration" />

Insert documents into a table. Accepts a single document or an array of
documents.

The optional arguments are:

- `durability`: possible values are `hard` and `soft`. This option will override the
table or query's durability setting (set in [run](/api/python/run/)).  
In soft durability mode RethinkDB will acknowledge the write immediately after
receiving it, but before the write has been committed to disk.
- `return_vals`: if set to `True` and in case of a single insert/upsert, the inserted/updated
document will be returned.
- `upsert`: when set to `True`, performs a [replace](/api/python/replace/) if a
document with the same primary key exists.


Insert returns an object that contains the following attributes:

- `inserted`: the number of documents that were succesfully inserted.
- `replaced`: the number of documents that were updated when upsert is used.
- `unchanged`: the number of documents that would have been modified, except that the
new value was the same as the old value when doing an upsert.
- `errors`: the number of errors encountered while performing the insert.
- `first_error`: If errors were encountered, contains the text of the first error.
- `deleted` and `skipped`: 0 for an insert operation.
- `generated_keys`: a list of generated primary keys in case the primary keys for some
documents were missing (capped to 100000).
- `warnings`: if the field `generated_keys` is truncated, you will get the warning _"Too
many generated keys (&lt;X&gt;), array truncated to 100000."_.
- `old_val`: if `return_vals` is set to `True`, contains `None`.
- `new_val`: if `return_vals` is set to `True`, contains the inserted/updated document.



__Example:__ Insert a document into the table `posts`.

```py
r.table("posts").insert({
    "id": 1,
    "title": "Lorem ipsum",
    "content": "Dolor sit amet"
}).run(conn)
```

The result will be:

```py
{
    "deleted": 0,
    "errors": 0,
    "inserted": 1,
    "replaced": 0,
    "skipped": 0,
    "unchanged": 0
}
```


__Example:__ Insert a document without a defined primary key into the table `posts` where the
primary key is `id`.

```py
r.table("posts").insert({
    "title": "Lorem ipsum",
    "content": "Dolor sit amet"
}).run(conn)
```

RethinkDB will generate a primary key and return it in `generated_keys`.

```py
{
    "deleted": 0,
    "errors": 0,
    "generated_keys": [
        "dd782b64-70a7-43e4-b65e-dd14ae61d947"
    ],
    "inserted": 1,
    "replaced": 0,
    "skipped": 0,
    "unchanged": 0
}
```

Retrieve the document you just inserted with:

```py
r.table("posts").get("dd782b64-70a7-43e4-b65e-dd14ae61d947").run(conn)
```

And you will get back:

```py
{
    "id": "dd782b64-70a7-43e4-b65e-dd14ae61d947",
    "title": "Lorem ipsum",
    "content": "Dolor sit amet",
}
```


__Example:__ Insert multiple documents into the table `users`.

```py
r.table("users").insert([
    {"id": "william", "email": "william@rethinkdb.com"},
    {"id": "lara", "email": "lara@rethinkdb.com"}
]).run(conn)
```


__Example:__ Insert a document into the table `users`, replacing the document if the document
already exists.  
_Note_: If the document exists, the `insert` command will behave like [replace](/api/python/replace/), not like [update](/api/python/update/) 

```py
r.table("users").insert(
    {"id": "william", "email": "william@rethinkdb.com"},
    upsert=True
).run(conn)
```


__Example:__ Copy the documents from `posts` to `posts_backup`.

```py
r.table("posts_backup").insert( r.table("posts") ).run(conn)
```


__Example:__ Get back a copy of the inserted document (with its generated primary key).

```py
r.table("posts").insert(
    {"title": "Lorem ipsum", "content": "Dolor sit amet"},
    return_vals=True
).run(conn)
```

The result will be

```py
{
    "deleted": 0,
    "errors": 0,
    "generated_keys": [
        "dd782b64-70a7-43e4-b65e-dd14ae61d947"
    ],
    "inserted": 1,
    "replaced": 0,
    "skipped": 0,
    "unchanged": 0,
    "old_val": None,
    "new_val": {
        "id": "dd782b64-70a7-43e4-b65e-dd14ae61d947",
        "title": "Lorem ipsum",
        "content": "Dolor sit amet"
    }
}
```
