---
layout: api-command
language: Ruby
permalink: api/ruby/config/
command: config
---
# Command syntax #

{% apibody %}
table.config() &rarr; selection&lt;object&gt;
database.config() &rarr; selection&lt;object&gt;
{% endapibody %}

# Description #

Query (read and/or update) the configurations for individual tables or databases.

The `config` command is a shorthand way to access the `table_config` or `db_config` [System tables](/docs/system-tables/#configuration-tables). It will return the single row from the system that corresponds to the database or table configuration, as if [get](/api/ruby/get) had been called on the system table with the UUID of the database or table in question.

__Example:__ Get the configuration for the `users` table.

```rb
r.table('users').config().run(conn)

{
    :id => "31c92680-f70c-4a4b-a49e-b238eb12c023",
    :name => "users",
    :db => "superstuff",
    :primary_key => "id",
    :shards => [
        {
            :primary_replica => "a",
            :replicas => ["a", "b"],
            :nonvoting_replicas: => []
        },
        {
            :primary_replica => "d",
            :replicas => ["c", "d"],
            :nonvoting_replicas: => []
        }
    ],
    :indexes => [],
    :write_acks => "majority",
    :durability => "hard"
}
```

__Example:__ Change the write acknowledgement requirement of the `users` table.

```rb
r.table('users').config().update({:write_acks => 'single'}).run(conn)
```
