---
layout: api-command
language: JavaScript
permalink: api/javascript/delete/
command: delete
io:
    -   - table
        - object
    -   - selection
        - object
    -   - singleSelection
        - object
related_commands:
    insert: insert/
    update: update/
    replace: replace/
---

# Command syntax #

{% apibody %}
table.delete([{durability: "hard", returnVals: false}])
    &rarr; object
selection.delete([{durability: "hard", returnVals: false}])
    &rarr; object
singleSelection.delete([{durability: "hard", returnVals: false}])
    &rarr; object
{% endapibody %}

# Description #

Delete one or more documents from a table.

The optional arguments are:

- `returnVals`: in case of a single row deletion, the deleted row will be returned.
- `durability`: Possible values are `hard` and `soft`. It will override the table or
query's default durability setting.

Delete returns an object that contains the following attributes:

- `deleted`: the number of documents that were deleted.
- `skipped`: the number of documents that were skipped.  
For example, if you delete a row that has already been deleted, that row will be skipped.
- `errors`: the number of errors encountered while deleting.  
If errors where encountered while deleting, `first_error` contains the text of the first
error.
- `inserted`, `replaced`, and `unchanged`: all 0 for a delete operation..
- `old_val`: the deleted row if `returnVals` was set to true.
- `new_val`: `null` if `returnVals` was set to true.


__Example:__ Delete a single row from the table `comments`.

```js
r.table("comments").get("7eab9e63-73f1-4f33-8ce4-95cbea626f59").delete().run(conn, callback)
```


__Example:__ Delete all documents from the table `comments`.

```js
r.table("comments").delete().run(conn, callback)
```


__Example:__ Delete all comments where the field `idPost` is `3`.

```js
r.table("comments").filter({idPost: 3}).delete().run(conn, callback)
```


__Example:__ Delete a single row from the table `comments` and return its value.

```js
r.table("comments").get("7eab9e63-73f1-4f33-8ce4-95cbea626f59").delete({returnVals: true}).run(conn, callback)
```

The result look like:

```js
{
    "deleted": 1,
    "errors": 0,
    "inserted": 0,
    "new_val": null,
    "old_val": {
        "id": "7eab9e63-73f1-4f33-8ce4-95cbea626f59",
        "author": "William",
        "comment": "Great post",
        "idPost": 3
    },
    "replaced": 0,
    "skipped": 0,
    "unchanged": 0
}
```


__Example:__ Delete all documents from the table `comments` without waiting for the
operation to be flushed to disk.

```js
r.table("comments").delete({durability: "soft"}).run(conn, callback)
```
