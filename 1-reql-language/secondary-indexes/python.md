---
layout: documentation
title: Using secondary indexes in RethinkDB
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

{% toctag %}

<img src="/assets/images/docs/api_illustrations/secondary-indexes.png"
     alt="Secondary Indexes Illustration"
     class="api_command_illustration" />

# Simple indexes #

Use simple indexes to efficiently retrieve and order documents by the value of a single field.

## Creation ##

```py
# Create a secondary index on the last_name attribute
r.table("users").index_create("last_name").run(conn)

# Wait for the index to be ready to use
r.table("users").index_wait("last_name").run(conn)
```

## Querying ##

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

{% infobox %}
__Want to learn more about joins in RethinkDB?__ See [how to use joins](/docs/table-joins/)
to query _one to many_ and _many to many_ relations.
{% endinfobox %}

# Compound indexes #

Compound indexes use arrays to efficiently retrieve documents by multiple fields.

## Creation ##

```py
# Create a compound secondary index based on the first_name and last_name attributes
r.table("users").index_create(
    "full_name", [r.row["last_name"], r.row["first_name"]]
).run(conn)

# Wait for the index to be ready to use
r.table("users").index_wait("full_name").run(conn)
```

### Querying ###

```py
# Get all users whose full name is John Smith.
r.table("users").get_all(["Smith", "John"], index="full_name").run(conn)

# Get all users whose full name is between "John Smith" and "Wade Welles"
r.table("users").between(
    ["Smith", "John"], ["Welles", "Wade"], index="full_name"
).run(conn)

# Get all users whose last name is Smith.
r.table("users").between(
    ["Smith", r.minval], ["Smith", r.maxval], index="full_name"
).run(conn)

# Efficiently order users by first name and last name using an index
r.table("users").order_by(index="full_name").run(conn)

# For each blog post, return the post and its author using the full_name index
r.table("posts").eq_join(
    "author_full_name", r.table("users"), index="full_name"
).run(conn)
```

Internally, compound indexes and simple indexes are the same type of index in RethinkDB; compound indexes are simply a special case of regular index that returns an array rather than a single value.

# Multi indexes #

With simple and compound indexes, a document will be indexed using at most one index key: a single value for a simple index and a set of values for a compound index. Multiple documents may have the same index key. With a _multi index_, a document can be indexed using more than one key in the same index. For instance, a blog post might have multiple tags, and each tag might refer to multiple blog posts.

The keys in a multi index can be single values, compound values or even arbitrary expressions. (See the section below for more detail on indexes using functions.)

## Creation ##

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

## Querying ##

```py
# Get all posts with the tag "travel" (where the field tags contains "travel")
r.table("posts").get_all("travel", index="tags").run(conn)

# For each tag, return the tag and the posts that have such tag
r.table("tags").eq_join("tag", r.table("posts"), index="tags").run(conn)
```

Note that queries with `getAll` or `between` may return the same document multiple times unless you use the [distinct](/api/python/distinct) command.

# Indexes on arbitrary ReQL expressions #

You can create an index on an arbitrary expression by passing an anonymous
function to `index_create`.

```py
# A different way to do a compound index
r.table("users").index_create("full_name2", lambda user:
    r.add(user["last_name"], "_", user["first_name"])).run(conn)
```

The function you give to `index_create` must be deterministic. In practice this means that that you cannot use a function that contains a sub-query or the `r.js` command.

{% infobox %}
If the function passed to `index_create` returns an error for a given document, that document will not be indexed. No error will be returned for those documents.
{% endinfobox %}

## Using multi indexes and arbitrary expressions together ##

You can create a multi index on an arbitrary expression in similar fashion,
by passing the multi option as the last parameter to `indexCreate`.

```py
# Create a multi index on a ReQL expression
r.table("users").index_create("activities", r.row["hobbies"] + r.row["sports"]),
    multi=True).run(conn)
```

## Use a multi index and a mapping function to speed get_all/contains ##

If your program frequently executes a [get_all](/api/python/get_all) followed by a [contains](/api/python/contains), that operation can be made more efficient by creating a compound multi index using a mapping function on the field that contains the list.

```py
# Create the index
r.table("users").index_create("user_equipment",
    lambda user: user['equipment'].map(
        lambda equipment: [user['id'], equipment]),
    multi=True).run(conn)

# Query equivalent to:
# r.table("users").get_all(1).filter(
#     lambda user: user['equipment'].contains('tent')
# ).run(conn)
r.table("users").get_all([1, "tent"], index="user_equipment").distinct().run(conn)
```

# Administrative operations #

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


# Notes #

The primary index of a table can be used in any ReQL command that uses a secondary index.

Indexes (both secondary and primary) are guaranteed to be updated by successful write operations. If an `insert`, `update` or `delete` operation is successful, the change will be correctly reflected in the index. (Read about RethinkDB [Consistency guarantees][cg] for write operations.)

[cg]: /docs/consistency/

The part of a secondary index key that's used for fast lookups depends on the length of the primary key (which must be 127 bytes or less). The length of this part is 238&minus;*PK*, where *PK* is the primary key length; if the primary key length is a 36-character GUID, for instance, this means that 202 characters in the secondary index will be significant. If a table has multiple entries where the first 238&minus;*PK* characters are identical, lookup performance will be sharply degraded, as RethinkDB will have to perform a linear search to find the correct entries.

Secondary indexes will not store `None` values or objects. Thus, the results of a command such as:

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

RethinkDB does not currently have an optimizer. As an example, the following query will not automatically use an index:

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

You cannot chain multiple `get_all` commands. Use a compound index to efficiently retrieve documents by multiple fields.

An `order_by` command that uses a secondary index cannot be chained after `get_all`. You can only chain it after a `table` command. However, you can chain `order_by` after a `between` command provided it uses the same index.

RethinkDB does not support unique secondary indexes even for non-sharded tables.

# More #

Browse the API reference to learn more about secondary index commands:

* Manipulating indexes: [index_create](/api/python/index_create/), [index_drop](/api/python/index_drop/) and [index_list](/api/python/index_list/)
* Using indexes: [get_all](/api/python/get_all/), [between](/api/python/between/), [eq_join](/api/python/eq_join/) and [order_by](/api/python/order_by/)
