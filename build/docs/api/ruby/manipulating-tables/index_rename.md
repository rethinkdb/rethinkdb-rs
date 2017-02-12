---
layout: api-command
language: Ruby
permalink: api/ruby/index_rename/
command: index_rename
related_commands:
    index_create: index_create/
    index_status: index_status/
    index_list: index_list/
    index_drop: index_drop/

---

# Command syntax #

{% apibody %}
table.index_rename(old_index_name, new_index_name[, {:overwrite => false}]) &rarr; object
{% endapibody %}

# Description #

Rename an existing secondary index on a table. If the optional argument `overwrite` is specified as `true`, a previously existing index with the new name will be deleted and the index will be renamed. If `overwrite` is `false` (the default) an error will be raised if the new index name already exists.

The return value on success will be an object of the format `{:renamed => 1}`, or `{:renamed => 0}` if the old and new names are the same.

An error will be raised if the old index name does not exist, if the new index name is already in use and `overwrite` is `False`, or if either the old or new index name are the same as the primary key field name.

__Example:__ Rename an index on the comments table.

```rb
r.table('comments').index_rename('post_id', 'message_id').run(conn)
```
