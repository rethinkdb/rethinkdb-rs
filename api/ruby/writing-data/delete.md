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
table.delete[({:durability => soft, :return_vals => true})]
    &rarr; object
selection.delete[({:durability => soft, :return_vals => true})]
    &rarr; object
singleSelection.delete[({:durability => soft, :return_vals => true})]
    &rarr; object
{% endapibody %}

# Description #

Delete one or more documents from a table. The optional argument return_vals will return
the old value of the row you're deleting when set to true (only valid for single-row
deletes). The optional argument durability with value 'hard' or 'soft' will override the
table or query's default durability setting.

Delete returns an object that contains the following attributes:

- `deleted`: the number of documents that were deleted
- `skipped`: the number of documents from the selection that were left unmodified because
there was nothing to do. For example, if you delete a row that has already been deleted,
that row will be skipped
- `errors`L the number of errors encountered while deleting
if errors occured, first_error contains the text of the first error
- `inserted`, `replaced`, and `unchanged`: all 0 for a delete operation.


__Example:__ Delete superman from the database.

```rb
r.table('marvel').get('superman').delete.run(conn)
```

__Example:__ Delete every document from the table 'marvel'. Also, don't wait for the
operation to be flushed to disk.

```rb
r.table('marvel').delete(:durability => 'soft').run(conn)
```


__Example:__ You can get back a copy of the row you delete from the database as well.

```rb
r.table('marvel').delete(:return_vals => true).run(conn)
```

