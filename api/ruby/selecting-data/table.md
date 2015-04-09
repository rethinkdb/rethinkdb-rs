---
layout: api-command
language: Ruby
permalink: api/ruby/table/
command: table
related_commands:
    filter: filter/
    get: get/
---

# Command syntax #

{% apibody %}
db.table(name[, {:use_outdated => false, :identifier_format => 'name'}) &rarr; table
{% endapibody %}

# Description #

Return all documents in a table. Other commands may be chained after `table` to return a subset of documents (such as `get` and `filter`) or perform further processing.

__Example:__ Return all documents in the table 'marvel' of the default database.

```rb
r.table('marvel').run(conn)
```

__Example:__ Return all documents in the table 'marvel' of the database 'heroes'.

```rb
r.db('heroes').table('marvel').run(conn)
```

There are two optional arguments.

* `use_outdated`: if `True`, this allows potentially out-of-date data to be returned, with potentially faster reads. It also allows you to perform reads from a secondary replica if a primary has failed. Default `False`.
* `identifier_format`: possible values are `name` and `uuid`, with a default of `name`. If set to `uuid`, then [system tables](/docs/system-tables/) will refer to servers, databases and tables by UUID rather than name. (This only has an effect when used with system tables.)

__Example:__ Allow potentially out-of-date data in exchange for faster reads.

```rb
r.db('heroes').table('marvel', {:use_outdated => true}).run(conn)
```
