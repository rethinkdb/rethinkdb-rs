---
layout: api-command
language: JavaScript
permalink: api/javascript/replace/
command: replace
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
    delete: delete/
---

# Command syntax #

{% apibody %}
table.replace(json | expr[, {durability: 'soft', returnVals: true}])
    &rarr; object
selection.replace(json | expr[, {durability: 'soft', returnVals: true}])
    &rarr; object
singleSelection.replace(json | expr[, {durability: 'soft', returnVals: true}])
    &rarr; object
{% endapibody %}

# Description #

Replace documents in a table. Accepts a JSON document or a ReQL expression, and replaces
the original document with the new one. The new document must have the same primary key
as the original document.

The optional arguments are:

- `durability`: possible values are `hard` and `soft`. It will override the table or
query's default durability setting.
- `returnVals`: when set to `true` and in case of a single update, the
updated row will be returned.


Replace returns an object that contains the following attributes:

- `replaced`: the number of documents that were replaced
- `unchanged`: the number of documents that would have been modified, except that the
new value was the same as the old value
- `inserted`: the number of new documents added. You can have new documents inserted if
you do a point-replace on a key that isn't in the table or you do a replace on a
selection and one of the documents you are replacing has been deleted
- `deleted`: the number of deleted documents when doing a replace with null
- `errors`: the number of errors encountered while performing the replace; if errors
occurred performing the replace, first_error contains the text of the first error encountered
- `skipped`: 0 for a replace operation
- `old_val`: for a single replace, the old document.
- `new_val`: for a single replace, the new document.


__Example:__ Replace the document with the primary key `1`.

```js
r.table("posts").get(1).replace({
    id: 1,
    title: "Lorem ipsum",
    content: "Aleas jacta est",
    status: "draft"
}).run(conn, callback)
```

__Example:__ Remove the field `status` from all posts.

```js
r.table("posts").replace(function(post) {
    return post.without("status")
).run(conn, callback)
```

__Example:__ Remove all the fields that are not `id`, `title` or `content`.

```js
r.table("posts").replace(function(post) {
    return post.pluck("id", "title", "content")
).run(conn, callback)
```

__Example:__ Replace the document with the primary key `1` using soft durability.

```js
r.table("posts").get(1).replace({
    id: 1,
    title: "Lorem ipsum",
    content: "Aleas jacta est",
    status: "draft"
}, {
    durability: "soft"
}).run(conn, callback)
```

__Example:__ Replace the document with the primary key `1` and return the values of the document before
and after the replace operation.

```js
r.table("posts").get(1).replace({
    id: 1,
    title: "Lorem ipsum",
    content: "Aleas jacta est",
    status: "published"
}, {
    returnVals: true
}).run(conn, callback)
```

The result will have two fields `old_val` and `new_val`.

```js
{
    "deleted": 0,
    "errors": 0,
    "inserted":0,
    "new_val":{
        "id":1,
        "title":"Lorem ipsum"
        "content":"Aleas jacta est",
        "status":"published",
    },
    "old_val": {
        "id":1,
        "title":"Lorem ipsum"
        "content":"TODO",
        "status":"draft",
        "author":"William",
    },
    "replaced":1,
    "skipped":0,
    "unchanged":0
}
```
