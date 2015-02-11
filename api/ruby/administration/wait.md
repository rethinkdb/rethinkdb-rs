---
layout: api-command
language: Ruby
permalink: api/ruby/wait/
command: wait
---
# Command syntax #

{% apibody %}
table.wait([{:wait_for => 'ready_for_writes', :timeout => <sec>}]) &rarr; object
database.wait([{:wait_for => 'ready_for_writes', :timeout => <sec>}]) &rarr; object
r.wait([{:wait_for => 'ready_for_writes', :timeout => <sec>}]) &rarr; object
{% endapibody %}

# Description #

Wait for a table (or tables) to be ready. A table may be temporarily unavailable after creation, rebalancing or reconfiguring.

The `wait` command takes two optional arguments:

* `wait_for`: a string indicating a table [status](/api/ruby/status) to wait on before returning, one of `ready_for_outdated_reads`, `ready_for_reads`, `ready_for_writes`, or `all_replicas_ready`. The default is `ready_for_writes`. 
* `timeout`: a number indicating maximum time to wait for in seconds before returning. The default is no timeout.

The return value is an object consisting of two key/value pairs:

* `ready`: an integer indicating the number of tables waited for. It will always be `1` when `wait` is called on a table, and the total number of tables when called on a database.
* `status_changes`: a list with one entry for each of the tables. Each member of the list will be an object with two fields:
    * `old_val`: The table's [status](/api/ruby/status) value before `wait` was executed. 
    * `new_val`: The table's `status` value after `wait` finished.

See [status](/api/ruby/status) and [System tables](/docs/system-tables/) for a description of the fields within `status_changes`.

If `wait` is called with no table or database specified (the `r.wait()` form), it will wait on all the tables in the default database (set with the [connect](/api/ruby/connect/) command's `db` parameter, which defaults to `test`).

__Example:__ Wait on a table to be ready.

```rb
> r.table('superheroes').wait().run(conn)

{
  :ready => 1,
  :status_changes => [
	{
	  :old_val => {
		:db => "database",
		:id => "5cb35225-81b2-4cec-9eef-bfad15481265",
		:name => "superheroes",
		:shards => [
		  {
			:primary_replica => nil,
			:replicas => [
			  {
				:server => "jeeves",
				:state => "ready"
			  }
			]
		  },
		  {
			:primary_replica => nil,
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
			:primary_replica => nil,
			:replicas => [
			  {
				:server => "jeeves",
				:state => "ready"
			  }
			]
		  },
		  {
			:primary_replica => nil,
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
	  }
	}
  ]
}
```
