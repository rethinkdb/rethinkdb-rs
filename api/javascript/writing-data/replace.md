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
table.replace(json | expr[, {durability: "hard", returnChanges: false, nonAtomic: false}])
    &rarr; object
selection.replace(json | expr[, {durability: "hard", returnChanges: false, nonAtomic: false}])
    &rarr; object
singleSelection.replace(json | expr[, {durability: "hard", returnChanges: false, nonAtomic: false}])
    &rarr; object
{% endapibody %}

<img src="/assets/images/docs/api_illustrations/replace.png" class="api_command_illustration" />

# Description #

Replace documents in a table. Accepts a JSON document or a ReQL expression, and replaces
the original document with the new one. The new document must have the same primary key
as the original document.

The optional arguments are:

- `durability`: possible values are `hard` and `soft`. This option will override the
table or query's durability setting (set in [run](/api/javascript/run/)).  
In soft durability mode RethinkDB will acknowledge the write immediately after
receiving it, but before the write has been committed to disk.
- `returnChanges`: if set to `true`, return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made.
- `nonAtomic`: set to `true` if you want to perform non-atomic replaces (replaces that
require fetching data from another document).


Replace returns an object that contains the following attributes:

- `replaced`: the number of documents that were replaced
- `unchanged`: the number of documents that would have been modified, except that the
new value was the same as the old value
- `inserted`: the number of new documents added. You can have new documents inserted if
you do a point-replace on a key that isn't in the table or you do a replace on a
selection and one of the documents you are replacing has been deleted
- `deleted`: the number of deleted documents when doing a replace with `null`
- `errors`: the number of errors encountered while performing the replace.
- `first_error`: If errors were encountered, contains the text of the first error.
- `skipped`: 0 for a replace operation
- `changes`: if `returnChanges` is set to `true`, this will be an array of objects, one for each objected affected by the `delete` operation. Each object will have two keys: `{new_val: <new value>, old_val: <old value>}`.

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
}).run(conn, callback)
```

__Example:__ Remove all the fields that are not `id`, `title` or `content`.

```js
r.table("posts").replace(function(post) {
    return post.pluck("id", "title", "content")
}).run(conn, callback)
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
    returnChanges: true
}).run(conn, callback)
```

The result will have two fields `old_val` and `new_val`.

```js
{
    deleted: 0,
    errors: 0,
    inserted: 0,
    changes: [
        {
            new_val: {
                id:1,
                title: "Lorem ipsum"
                content: "Aleas jacta est",
                status: "published",
            },
            old_val: {
                id:1,
                title: "Lorem ipsum"
                content: "TODO",
                status: "draft",
                author: "William",
            }
        }
    ],
    replaced: 1,
    skipped: 0,
    unchanged: 0
}
```
