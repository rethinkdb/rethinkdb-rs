---
layout: api
title: "ReQL command reference"
active: api
no_footer: true
permalink: api/javascript/
language: JavaScript
---


{% apisection Accessing RQL%}
All RQL queries begin from the top level module.

## r ##

<div class="command-body-parent"><div class="command-body">
r → r
</div></div>

The toplevel RQL namespace.

__Example:__ Setup your top level namespace.

```js
var r = require('rethinkdb');
```


## connect ##

<div class="command-body-parent"><div class="command-body">
r.connect(opts, callback)
</div></div>

Create a new connection to the database server.

If the connection cannot be established, a `RqlDriverError` exception will be thrown

__Example:__ Opens a new connection to the database.

```js
r.connect({host:'localhost', port:28015, db:'marvel', authKey:'hunter2'},
   function(err, conn) { ... })
```


## close ##
<div class="command-body-parent"><div class="command-body">
conn.close()
</div></div>

Close an open connection. Closing a connection cancels all outstanding requests and frees
the memory associated with the open requests.

__Example:__ Close an open connection.

```js
conn.close()
```


## reconnect ##
<div class="command-body-parent"><div class="command-body">
conn.reconnect()
</div></div>

Close and attempt to reopen a connection. Has the effect of canceling any outstanding
request while keeping the connection open.

__Example:__ Cancel outstanding requests/queries that are no longer needed.

```js
conn.reconnect(function(errror, connection) { ... })
```


## use ##
<div class="command-body-parent"><div class="command-body">
conn.use(dbName)
</div></div>

Change the default database on this connection.

__Example:__ Change the default database so that we don't need to specify the database
when referencing a table.

```js
conn.use('heroes')
```


## run ##
<div class="command-body-parent"><div class="command-body">
query.run(connection, callback) → r
query.run(options[, callback]) → r
</div></div>

Run a query on a connection.

__Example: Call run on the connection with a query to execute the query. The callback
will get a cursor from which results may be retrieved.



## next ##
<div class="command-body-parent"><div class="command-body">
cursor.next(callback)
</div></div>

Get the next element in the cursor.

__Example:__ Let's grab the next element!

```js
cur.next(function(err, row) {
    return processRow(row);
});
```

## hasNext ##
<div class="command-body-parent"><div class="command-body">
cursor.hasNext() → bool
</div></div>

Check if there are more elements in the cursor.

__Example:__ Are there more elements in the cursor?

var hasMore = cur.hasNext();


## each ##
<div class="command-body-parent"><div class="command-body">
cursor.each(callback[, onFinishedCallback])
</div></div>

Lazily iterate over the result set one element at a time.

__Example:__ Let's process all the elements!

```js
cur.each(function(err, row) {
    processRow(row);
});
```


## toArray ##
<div class="command-body-parent"><div class="command-body">
cursor.toArray(callback)
</div></div>

Retrieve all results and pass them as an array to the given callback.

__Example:__ For small result sets it may be more convenient to process them at once as
an array.

```js
cur.toArray(function(err, results) {
    for(var i in results) {
        processRow(results[i]);
    }
});
```


## close (cursor) ##
<div class="command-body-parent"><div class="command-body">
cursor.close()
</div></div>


Close a cursor. Closing a cursor cancels the corresponding query and frees the memory
associated with the open request.

__Example:__ Close a cursor.

```js
cursor.close()
```


## addListener ##
<div class="command-body-parent"><div class="command-body">
connection.addListener(event, listener)
</div></div>

The connection object also supports the event emitter interface so you can listen for
changes in connection state.

Example: Monitor connection state with events 'connect', 'close', and 'error'.


```js
r.connect({}, function(err, conn) {
    if (err) throw err;

    conn.addListener('error', function(e) {
        processNetworkError(e);
    });

    conn.addListener('close', function() {
        cleanup();
    });

    runQueries(conn);
});

```


{% endapisection %}

