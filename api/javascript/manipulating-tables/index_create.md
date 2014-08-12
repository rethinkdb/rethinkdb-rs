---
layout: api-command
language: JavaScript
permalink: api/javascript/index_create/
command: indexCreate
io:
    -   - table
        - object
related_commands:
    indexWait: index_wait/
    indexStatus: index_status/
    indexList: index_list/
    indexDrop: index_drop/

---

# Command syntax #

{% apibody %}
table.indexCreate(indexName[, indexFunction][, {multi: false}]) &rarr; object
{% endapibody %}

# Description #

Create a new secondary index on a table. Secondary indexes improve the speed of many read queries at the slight cost of increased storage space and decreased write performance. For more information about secondary indexes, read the article "[Using secondary indexes in RethinkDB](/docs/secondary-indexes/)."

RethinkDB supports different types of secondary indexes:

- *Simple indexes* based on the value of a single field.
- *Compound indexes* based on multiple fields.
- *Multi indexes* based on arrays of values.
- Indexes based on *arbitrary expressions*.

The `indexFunction` can be an anonymous function or a binary representation obtained from the `function` field of [indexStatus](/api/javascript/index_status).

__Example:__ Create a simple index based on the field `postId`.

```js
r.table('comments').indexCreate('postId').run(conn, callback)
```

__Example:__ Create a simple index based on the nested field `author > name`.

```js
r.table('comments').indexCreate('authorName', r.row("author")("name")).run(conn, callback)
```


__Example:__ Create a compound index based on the fields `postId` and `date`.

```js
r.table('comments').indexCreate('postAndDate', [r.row("postId"), r.row("date")]).run(conn, callback)
```

__Example:__ Create a multi index based on the field `authors`.

```js
r.table('posts').indexCreate('authors', {multi: true}).run(conn, callback)
```

__Example:__ Create an index based on an arbitrary expression.

```js
r.table('posts').indexCreate('authors', function(doc) {
    return r.branch(
        doc.hasFields("updatedAt"),
        doc("updatedAt"),
        doc("createdAt")
    )
}).run(conn, callback)
```

__Example:__ Create a new secondary index based on an existing one.

```js
r.table('posts').indexStatus('authors').nth(0)('function').run(conn, function (func) {
    r.table('newPosts').indexCreate('authors', func).run(conn, callback);
});
```

__Example:__ Rebuild an outdated secondary index on a table.

```js
r.table('posts').indexStatus('oldIndex').nth(0).do(function(oldIndex) {
  return r.table('posts').indexCreate('newIndex', oldIndex("function")).do(function() {
    return r.table('posts').indexWait('newIndex').do(function() {
      return r.table('posts').indexRename('newIndex', 'oldIndex', {overwrite: true})
    })
  })
})
```
