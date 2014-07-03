---
layout: documentation
title: Using secondary indexes in RethinkDB
active: docs
docs_active: secondary-indexes
permalink: docs/secondary-indexes/python/
alias: docs/secondary-indexes/
switcher: true
language : Python
js: [fancybox]
---

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

```py
# Create a secondary index on the last_name attribute
r.table("users").index_create("last_name").run(conn)

# Wait for the index to be ready to use
r.table("users").index_wait("last_name").run(conn)
```

### Querying ###

```py
# Get all users whose last name is "Smith"
r.table("users").get_all("Smith", index="last_name").run(conn)

# Get all users whose last names are "Smith" or "Lewis"
r.table("users").get_all("Smith", "Lewis", index="last_name").run(conn)

# Get all users whose last names are between "Smith" and "Wade"
r.table("users").between("Smith", "Wade", index="last_name").run(conn)

# Efficiently order users by last name using an index
r.table("users").order_by(index="last_name").run(conn)

# For each blog post, return the post and its author using the last_name index
r.table("posts").eq_join("author_last_name", r.table("users"), index="last_name") \
    .zip().run(conn)
```

{% infobox info %}
__Want to learn more about joins in RethinkDB?__ See [how to use joins](/docs/table-joins/)
to query _one to many_ and _many to many_ relations.
{% endinfobox %}

## Compound indexes ##

Use compound indexes to efficiently retrieve documents by multiple fields.

### Creation ###

```py
# Create a compound secondary index based on the first_name and last_name attributes
r.table("users").index_create("full_name", [r.row["first_name"], r.row["last_name"]]) \
    .run(conn)

# Wait for the index to be ready to use
r.table("users").index_wait("full_name").run(conn)
```

### Querying ###

```py
# Get all users whose full name is John Smith.
r.table("users").get_all(["John", "Smith"], index="full_name").run(conn)

# Get all users whose full name is between "John Smith" and "Wade Welles"
r.table("users").between(["John", "Smith"], ["Wade", "Welles"], index="full_name") \
    .run(conn)

# Efficiently order users by first name and last name using an index
r.table("users").order_by(index="full_name").run(conn)

# For each blog post, return the post and its author using the full_name index
r.table("posts").eq_join("author_full_name", r.table("users"), index="full_name") \
    .run(conn)
```

## Multi indexes ##

To index a single document multiple times by different values use a multi index. For
example, you can index a blog post with an array of tags by each tag.

### Creation ###
Suppose each post has a field `tags` that maps to an array of tags. The schema of the
table `posts` would be something like:

```py
{
    "title": "...",
    "content": "...",
    "tags": [ <tag1>, <tag2>, ... ]
}

```

```py
# Create the multi index based on the field tags
r.table("posts").index_create("tags", multi=True)

# Wait for the index to be ready to use
r.table("posts").index_wait("tags").run(conn)
```

### Querying ###

```py
# Get all posts with the tag "travel" (where the field tags contains "travel")
r.table("posts").get_all("travel", index="tags").run(conn)

# For each tag, return the tag and the posts that have such tag
r.table("tags").eq_join("tag", r.table("posts"), index="tags").run(conn)
```

## Indexes on arbitrary ReQL expressions ##

You can create an index on an arbitrary expression by passing an anonymous
function to `index_create`.

```py
# A different way to do a compound index
r.table("users").index_create("full_name2", lambda user:
    r.add(user["last_name"], "_", user["first_name"])).run(conn)
```

The function you give to `index_create` must be deterministic. In practice this means that
that you cannot use a function that contains a sub-query or the `r.js` command.

### Using multi indexes and arbitrary expressions together ###

You can create a multi index on an arbitrary expression in similar fashion,
by passing the multi option as the last parameter to `indexCreate`.

```py
# Create a multi index on a ReQL expression
r.table("users").index_create("activities", r.row["hobbies"] + r.row["sports"]),
    multi=True).run(conn)
```

# Administrative operations #

## With ReQL ##

```py
# list indexes on table "users"
r.table("users").index_list().run(conn)

# drop index "last_name" on table "users"
r.table("users").index_drop("last_name").run(conn)

# return the status of all indexes
r.table("users").index_status().run(conn)

# return the status of the index "last_name"
r.table("users").index_status("last_name").run(conn)

# return only when the index "last_name" is ready
r.table("users").index_wait("last_name").run(conn)
```


## Manipulating indexes with the web UI ##

The web UI supports creation and deletion of simple secondary
indexes. In the table list, click on the table `users`. You can
manipulate indexes through the secondary index panel in the table
view.

<div class="screenshots">
    <a href="/assets/images/docs/query-language/secondary-index-ui.png"><img src="/assets/images/docs/query-language/secondary-index-ui.png" style="width: 269px; height: 105px; "/></a>
</div>


# Limitations #

Secondary indexes have the following limitations:

- Secondary indexes will not store `None` values or objects. Thus, the results of a command such as:

    ```py
    r.table("users").index_create("group").run(conn)
    r.table("users").order_by(index="group").run(conn)
    ```
    
    may be different from an equivalent command without an index:
    
    ```py
    r.table("users").order_by("group").run(conn)
    ```
    
    if the field being indexed has non-indexable values.
    
    This limitation will be removed in a future version of RethinkDB. See GitHub issue [#1032](https://github.com/rethinkdb/rethinkdb/issues/1032) to track progress on this.

- RethinkDB does not currently have an optimizer. As an example,
  the following query will not automatically use an index:

  ```python
  # This query does not use a secondary index! Use get_all instead.
  r.table("users").filter({"last_name": "Smith" }).run(conn)
  ```

  You have to explicitly use the `get_all` command to take advantage
  of secondary indexes.

  ```python
  # This query uses a secondary index.
  r.table("users").get_all("Smith", index="last_name").run(conn)
  ```

- You cannot chain multiple `get_all` commands. Use a compound index to efficiently
  retrieve documents by multiple fields.

- Currently, compound indexes cannot be queried by a prefix.  
  See [Github issue #955](https://github.com/rethinkdb/rethinkdb/issues/955)
  to track progress.

- RethinkDB does not provide a geospatial index yet.  
  See [Github issue #1158](https://github.com/rethinkdb/rethinkdb/issues/1158)
  to track progress.

- RethinkDB does not support unique secondary indexes even for non-sharded tables.

# More #

Browse the API reference to learn more about secondary index commands:

* Manipulating indexes: [index_create](/api/python/index_create/), [index_drop](/api/python/index_drop/) and [index_list](/api/python/index_list/)
* Using indexes: [get_all](/api/python/get_all/), [between](/api/python/between/), [eq_join](/api/python/eq_join/) and [order_by](/api/python/order_by/)



<script type="text/javascript">
    $( function() {
        $('.screenshots a').fancybox();
    })
</script>