{% apisection Manipulating databases%}
## dbCreate ##
<div class="command-body-parent"><div class="command-body">
r.dbCreate(dbName) → object
</div></div>

Create a database. A RethinkDB database is a collection of tables, similar to relational databases.

If successful, the operation returns an object: {created: 1}. If a database with the same name already exists the operation throws RqlRuntimeError.
Note: that you can only use alphanumeric characters and underscores for the database name.

__Example:__ Create a database named 'superheroes'.

```js
r.dbCreate('superheroes').run(conn, callback)
```


## dbDrop ##
<div class="command-body-parent"><div class="command-body">
r.dbDrop(dbName) → object
</div></div>

Drop a database. The database, all its tables, and corresponding data will be deleted.

If successful, the operation returns the object {dropped: 1}. If the specified database doesn't exist a RqlRuntimeError is thrown.

__Example:__ Drop a database named 'superheroes'.

```js
r.dbDrop('superheroes').run(conn, callback)
```

## dbList ##
<div class="command-body-parent"><div class="command-body">
r.dbList() → array
</div></div>

List all database names in the system. The result is a list of strings.

__Example:__ List all databases.

```js
r.dbList().run(conn, callback)
```

{% endapisection %}




{% apisection Manipulating tables%}
## tableCreate ##
<div class="command-body-parent"><div class="command-body">
db.tableCreate(tableName[, options]) → object
</div></div>

Create a table. A RethinkDB table is a collection of JSON documents. 

If successful, the operation returns an object: `{created: 1}`. If a table with the same
name already exists, the operation throws `RqlRuntimeError`.
Note: that you can only use alphanumeric characters and underscores for the table name.

__Example:__ Create a table named 'dc_universe' with the default settings.

```js
r.db('test').tableCreate('dc_universe').run(conn, callback)
```

## tableDrop ##
<div class="command-body-parent"><div class="command-body">
db.tableDrop(tableName) → object
</div></div>

Drop a table. The table and all its data will be deleted.

If succesful, the operation returns an object: {dropped: 1}. If the specified table doesn''t exist a RqlRuntimeError is thrown.

__Example:__ Drop a table named 'dc_universe'.

```js
r.db('test').tableDrop('dc_universe').run(conn, callback)
```


##  ##
<div class="command-body-parent"><div class="command-body">
</div></div>


##  ##
<div class="command-body-parent"><div class="command-body">
</div></div>


##  ##
<div class="command-body-parent"><div class="command-body">
</div></div>


##  ##
<div class="command-body-parent"><div class="command-body">
</div></div>


##  ##
<div class="command-body-parent"><div class="command-body">
</div></div>



{% endapisection %}


{% apisection Writing data%}

##  ##
<div class="command-body-parent"><div class="command-body">
</div></div>


##  ##
<div class="command-body-parent"><div class="command-body">
</div></div>


##  ##
<div class="command-body-parent"><div class="command-body">
</div></div>


##  ##
<div class="command-body-parent"><div class="command-body">
</div></div>


{% endapisection %}


{% apisection Selecting data%}

##  ##
<div class="command-body-parent"><div class="command-body">
</div></div>


##  ##
<div class="command-body-parent"><div class="command-body">
</div></div>


##  ##
<div class="command-body-parent"><div class="command-body">
</div></div>


##  ##
<div class="command-body-parent"><div class="command-body">
</div></div>


##  ##
<div class="command-body-parent"><div class="command-body">
</div></div>


##  ##
<div class="command-body-parent"><div class="command-body">
</div></div>


{% endapisection %}


{% apisection Joins data%}
{% endapisection %}

{% apisection Transformations%}
{% endapisection %}


{% apisection Aggregation%}
{% endapisection %}


{% apisection Aggregators%}
{% endapisection %}


{% apisection Document manipulation%}
{% endapisection %}


{% apisection String manipulation%}
{% endapisection %}


{% apisection Math and logic%}
{% endapisection %}


{% apisection Dates and times%}
{% endapisection %}


{% apisection Control structures%}
{% endapisection %}






