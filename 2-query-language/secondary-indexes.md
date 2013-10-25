---
layout: documentation
title: Using secondary indexes in RethinkDB
active: docs
docs_active: secondary-indexes
permalink: docs/secondary-indexes/
js: [fancybox]
---

Secondary indexes are data structures that improve the speed of many
read queries at the cost of increased storage space and decreased
write performance.

RethinkDB supports different types of secondary indexes:

- __Simple indexes__ based on the value of a single field.
- __Compound indexes__ based on multiple fields.
- Indexes based on __arbitrary expressions__.

# Basic index usage #

## Creation and querying ##

```javascript
// Create a secondary index on the last_name attribute
r.table('users').indexCreate('last_name')

// Get all users with the last name Smith
r.table('users').getAll('Smith', { index: 'last_name' })

// Get all users whose last names are between Smith and Wade
r.table('users').between('Smith', 'Wade', { index: 'last_name' })
```

## Administrative operations ##

```javascript
// list indexes on table 'users'
r.table('users').indexList()

// drop index 'last_name' on table 'users'
r.table('users').indexDrop('last_name')
```

# More advanced operations #

## Compound indexes ##

```javascript
// Create a compound secondary index based on the last_name and first_name attributes
r.table('users').indexCreate('full_name', [r.row('last_name'), r.row('first_name')])
```

## Efficient table joins ##

```javascript
// For each blog post, return the post and its author using the full_name index
r.table('posts').eqJoin('author', r.table('users'), { index: 'full_name' })
```

{% infobox info %}
__Want to learn more about joins in RethinkDB?__ See [how to use joins](/docs/table-joins/)
to query _one to many_ and _many to many_ relations.
{% endinfobox %}

## Indexes on arbitrary ReQL expressions ##

```javascript
// A different way to do a compound index
r.table('users').indexCreate('full_name2',
                             r.add(r.row('last_name'), '_', r.row('first_name')))
```

# Manipulating indexes with the web UI #

The web UI supports creation and deletion of simple secondary
indexes. In the table list, click on the table `users`. You can
manipulate indexes through the secondary index panel in the table
view.

<div class="screenshots">
    <a href="/assets/images/docs/query-language/secondary-index-ui.png"><img src="/assets/images/docs/query-language/secondary-index-ui.png" style="width: 269px; height: 105px; "/></a>
</div>

# Limitations #

Secondary indexes have the following limitations:

- RethinkDB does not currently have an optimizer. As an example,
  following query will not automatically use an index:

  ```python
  # This query does not use a secondary index! Use get_all instead.
  r.table("posts").filter( {"author_last_name": "Smith" }).run()
  ```

  You have to explicitly use the `get_all` command to take advantage
  of secondary indexes.
- The `order_by` command does not currently use secondary
  indexes. This will be resolved soon (see [Github issue #159](https://github.com/rethinkdb/rethinkdb/issues/159)
  to track progress).
- Currently, compound indexes cannot be queried by a prefix. See
  [Github issue #955](https://github.com/rethinkdb/rethinkdb/issues/955)
  to track progress.


# More #

Browse the API reference to learn more about secondary index commands:

* [indexCreate](/api/python/index_create/)
* [indexDrop](/api/python/index_drop/)
* [indexList](/api/python/index_list/)
* [getAll](/api/python/get_all/)
* [between](/api/python/between/)



<script type="text/javascript">
    $( function() {
        $('.screenshots a').fancybox();
    })
</script>
