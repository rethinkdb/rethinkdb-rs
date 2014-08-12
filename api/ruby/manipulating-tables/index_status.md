---
layout: api-command
language: Ruby
permalink: api/ruby/index_status/
command: index_status
related_commands:
    index_wait: index_wait/
---

# Command syntax #

{% apibody %}
table.index_status([, index...]) &rarr; array
{% endapibody %}

# Description #

Get the status of the specified indexes on this table, or the status
of all indexes on this table if no indexes are specified.

The result is an array where for each index, there will be an object like this one:

```rb
{
    :index => <index_name>,
    :ready => true,
    :function => <binary>,
    :multi => <bool>,
    :outdated => <bool>
}
```

or this one:

```rb
{
    :index => <index_name>,
    :ready => false,
    :blocks_processed => <int>,
    :blocks_total => <int>,
    :function => <binary>,
    :multi => <bool>,
    :outdated => <bool>
}
```

The `multi` field will be `true` or `false` depending on whether this index was created as a multi index (see [index_create](/api/ruby/index_create/) for details). The `outdated` field will be true if the index is outdated in the current version of RethinkDB and needs to be rebuilt.

The `function` field is a binary object containing an opaque representation of the secondary index (including the `multi` argument if specified). It can be passed as the second argument to [index_create](/api/ruby/index_create/) to create a new index with the same function; see `index_create` for more information.

__Example:__ Get the status of all the indexes on `test`:

```rb
r.table('test').index_status.run(conn)
```

__Example:__ Get the status of the `timestamp` index:

```rb
r.table('test').index_status('timestamp').run(conn)
```

__Example:__ Save the binary representation of the index:

```rb
func = r.table('test').index_status('timestamp').nth(0)['function'].run(conn)
```
