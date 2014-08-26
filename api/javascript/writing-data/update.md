---
layout: api-command
language: JavaScript
permalink: api/javascript/update/
command: update
io:
    -   - table
        - object
    -   - selection
        - object
    -   - singleSelection
        - object
related_commands:
    insert: insert/
    replace: replace/
    delete: delete/
---


# Command syntax #

{% apibody %}
table.update(json | expr[, {durability: "hard", returnChanges: false, nonAtomic: false}])
    &rarr; object
selection.update(json | expr[, {durability: "hard", returnChanges: false, nonAtomic: false}])
    &rarr; object
singleSelection.update(json | expr[, {durability: "hard", returnChanges: false, nonAtomic: false}])
    &rarr; object
{% endapibody %}

# Description #

Update JSON documents in a table. Accepts a JSON document, a ReQL expression, or a
combination of the two.

The optional arguments are:

- `durability`: possible values are `hard` and `soft`. This option will override the
table or query's durability setting (set in [run](/api/javascript/run/)).  
In soft durability mode RethinkDB will acknowledge the write immediately after
receiving it, but before the write has been committed to disk.
- `returnChanges`: if set to `true`, return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made.
- `nonAtomic`: set to `true` if you want to perform non-atomic updates (updates that
require fetching data from another document).


Update returns an object that contains the following attributes:

- `replaced`: the number of documents that were updated.
- `unchanged`: the number of documents that would have been modified except the new
value was the same as the old value.
- `skipped`: the number of documents that were skipped because the document didn't exist.
- `errors`: the number of errors encountered while performing the update.
- `first_error`: If errors were encountered, contains the text of the first error.
- `deleted` and `inserted`: 0 for an update operation.
- `changes`: if `returnChanges` is set to `true`, this will be an array of objects, one for each objected affected by the `update` operation. Each object will have two keys: `{new_val: <new value>, old_val: <old value>}`.


__Example:__ Update the status of the post with `id` of `1` to `published`.

```js
r.table("posts").get(1).update({status: "published"}).run(conn, callback)
```

__Example:__ Update the status of all posts to `published`.

```js
r.table("posts").update({status: "published"}).run(conn, callback)
```

__Example:__ Update the status of all the post written by William.

```js
r.table("posts").filter({author: "William"}).update({status: "published"}).run(conn, callback)
```


__Example:__ Increment the field `view` with `id` of `1`.
This query will throw an error if the field `views` doesn't exist.

```js
r.table("posts").get(1).update({
    views: r.row("views").add(1)
}).run(conn, callback)
```

__Example:__ Increment the field `view` of the post with `id` of `1`.
If the field `views` does not exist, it will be set to `0`.

```js
r.table("posts").update({
    views: r.row("views").add(1).default(0)
}).run(conn, callback)
```

__Example:__ Perform a conditional update.  
If the post has more than 100 views, set the `type` of a post to `hot`, else set it to `normal`.

```js
r.table("posts").get(1).update(function(post) {
    return r.branch(
        post("views").gt(100),
        {type: "hot"},
        {type: "normal"}
    )
}).run(conn, callback)
```

__Example:__ Update the field `numComments` with the result of a sub-query. Because
this update is not atomic, you must pass the `nonAtomic` flag.

```js
r.table("posts").get(1).update({
    numComments: r.table("comments").filter({idPost: 1}).count()
},{
    nonAtomic: true
}).run(conn, callback)
```

If you forget to specify the `nonAtomic` flag, you will get a `RqlRuntimeError`.

```
RqlRuntimeError: Could not prove function deterministic.  Maybe you want to use the non_atomic flag? 
```

__Example:__ Update the field `numComments` with a random value between 0 and 100.  
This update cannot be proven deterministic because of `r.js` (and in fact is not), so you
must pass the `nonAtomic` flag.

```js
r.table("posts").get(1).update({
    num_comments: r.js("Math.floor(Math.random()*100)")
},{
    nonAtomic: true
}).run(conn, callback)
```

__Example:__ Update the status of the post with `id` of `1` using soft durability.

```js
r.table("posts").get(1).update({status: "published"}, {durability: "soft"}).run(conn, callback)
```

__Example:__ Increment the field `views` and return the values of the document before
and after the update operation.

```js
r.table("posts").get(1).update({
    views: r.row("views").add(1)
}, {
    returnChanges: true
}).run(conn, callback)
```

The result will have two fields `old_val` and `new_val`.

```js
{
    deleted: 1,
    errors: 0,
    inserted: 0,
    changes: [
        {
            new_val: {
                id: 1,
                author: "Julius_Caesar",
                title: "Commentarii de Bello Gallico",
                content: "Aleas jacta est",
                views: 207
            },
            old_val: {
                id: 1,
                author: "Julius_Caesar",
                title: "Commentarii de Bello Gallico",
                content: "Aleas jacta est",
                views: 206
            }
        }
    ],
    replaced: 0,
    skipped: 0,
    unchanged: 0
}
```

