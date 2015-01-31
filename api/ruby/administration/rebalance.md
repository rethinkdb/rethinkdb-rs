---
layout: api-command
language: Ruby
permalink: api/ruby/rebalance/
command: rebalance
---
# Command syntax #

{% apibody %}
table.rebalance() &rarr; object
database.rebalance() &rarr; object
{% endapibody %}

# Description #

Rebalances the shards of a table. When called on a database, all the tables in that database will be rebalanced.

The `rebalance` command operates by measuring the distribution of primary keys within a table and picking split points that will give each shard approximately the same number of documents. It won't change the number of shards within a table, or change any other configuration aspect for the table or the database.

A table will lose availability temporarily after `rebalance` is called; use the [wait](/api/ruby/wait) command to wait for the table to become available again, or [status](/api/ruby/status) to check if the table is available for writing.

RethinkDB automatically rebalances tables when the number of shards are increased, and as long as your documents have evenly distributed primary keys--such as the default UUIDs--it is rarely necessary to call `rebalance` manually. Cases where `rebalance` may need to be called include:

* Tables with unevenly distributed primary keys, such as incrementing integers
* Changing a table's primary key type
* Increasing the number of shards on an empty table, then using non-UUID primary keys in that table

The [web UI](/docs/administration-tools/) (and the [info](/api/ruby/info) command) can be used to tell you when a table's shards need to be rebalanced.

The return value of `rebalance` is an object with two fields:

* `rebalanced`: the number of tables rebalanced.
* `status_changes`: a list of new and old table status values. Each element of the list will be an object with two fields:
    * `old_val`: The table's [status](/api/ruby/status) value before `rebalance` was executed.
    * `new_val`: The table's `status` value after `rebalance` was executed. (This value will almost always indicate the table is unavailable.)

See the [status](/api/ruby/status) command for an explanation of the objects returned in the `old_val` and `new_val` fields.

__Example:__ Rebalance a table.

```rb
r.table('superheroes').rebalance().run(conn)

{
  :rebalanced => 1,
  :status_changes => [
    {
      :old_val => {
        :db => "database",
        :id => "5cb35225-81b2-4cec-9eef-bfad15481265",
        :name => "superheroes",
        :shards => [
          {
            :primary_replica => "jeeves",
            :replicas => [
              {
                :server => "jeeves",
                :state => "ready"
              }
            ]
          },
          {
            :primary_replica => "jeeves",
            :replicas => [
              {
                :server => "jeeves",
                :state => "ready"
              }
            ]
          }
        ],
        :status => {
          :all_replicas_ready => true,
          :ready_for_outdated_reads => true,
          :ready_for_reads => true,
          :ready_for_writes => true
        }
      },
      :new_val => {
        :db => "database",
        :id => "5cb35225-81b2-4cec-9eef-bfad15481265",
        :name => "superheroes",
        :shards => [
          {
            :primary_replica => "jeeves",
            :replicas => [
              {
                :server => "jeeves",
                :state => "transitioning"
              }
            ]
          },
          {
            :primary_replica => "jeeves",
            :replicas => [
              {
                :server => "jeeves",
                :state => "transitioning"
              }
            ]
          }
        ],
        :status => {
          :all_replicas_ready => false,
          :ready_for_outdated_reads => false,
          :ready_for_reads => false,
          :ready_for_writes => false
        }
      }

    }
  ]
}
```
