---
layout: api-command
language: Java
permalink: api/java/delete/
command: delete
related_commands:
    insert: insert/
    update: update/
    replace: replace/
---

# Command syntax #

{% apibody %}
table.delete([{durability: "hard", returnChanges: false}])
    &rarr; object
selection.delete([{durability: "hard", returnChanges: false}])
    &rarr; object
singleSelection.delete([{durability: "hard", returnChanges: false}])
    &rarr; object
{% endapibody %}

<img src="/assets/images/docs/api_illustrations/delete-vector.png" class="api_command_illustration" />

# Description #

Delete one or more documents from a table.

The optional arguments are:

- `durability`: possible values are `hard` and `soft`. This option will override the
table or query's durability setting (set in [run](/api/java/run/)).  
In soft durability mode RethinkDB will acknowledge the write immediately after
receiving it, but before the write has been committed to disk.
- `returnChanges`:
    - `true`: return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made, only including the documents actually updated.
    - `false`: do not return a `changes` array (the default).
    - `"always"`: behave as `true`, but include all documents the command tried to update whether or not the update was successful. (This was the behavior of `true` pre-2.0.)


Delete returns an object that contains the following attributes:

- `deleted`: the number of documents that were deleted.
- `skipped`: the number of documents that were skipped.  
For example, if you attempt to delete a batch of documents, and another concurrent query
deletes some of those documents first, they will be counted as skipped.
- `errors`: the number of errors encountered while performing the delete.
- `first_error`: If errors were encountered, contains the text of the first error.
- `inserted`, `replaced`, and `unchanged`: all 0 for a delete operation..
- `changes`: if `returnChanges` is set to `true`, this will be an array of objects, one for each objected affected by the `delete` operation. Each object will have two keys: `{new_val: null, old_val: <old value>}`.


__Example:__ Delete a single document from the table `comments`.

```js
r.table("comments").get("7eab9e63-73f1-4f33-8ce4-95cbea626f59").delete().run(conn)
```


__Example:__ Delete all documents from the table `comments`.

```js
r.table("comments").delete().run(conn)
```


__Example:__ Delete all comments where the field `idPost` is `3`.

```js
r.table("comments").filter({idPost: 3}).delete().run(conn)
```


__Example:__ Delete a single document from the table `comments` and return its value.

```js
r.table("comments").get("7eab9e63-73f1-4f33-8ce4-95cbea626f59").delete({returnChanges: true}).run(conn)
```

The result look like:

```js
{
    deleted: 1,
    errors: 0,
    inserted: 0,
    changes: [
        {
            new_val: null,
            old_val: {
                id: "7eab9e63-73f1-4f33-8ce4-95cbea626f59",
                author: "William",
                comment: "Great post",
                idPost: 3
            }
        }
    ],
    replaced: 0,
    skipped: 0,
    unchanged: 0
}
```


__Example:__ Delete all documents from the table `comments` without waiting for the
operation to be flushed to disk.

```js
r.table("comments").delete({durability: "soft"}).run(conn)
```
