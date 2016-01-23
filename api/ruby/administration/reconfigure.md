---
layout: api-command
language: Ruby
permalink: api/ruby/reconfigure/
command: reconfigure
---
# Command syntax #

{% apibody %}
table.reconfigure({:shards => <s>, :replicas => <r>[, :primary_replica_tag => <t>, :dry_run => false, :nonvoting_replica_tags => nil]}) &rarr; object
database.reconfigure({:shards => <s>, :replicas => <r>[, :primary_replica_tag => <t>, :dry_run => false, :nonvoting_replica_tags => nil]}) &rarr; object
table.reconfigure(:emergency_repair => <option>, :dry_run => false) &rarr; object
{% endapibody %}

# Description #

Reconfigure a table's sharding and replication.

* `shards`: the number of shards, an integer from 1-32. Required.
* `replicas`: either an integer or a mapping object. Required.
    * If `replicas` is an integer, it specifies the number of replicas per shard. Specifying more replicas than there are servers will return an error.
    * If `replicas` is an object, it specifies key-value pairs of server tags and the number of replicas to assign to those servers: `{:tag1 => 2, :tag2 => 4, :tag3 => 2, ...}`. For more information about server tags, read [Administration tools](/docs/administration-tools/).
* `primary_replica_tag`: the primary server specified by its server tag. Required if `replicas` is an object; the tag must be in the object. This must *not* be specified if `replicas` is an integer.
* `dry_run`: if `true` the generated configuration will not be applied to the table, only returned.
* `nonvoting_replica_tags`: replicas with these server tags will be added to the `nonvoting_replicas` list of the resulting configuration. (See [failover](/docs/failover) for details about non-voting replicas.)
* `emergency_repair`: Used for the Emergency Repair mode. See the separate section below.

The return value of `reconfigure` is an object with three fields:

* `reconfigured`: the number of tables reconfigured. This will be `0` if `dry_run` is `true`.
* `config_changes`: a list of new and old table configuration values. Each element of the list will be an object with two fields:
    * `old_val`: The table's [config](/api/ruby/config) value before `reconfigure` was executed. 
    * `new_val`: The table's `config` value after `reconfigure` was executed.
* `status_changes`: a list of new and old table status values. Each element of the list will be an object with two fields:
    * `old_val`: The table's [status](/api/ruby/status) value before `reconfigure` was executed. 
    * `new_val`: The table's `status` value after `reconfigure` was executed.

For `config_changes` and `status_changes`, see the [config](/api/ruby/config) and [status](/api/ruby/status) commands for an explanation of the objects returned in the `old_val` and `new_val` fields.

A table will lose availability temporarily after `reconfigure` is called; use the [wait](/api/ruby/wait) command to wait for the table to become available again, or [status](/api/ruby/status) to check if the table is available for writing.

**Note:** Whenever you call `reconfigure`, the write durability will be set to `hard` and the write acknowledgments will be set to `majority`; these can be changed by using the `config` command on the table.

If `reconfigure` is called on a database, all the tables in the database will have their configurations affected. The return value will be an array of the objects described above, one per table.

Read [Sharding and replication](/docs/sharding-and-replication/) for a complete discussion of the subject, including advanced topics.

__Example:__ Reconfigure a table.

```rb
r.table('superheroes').reconfigure({:shards => 2, :replicas => 1}).run(conn)
```

<!-- stop -->

Example return:

```rb
{
  :reconfigured => 1,
  :config_changes => [
    {
      :new_val => {
        :id => "31c92680-f70c-4a4b-a49e-b238eb12c023",
        :name => "superheroes",
        :db => "superstuff",
        :primary_key => "id",
        :shards => [
          {
            :primary_replica => "jeeves",
            :replicas => ["jeeves"],
            :nonvoting_replicas => []
          },
          {
            :primary_replica => "alfred",
            :replicas => ["alfred"],
            :nonvoting_replicas => []
          }
        ],
        :indexes => [],
        :write_acks => "majority",
        :durability => "hard"
      },
      :old_val => {
        :id => "31c92680-f70c-4a4b-a49e-b238eb12c023",
        :name => "superheroes",
        :db => "superstuff",
        :primary_key => "id",
        :shards => [
          {
            :primary_replica => "alfred",
            :replicas => ["alfred"],
            :nonvoting_replicas => []
          }
        ],
        :indexes => [],
        :write_acks => "majority",
        :durability => "hard"
      }
    }
  ],
  :status_changes => [
    {
      :new_val => (status object),
      :old_val => (status object)
    }
  ]
}
```

