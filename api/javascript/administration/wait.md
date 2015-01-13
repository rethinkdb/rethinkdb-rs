---
layout: api-command
language: JavaScript
permalink: api/javascript/table_wait/
command: tableWait
io:
    -   - table
        - object
    -   - database
        - object
    -   - r
        - object
---
# Command syntax #

{% apibody %}
table.wait() &rarr; object
database.wait() &rarr; object
r.wait() &rarr; object
{% endapibody %}

# Description #

Wait for a table or all the tables in a database to be ready. A table may be temporarily unavailable after creation, rebalancing or reconfiguring. The `wait` command blocks until the given table (or database) is fully up to date.

The return value is an object consisting of two key/value pairs:

* `ready`: an integer indicating the number of tables waited for. It will always be `1` when `wait` is called on a table, and the total number of tables when called on a database.
* `status_changes`: a list with one entry for each of the tables. Each member of the list will be an object with two fields:
    * `old_val`: The table's [status](/api/javascript/status) value before `wait` was executed.
    * `new_val`: The table's `status` value after `wait` finished.

See [status](/api/javascript/status) and [System tables](/docs/system-tables/) for a discussion of the fields within the `table_status` rows.

If `wait` is called with no table or database specified (the `r.wait()` form), it will wait on all the tables in the default database (set with the [connect](/api/javascript/connect/) command's `db` parameter, which defaults to `test`).

__Example:__ Get a table's status.

```js
> r.table('superheroes').wait().run(conn, callback);
// Result passed to callback
{
  "ready": 1,
  "status_changes": [
	{
	  "old_val": {
		"db": "database",
		"id": "5cb35225-81b2-4cec-9eef-bfad15481265",
		"name": "superheroes",
		"shards": [
		  {
			"primary_replica": null,
			"replicas": [
			  {
				"server": "jeeves",
				"state": "ready"
			  }
			]
		  },
		  {
			"primary_replica": null,
			"replicas": [
			  {
				"server": "jeeves",
				"state": "ready"
			  }
			]
		  }
		],
		"status": {
		  "all_replicas_ready": true,
		  "ready_for_outdated_reads": true,
		  "ready_for_reads": true,
		  "ready_for_writes": true
		}
	  },
	  "new_val": {
		"db": "database",
		"id": "5cb35225-81b2-4cec-9eef-bfad15481265",
		"name": "superheroes",
		"shards": [
		  {
			"primary_replica": null,
			"replicas": [
			  {
				"server": "jeeves",
				"state": "ready"
			  }
			]
		  },
		  {
			"primary_replica": null,
			"replicas": [
			  {
				"server": "jeeves",
				"state": "ready"
			  }
			]
		  }
		],
		"status": {
		  "all_replicas_ready": true,
		  "ready_for_outdated_reads": true,
		  "ready_for_reads": true,
		  "ready_for_writes": true
		}
	  }
	}
  ]
}
```
