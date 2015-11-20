---
layout: api-command
language: Ruby
permalink: api/ruby/db_create/
command: db_create
related_commands:
    db_drop: db_drop/
    db_list: db_list/
    table_create: table_create/
---

# Command syntax #

{% apibody %}
r.db_create(db_name) &rarr; object
{% endapibody %}

# Description #

Create a database. A RethinkDB database is a collection of tables, similar to
relational databases.

If successful, the command returns an object with two fields:

* `dbs_created`: always `1`.
* `config_changes`: a list containing one object with two fields, `old_val` and `new_val`:
    * `old_val`: always `nil`.
    * `new_val`: the database's new [config](/api/ruby/config) value.

If a database with the same name already exists, the command throws `ReqlRuntimeError`.

Note: Only alphanumeric characters and underscores are valid for the database name.

__Example:__ Create a database named 'superheroes'.

```rb
r.db_create('superheroes').run(conn)

{
    :config_changes => [
        {
            :new_val => {
                :id => "e4689cfc-e903-4532-a0e6-2d6797a43f07",
                :name => "superheroes"
            },
            :old_val => nil
        }
    ],
    :dbs_created => 1
}
```