__Example:__ Reconfigure a table, specifying replicas by server tags.

```rb
r.table('superheroes').reconfigure({:shards => 2, :replicas => {:wooster => 1, :wayne => 1}, :primary_replica_tag => 'wooster'}).run(conn)

{
  :reconfigured => 1,
  :config_changes => [
    {
      :new_val => {
        :id => "31c92680-f70c-4a4b-a49e-b238eb12c023",
        :name => "superheroes",
        :db => "superstuff",
        :primary_key => "id",
        :shards => [
          {
            :primary_replica => "jeeves",
            :replicas => ["jeeves"],
            :nonvoting_replicas => []
          },
          {
            :primary_replica => "alfred",
            :replicas => ["alfred"],
            :nonvoting_replicas => []
          }
        ],
        :indexes => [],
        :write_acks => "majority",
        :durability => "hard"
      },
      :old_val => {
        :id => "31c92680-f70c-4a4b-a49e-b238eb12c023",
        :name => "superheroes",
        :db => "superstuff",
        :primary_key => "id",
        :shards => [
          {
            :primary_replica => "alfred",
            :replicas => ["alfred"],
            :nonvoting_replicas => []
          }
        ],
        :indexes => [],
        :write_acks => "majority",
        :durability => "hard"
      }
    }
  ],
  :status_changes => [
    {
      :new_val => (status object),
      :old_val => (status object)
    }
  ]
}
```

# Emergency Repair mode #

RethinkDB supports automatic failover when more than half of the voting replicas for each shard of a table are still available (see the [Failover][fail] documentation for more details). However, if half or more of the voting replicas for a shard are lost, failover will not happen automatically, leaving two options:

[fail]: /docs/failover/

* Bring enough of the missing servers back online to allow automatic failover
* Use emergency repair mode to reconfigure the table

The `emergency_repair` argument is effectively a different command; when it is specified, no other arguments to `reconfigure` are allowed except for `dry_run`. When it's executed, each shard of the table is examined and classified into one of three categories:

* **Healthy:** more than half of the shard's voting replicas are still available.
* **Repairable:** the shard is not healthy, but has at least one replica, whether voting or non-voting, available.
* **Beyond repair:** the shard has no replicas available.

For each repairable shard, `emergency_repair` will convert all unavailable voting replicas into non-voting replicas. If all the voting replicas were removed, an arbitrarily-chosen available non-voting replica will be converted into a voting replica. After this operation, all of the shard's available replicas will be voting replicas.

Specify `emergency_repair` with one of two string options:

* `unsafe_rollback`: shards that are beyond repair will be left alone.
* `unsafe_rollback_or_erase`: a shard that is beyond repair will be destroyed and recreated on an available server that holds another shard for that table.

The return value of `reconfigure` in emergency repair mode is the same as before. Examine the `config_changes` field to see the old and new configuration settings for the table. As in the normal mode, if you specify `emergency_repair` with `dry_run: true`, the table will not actually be reconfigured.

__Note:__ `emergency_repair` may only be used on individual tables, not on databases. It cannot be used after the `db` command.

{% infobox alert %}
**The emergency repair mode is extremely dangerous.** It bypasses normal safeguards that prevent data loss and invalidates the [consistency guarantees](/docs/consistency/) that RethinkDB normally provides, and can easily lose data in either mode&mdash;in `unsafe_rollback_or_erase` mode it could lose *all* of a shard's data.
{% endinfobox %}

__Example:__ Perform an emergency repair on a table.

```rb
r.table('superheroes').reconfigure(
    {:emergency_repair => 'unsafe_rollback'}
).run(conn)
```
