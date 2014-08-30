---
layout: documentation
title: Using secondary indexes in RethinkDB
active: docs
docs_active: secondary-indexes
permalink: docs/secondary-indexes/javascript/
switcher: true
language : JavaScript 
js: [fancybox]
---

<img src="/assets/images/docs/api_illustrations/secondary-indexes.png"
     alt="Secondary Indexes Illustration"
     class="api_command_illustration" />

Secondary indexes are data structures that improve the speed of many
read queries at the slight cost of increased storage space and decreased
write performance.

RethinkDB supports different types of secondary indexes:

- __Simple indexes__ based on the value of a single field.
- __Compound indexes__ based on multiple fields.
- __Multi indexes__ based on arrays of values.
- Indexes based on __arbitrary expressions__.

# Using indexes #

## Simple indexes ##

Use simple indexes to efficiently retrieve and order documents by the value of a single field.

### Creation ###

```js
// Create a secondary index on the last_name attribute
r.table("users").indexCreate("last_name").run(conn, callback)
```

```js
// Wait for the index to be ready to use
r.table("users").indexWait("last_name").run(conn, callback)
```

### Querying ###

```js
// Get all users whose last name is "Smith"
r.table("users").getAll("Smith", {index: "last_name"}).run(conn, callback)

// Get all users whose last names are "Smith" or "Lewis"
r.table("users").getAll("Smith", "Lewis", {index: "last_name"}).run(conn, callback)

// Get all users whose last names are between "Smith" and "Wade"
r.table("users").between("Smith", "Wade", {index: "last_name"}).run(conn, callback)

// Efficiently order users by last name using an index
r.table("users").orderBy({index: "last_name"}).run(conn, callback)

// For each blog post, return the post and its author using the last_name index
r.table("posts").eqJoin("author_last_name", r.table("users"), {index: "last_name"}) \
    .zip().run(conn, callback)
```

{% infobox info %}
__Want to learn more about joins in RethinkDB?__ See [how to use joins](/docs/table-joins/)
to query _one to many_ and _many to many_ relations.
{% endinfobox %}

## Compound indexes ##

Use compound indexes to efficiently retrieve documents by multiple fields.

### Creation ###

```js
// Create a compound secondary index based on the first_name and last_name attributes
r.table("users").indexCreate("full_name", [r.row("first_name"), r.row("last_name")]) \
    .run(conn, callback)
```

```js
// Wait for the index to be ready to use
r.table("users").indexWait("full_name").run(conn, callback)
```

### Querying ###

```js
// Get all users whose full name is John Smith.
r.table("users").getAll(["John", "Smith"], {index: "full_name"}).run(conn, callback)

// Get all users whose full name is between "John Smith" and "Wade Welles"
r.table("users").between(["John", "Smith"], ["Wade", "Welles"], {index: "full_name"}) \
    .run(conn, callback)

// Efficiently order users by first name and last name using an index
r.table("users").orderBy({index: "full_name"}).run(conn, callback)

// For each blog post, return the post and its author using the full_name index
r.table("posts").eqJoin("author_full_name", r.table("users"), {index: "full_name"}) \
    .run(conn, callback)
```

## Multi indexes ##

To index a single document multiple times by different values use a multi index. For
example, you can index a blog post with an array of tags by each tag.

### Creation ###

Suppose each post has a field `tags` that maps to an array of tags. The schema of the
table `posts` would be something like:

```js
{
    title: "...",
    content: "...",
    tags: [ <tag1>, <tag2>, ... ]
}

```

```js
// Create the multi index based on the field tags
r.table("posts").indexCreate("tags", {multi: true})

// Wait for the index to be ready to use
r.table("posts").indexWait("tags").run(conn, callback)
```

### Querying ###

```js
// Get all posts with the tag "travel" (where the field tags contains "travel")
r.table("posts").getAll("travel", {index: "tags"}).run(conn, callback)

// For each tag, return the tag and the posts that have such tag
r.table("tags").eqJoin("tag", r.table("posts"), {index: "tags"}).run(conn, callback)
```

## Indexes on arbitrary ReQL expressions ##

You can create an index on an arbitrary expression by passing an anonymous
function to `indexCreate`.

```js
// A different way to do a compound index
r.table("users").indexCreate("full_name2", function(user) {
    return r.add(user("last_name"), "_", user("first_name"))
}).run(conn, callback)
```

The function you give to `indexCreate` must be deterministic. In practice this means that
that you cannot use a function that contains a sub-query or the `r.js` command.

### Using multi indexes and arbitrary expressions together ###

You can create a multi index on an arbitrary expression in similar fashion,
by passing the multi option as the last parameter to `indexCreate`.

