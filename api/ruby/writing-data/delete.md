---
layout: api-command
language: Ruby
permalink: api/ruby/delete/
command: delete
related_commands:
    insert: insert/
    update: update/
    replace: replace/
---


# Command syntax #

{% apibody %}
table.delete[({:durability => "hard", :return_vals => false})]
    &rarr; object
selection.delete[({:durability => "hard", :return_vals => false})]
    &rarr; object
singleSelection.delete[({:durability => "hard", :return_vals => false})]
    &rarr; object
{% endapibody %}

<img src="/assets/images/docs/api_illustrations/delete-vector.png" class="api_command_illustration" />

# Description #

Delete one or more documents from a table.

The optional arguments are:

- `durability`: possible values are `hard` and `soft`. This option will override the
table or query's durability setting (set in [run](/api/ruby/run/)).  
In soft durability mode RethinkDB will acknowledge the write immediately after
receiving it, but before the write has been committed to disk.
- `return_vals`: if set to `true` and in case of a single document deletion, the deleted
document will be returned.


Delete returns an object that contains the following attributes:

- `deleted`: the number of documents that were deleted.
- `skipped`: the number of documents that were skipped.  
For example, if you attempt to delete a batch of documents, and another concurrent query
deletes some of those documents first, they will be counted as skipped.
- `errors`: the number of errors encountered while performing the delete.
- `first_error`: If errors were encountered, contains the text of the first error.
- `inserted`, `replaced`, and `unchanged`: all 0 for a delete operation..
- `old_val`: if `return_vals` is set to `true`, contains the deleted document.
- `new_val`: if `return_vals` is set to `true`, contains `nil`.


__Example:__ Delete a single document from the table `comments`.

```rb
r.table("comments").get("7eab9e63-73f1-4f33-8ce4-95cbea626f59").delete.run(conn)
```


__Example:__ Delete all documents from the table `comments`.

```rb
r.table("comments").delete.run(conn)
```


__Example:__ Delete all comments where the field `id_post` is `3`.

```rb
r.table("comments").filter({:id_post => 3}).delete.run(conn)
```


__Example:__ Delete a single document from the table `comments` and return its value.

```rb
r.table("comments").get("7eab9e63-73f1-4f33-8ce4-95cbea626f59").delete(:return_vals => true).run(conn)
```

The result look like:

```rb
{
    :deleted => 1,
    :errors => 0,
    :inserted => 0,
    :new_val => nil,
    :old_val => {
        :id => "7eab9e63-73f1-4f33-8ce4-95cbea626f59",
        :author => "William",
        :comment => "Great post",
        :id_post => 3
    },
    :replaced => 0,
    :skipped => 0,
    :unchanged => 0
}
```


__Example:__ Delete all documents from the table `comments` without waiting for the
operation to be flushed to disk.

```rb
r.table("comments").delete(:durability => "soft").run(conn)
```
