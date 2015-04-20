---
layout: api-command
language: Python
permalink: api/python/update/
command: update
related_commands:
    insert: insert/
    replace: replace/
    delete: delete/
---

# Command syntax #

{% apibody %}
table.update(object | expr[, durability="hard", return_changes=False, non_atomic=False])
    &rarr; object
selection.update(object | expr[, durability="hard", return_changes=False, non_atomic=False])
    &rarr; object
singleSelection.update(object | expr[, durability="hard", return_changes=False, non_atomic=False])
    &rarr; object
{% endapibody %}

# Description #

Update JSON documents in a table. Accepts a JSON document, a ReQL expression, or a
combination of the two.

The optional arguments are:

- `durability`: possible values are `hard` and `soft`. This option will override the
table or query's durability setting (set in [run](/api/python/run/)).  
In soft durability mode RethinkDB will acknowledge the write immediately after
receiving it, but before the write has been committed to disk.
- `return_changes`: if set to `True`, return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made.
- `non_atomic`: if set to `True`, executes the update and distributes the result to replicas in a non-atomic fashion. This flag is required to perform non-deterministic updates, such as those that require reading data from another table.

Update returns an object that contains the following attributes:

- `replaced`: the number of documents that were updated.
- `unchanged`: the number of documents that would have been modified except the new
value was the same as the old value.
- `skipped`: the number of documents that were skipped because the document didn't exist.
- `errors`: the number of errors encountered while performing the update.
- `first_error`: If errors were encountered, contains the text of the first error.
- `deleted` and `inserted`: 0 for an update operation.
- `changes`: if `return_changes` is set to `True`, this will be an array of objects, one for each objected affected by the `update` operation. Each object will have two keys: `{"new_val": <new value>, "old_val": <old value>}`.


__Example:__ Update the status of the post with `id` of `1` to `published`.

```py
r.table("posts").get(1).update({"status": "published"}).run(conn)
```

__Example:__ Update the status of all posts to `published`.

```py
r.table("posts").update({"status": "published"}).run(conn)
```

__Example:__ Update the status of all the post written by William.

```py
r.table("posts").filter({"author": "William"}).update({"status": "published"}).run(conn)
```


__Example:__ Increment the field `view` with `id` of `1`.
This query will throw an error if the field `views` doesn't exist.

```py
r.table("posts").get(1).update({
    "views": r.row["views"]+1
}).run(conn)
```

__Example:__ Increment the field `view` of the post with `id` of `1`.
If the field `views` does not exist, it will be set to `0`.

```py
r.table("posts").update({
    "views": (r.row["views"]+1).default(0)
}).run(conn)
```

__Example:__ Perform a conditional update.  
If the post has more than 100 views, set the `type` of a post to `hot`, else set it to `normal`.

```py
r.table("posts").get(1).update(lambda post:
    r.branch(
        post["views"] > 100,
        {"type": "hot"},
        {"type": "normal"}
    )
).run(conn)
```

__Example:__ Update the field `num_comments` with the result of a sub-query. Because
this update is not atomic, you must pass the `non_atomic` flag.

```py
r.table("posts").get(1).update({
    "num_comments": r.table("comments").filter({"id_post": 1}).count()
}, non_atomic=True ).run(conn)
```

If you forget to specify the `non_atomic` flag, you will get a `RqlRuntimeError`.

```
RqlRuntimeError: Could not prove function deterministic.  Maybe you want to use the non_atomic flag? 
```

__Example:__ Update the field `num_comments` with a random value between 0 and 100.  
This update cannot be proven deterministic because of `r.js` (and in fact is not), so you
must pass the `non_atomic` flag.

```py
r.table("posts").get(1).update({
    "num_comments": r.js("Math.floor(Math.random()*100)")
}, non_atomic=True ).run(conn)
```

__Example:__ Update the status of the post with `id` of `1` using soft durability.

```py
r.table("posts").get(1).update({status: "published"}, durability="soft").run(conn)
```

__Example:__ Increment the field `views` and return the values of the document before
and after the update operation.

```py
r.table("posts").get(1).update({
    "views": r.row["views"]+1
}, return_changes=True).run(conn)
```

The result will have a `changes` field:

```py
{
    "deleted": 1,
    "errors": 0,
    "inserted": 0,
    "changes": [
        {
            "new_val": {
                "id": 1,
                "author": "Julius_Caesar",
                "title": "Commentarii de Bello Gallico",
                "content": "Aleas jacta est",
                "views": 207
            },
            "old_val": {
                "id": 1,
                "author": "Julius_Caesar",
                "title": "Commentarii de Bello Gallico",
                "content": "Aleas jacta est",
                "views": 206
            }
        }
    ],
    "replaced": 0,
    "skipped": 0,
    "unchanged": 0
}
```