```js
// Create a multi index on a ReQL expression
r.table("users").indexCreate("activities", r.row("hobbies").add(r.row("sports")),
    {multi: true}).run(conn, callback)
```

### Use a multi index and a mapping function to speed getAll/contains ###

If your program frequently executes a [getAll](/api/javascript/get_all) followed by a [contains](/api/javascript/contains), that operation can be made more efficient by creating a compound multi index using a mapping function on the field that contains the list.

```js
// Create the index
r.table("users").indexCreate("userEquipment", function(user) {
    return user("equipment").map(function(equipment) {
        return [ user("id"), equipment ];
    });
}, {multi: true}).run(conn, callback);

// Query equivalent to:
// r.table("users").get(1).filter(function (user) {
//     return user("equipment").contains("tent");
// });
r.table("users").getAll([1, "tent"], {index: "userEquipment"}).run(conn, callback);
```

# Administrative operations #

## With ReQL ##

```js
// list indexes on table "users"
r.table("users").indexList().run(conn, callback)

// drop index "last_name" on table "users"
r.table("users").indexDrop("last_name").run(conn, callback)

// return the status of all indexes
r.table("users").indexStatus().run(conn, callback)

// return the status of the index "last_name"
r.table("users").indexStatus("last_name").run(conn, callback)

// return only when the index "last_name" is ready
r.table("users").indexWait("last_name").run(conn, callback)
```

## Manipulating indexes with the web UI ##

The web UI supports creation and deletion of simple secondary
indexes. In the table list, click on the table `users`. You can
manipulate indexes through the secondary index panel in the table
view.

<div class="screenshots">
    <a href="/assets/images/docs/query-language/secondary-index-ui.png"><img src="/assets/images/docs/query-language/secondary-index-ui.png" style="width: 269px; height: 105px;"></a>
</div>


# Limitations #

Secondary indexes have the following limitations:

- The part of a secondary index key that's used for fast lookups depends on the length of the primary key (which must be 127 bytes or less). The length of this part is 238&minus;*PK*, where *PK* is the primary key length; if the primary key length is a 36-character GUID, for instance, this means that 202 characters in the secondary index will be significant. If a table has multiple entries where the first 238&minus;*PK* characters are identical, lookup performance will be sharply degraded, as RethinkDB will have to perform a linear search to find the correct entries.

- Secondary indexes will not store `null` values or objects. Thus, the results of a command such as:

    ```js
    r.table("users").indexCreate("group").run(conn, callback)
    r.table("users").orderBy({index: "group"}).run(conn, callback)
    ```
    
    may be different from an equivalent command without an index:
    
    ```js
    r.table("users").orderBy("group").run(conn, callback)
    ```
    
    if the field being indexed has non-indexable values.
    
    This limitation will be removed in a future version of RethinkDB. See GitHub issue [#1032](https://github.com/rethinkdb/rethinkdb/issues/1032) to track progress on this.

- RethinkDB does not currently have an optimizer. As an example,
  the following query will not automatically use an index:

    ```js
    // This query does not use a secondary index! Use getAll instead.
    r.table("users").filter({"last_name": "Smith" }).run(conn, callback)
    ```

    You have to explicitly use the `getAll` command to take advantage
    of secondary indexes.

    ```js
    // This query uses a secondary index.
    r.table("users").getAll("Smith", {index: "last_name"}).run(conn, callback)
    ```

- You cannot chain multiple `getAll` commands. Use a compound index to
  efficiently retrieve documents by multiple fields.

- Currently, you cannot chain an `orderBy` using a secondary index after a `getAll` but only after a `table`. (You can, however, chain an `orderBy` using a secondary index after a `between` *if* they use the same index.)

- Currently, compound indexes cannot be queried by a prefix.  
  See [Github issue #955](https://github.com/rethinkdb/rethinkdb/issues/955)
  to track progress.

- RethinkDB does not provide a geospatial index yet.  
  See [Github issue #1158](https://github.com/rethinkdb/rethinkdb/issues/1158)
  to track progress.

- RethinkDB does not support unique secondary indexes even for non-sharded tables.

# More #

Browse the API reference to learn more about secondary index commands:

* Manipulating indexes: [indexCreate](/api/javascript/index_create/), [indexDrop](/api/javascript/index_drop/) and [indexList](/api/javascript/index_list/)
* Using indexes: [getAll](/api/javascript/get_all/), [between](/api/javascript/between/), [eqJoin](/api/javascript/eq_join/) and [orderBy](/api/javascript/order_by/)



<script type="text/javascript">
    $( function() {
        $('.screenshots a').fancybox();
    })
</script>
