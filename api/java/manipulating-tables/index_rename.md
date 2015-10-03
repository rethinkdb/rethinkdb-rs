---
layout: api-command
language: Java
permalink: api/javascript/index_rename/
command: indexRename
related_commands:
    indexCreate: index_create/
    indexStatus: index_status/
    indexList: index_list/
    indexDrop: index_drop/

---

# Command syntax #

{% apibody %}
table.indexRename(oldIndexName, newIndexName[, {overwrite: false}]) &rarr; object
{% endapibody %}

# Description #

Rename an existing secondary index on a table. If the optional argument `overwrite` is specified as `true`, a previously existing index with the new name will be deleted and the index will be renamed. If `overwrite` is `false` (the default) an error will be raised if the new index name already exists.

The return value on success will be an object of the format `{renamed: 1}`, or `{renamed: 0}` if the old and new names are the same.

An error will be raised if the old index name does not exist, if the new index name is already in use and `overwrite` is `false`, or if either the old or new index name are the same as the primary key field name.

__Example:__ Rename an index on the comments table.

```js
r.table('comments').indexRename('postId', 'messageId').run(conn)
```
