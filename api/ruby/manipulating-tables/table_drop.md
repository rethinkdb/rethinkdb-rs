---
layout: api-command
language: Ruby
permalink: api/ruby/table_drop/
command: table_drop
related_commands:
    table_create: table_create
    table_list: table_list/
---

# Command syntax #

{% apibody %}
db.table_drop(table_name) &rarr; object
{% endapibody %}

# Description #

Drop a table. The table and all its data will be deleted.

If successful, the command returns an object with two fields:

* `tables_dropped`: always `1`.
* `config_changes`: a list containing one two-field object, `old_val` and `new_val`:
    * `old_val`: the dropped table's [config](/api/ruby/config) value.
    * `new_val`: always `nil`.

If the given table does not exist in the database, the command throws `ReqlRuntimeError`.

__Example:__ Drop a table named 'dc_universe'.

```rb
r.db('test').table_drop('dc_universe').run(conn)

{
    :config_changes => [
        {
            :old_val => {
                :db => "test",
                :durability =>  "hard",
                :id => "20ea60d4-3b76-4817-8828-98a236df0297",
                :name => "dc_universe",
                :primary_key => "id",
                :shards => [
                    {
                        :primary_replica => "rethinkdb_srv1",
                        :replicas => [
                            "rethinkdb_srv1",
                            "rethinkdb_srv2"
                        ]
                    }
                ],
                :write_acks => "majority"
            },
            :new_val => nil
        }
    ],
    :tables_dropped => 1
}
```
