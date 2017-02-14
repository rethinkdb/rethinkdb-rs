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
table.replace(object | function[, {durability: "hard", returnChanges: false, nonAtomic: false}])
    &rarr; object
selection.replace(object | function[, {durability: "hard", returnChanges: false, nonAtomic: false}])
    &rarr; object
singleSelection.replace(object | function[, {durability: "hard", returnChanges: false, nonAtomic: false}])
    &rarr; object
{% endapibody %}

<img src="/assets/images/docs/api_illustrations/replace.png" class="api_command_illustration" />

# Description #

Replace documents in a table. Accepts a JSON document or a ReQL expression,
and replaces the original document with the new one. The new document must
have the same primary key as the original document.

The `replace` command can be used to both insert and delete documents. If
the "replaced" document has a primary key that doesn't exist in the table,
the document will be inserted; if an existing document is replaced with
`null`, the document will be deleted. Since `update` and `replace` operations
are performed atomically, this allows atomic inserts and deletes as well.

The optional arguments are:

- `durability`: possible values are `hard` and `soft`. This option will override
  the table or query's durability setting (set in [run](/api/javascript/run/)).
  In soft durability mode RethinkDB will acknowledge the write immediately after
  receiving it, but before the write has been committed to disk.
- `returnChanges`:
    - `true`: return a `changes` array consisting of `old_val`/`new_val` objects
      describing the changes made, only including the documents actually
      updated.
    - `false`: do not return a `changes` array (the default).
    - `"always"`: behave as `true`, but include all documents the command tried
      to update whether or not the update was successful. (This was the behavior
      of `true` pre-2.0.)
- `nonAtomic`: if set to `true`, executes the replacement and distributes the
  result to replicas in a non-atomic fashion. This flag is required to perform
  non-deterministic updates, such as those that require reading data from
  another table.

Replace returns an object that contains the following attributes:

- `replaced`: the number of documents that were replaced.
- `unchanged`: the number of documents that would have been modified, except
  that the new value was the same as the old value.
- `inserted`: the number of new documents added. A document is considered inserted if its primary key did not exist in the table at the time of the `replace` operation.
- `deleted`: the number of deleted documents when doing a replace with `null`.
- `errors`: the number of errors encountered while performing the replace.
- `first_error`: If errors were encountered, contains the text of the first
  error.
- `skipped`: 0 for a replace operation.
- `changes`: if `returnChanges` is set to `true`, this will be an array of
  objects, one for each objected affected by the `replace` operation. Each
  object will have two keys: `{new_val: <new value>, old_val: <old value>}`.

{% infobox alert %}
RethinkDB write operations will only throw exceptions if errors occur before any writes. Other errors will be listed in `first_error`, and `errors` will be set to a non-zero count. To properly handle errors with this term, code must both handle exceptions and check the `errors` return value!
{% endinfobox %}

__Example:__ Replace the document with the primary key `1`.

```javascript
r.table("posts").get(1).replace({
    id: 1,
    title: "Lorem ipsum",
    content: "Aleas jacta est",
    status: "draft"
}).run(conn, callback)
```

__Example:__ Remove the field `status` from all posts.

```javascript
r.table("posts").replace(function(post) {
    return post.without("status")
}).run(conn, callback)
```

__Example:__ Remove all the fields that are not `id`, `title` or `content`.

```javascript
r.table("posts").replace(function(post) {
    return post.pluck("id", "title", "content")
}).run(conn, callback)
```

__Example:__ Replace the document with the primary key `1` using soft durability.

```javascript
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

```javascript
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

```javascript
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
