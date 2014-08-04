---
layout: example-app 
title: "Node.js TODO with AngularJS, Koa and RethinkDB"
github_url: "https://github.com/rethinkdb/rethinkdb-example-nodejs/tree/master/todo-angular-koa"
active: docs
docs_active: examples
permalink: docs/examples/node-todo-koa/
---


# About

You can find all the code for this example on GitHub in the
[rethinkdb-example-nodejs repository](https://github.com/rethinkdb/rethinkdb-example-nodejs/tree/master/todo-angular-koa).

This application was originally an example to illustrate AngularJS taken from
[todomvc.com](http://todomvc.com/). The code was slightly modified to persist data with
a server running on Node.js with [Koa](http://koajs.com).

This article discusses only the server-side code and more precisely the file
[app.js](https://github.com/rethinkdb/rethinkdb-example-nodejs/blob/master/todo-angular-koa/app.js).

This example uses [Koa](http://koajs.com) and requires an unstable version of Node (>= 0.11.9) since we will be using [generators](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Iterators_and_Generators#Generators.3A_a_better_way_to_build_Iterators).
Variants of this example are available:

- [With Express and callbacks](/docs/examples/node-todo/)
- [With Express and promises](/docs/examples/node-todo-promises/)


# Note

Koa will be started by default on the port `3000`. The Node server will try to connect to
RethinkDB on `localhost:28015`.   
You can change these parameters in `config.js`.

# Code and comments

The server has two functions:

- Serve static files (HTML, CSS, JavaScript files)
- Provide a REST API to retrieve/save/update/delete todos


## Import modules

We first have to import some modules: `koa`, `rethinkdb`, `koa-static` etc.

We also import the file `config.js` that contains some parameters for RethinkDB and Koa.

```js
var koa = require('koa');

// Middleware and helpers
var serve = require('koa-static');
var parse = require('co-body');
var router = require('koa-router');
var http = require('http');

// Import rethinkdb
var r = require('rethinkdb');

// Load config for RethinkDB and koa
var config = require(__dirname+"/config.js");
```

Create an HTTP server with Koa.

```js
var app = koa();
```


## Sequence of middleware

Koa will pass each request through the middleware. A middleware usually has one function
like parsing the cookie, parsing the header, opening a connection to the database, reading a static file
etc.

In our example, for each request we look if there is a file in the directory `public`
that matches the route. If we find such file, we serve it. If we do not, we will pass
the request to the next middleware.

The next middleware will form the REST API. Each request will go through:

- `createConnection`: Create a connection to RethinkDB and save it in `req._rdbConn`
- `get`/`create`/`update`/`del`: Perform an operation on the database depending on the route
- `closeConnection`: Close the connection to the database

This is how the sequence of middleware is defined.

```js
// Static content
app.use(serve(__dirname+'/public'));

// Create a RethinkDB connection
app.use(createConnection);

app.use(router(app));
app.get('/todo/get', get);
app.put('/todo/new', create);
app.post('/todo/update', update);
app.post('/todo/delete', del);

// Close the RethinkDB connection
app.use(closeConnection);
```


## Implement the middleware

Each middleware is a generator (note the star after `function`), the context is shared among all middleware.

_Create a connection_

Create a RethinkDB connection, and save it in `req._rdbConn`.

```js
function* createConnection(next) {
    try{
        // Open a connection and wait for r.connect(...) to be resolve
        var conn = yield r.connect(config.rethinkdb);
        // Save the connection in the current context (will be passed to the next middleware)
        this._rdbConn = conn;
    }
    catch(err) {
        this.status = 500;
        this.body = e.message || http.STATUS_CODES[this.status];
    }
    yield next;
}
```


_Retrieve all the todos_

The next middleware retrieves all the todos from the database ordered by date.

The ReQL query consists of 3 parts:

- `r.table('todos')`: selects all the documents from the table `todos`.
- `orderBy({index: "createdAt"})`: order the todos with the index `createdAt`.
- `run(...)`: Execute the query

Because there can be many todos, the server returns a cursor. In our case, we just
retrieve all the elements of the cursor and save it in an array.

```js
function* get(next) {
    try{
        var cursor = yield r.table('todos').orderBy({index: "createdAt"}).run(this._rdbConn);
        var result = yield cursor.toArray();
        this.body = JSON.stringify(result);
    }
    catch(e) {
        this.status = 500;
        this.body = e.message || http.STATUS_CODES[this.status];
    }
    yield next;
}
```

_Insert a todo_

Insert a new document in the table `todos` with the `insert` command.

```js
function* create(next) {
    try{
        // Parse the POST data
        var todo = yield parse(this);
        todo.createdAt = r.now(); // Set the field `createdAt` to the current time

        // Insert a new Todo
        var result = yield r.table('todos').insert(todo, {returnChanges: true}).run(this._rdbConn);

        todo = result.new_val; // todo now contains the previous todo + a field `id` and `createdAt`
        this.body = JSON.stringify(todo);
    }
    catch(e) {
        this.status = 500;
        this.body = e.message || http.STATUS_CODES[this.status];
    }
    yield next;
}
```


_Update a todo_

Update an existing todo.

We first select the todo with the `get` command, then call `update` on it.

```js
function* update(next) {
    try{
        var todo = yield parse(this);
        delete todo._saving;
        if ((todo == null) || (todo.id == null)) {
            throw new Error("The todo must have a field `id`.");
        }

        var result = yield r.table('todos').get(todo.id).update(todo, {returnChanges: true}).run(this._rdbConn);
        this.body = JSON.stringify(result.changes[0].new_val);
    }
    catch(e) {
        this.status = 500;
        this.body = e.message || http.STATUS_CODES[this.status];
    }
    yield next;
}
```



_Delete a todo_

Delete a todo given its `id`.

In a similar way as for the `update`, we first select the todo to delete, and then call `delete` on it.

```js
function* del(next) {
    try{
        var todo = yield parse(this);
        if ((todo == null) || (todo.id == null)) {
            throw new Error("The todo must have a field `id`.");
        }
        var result = yield r.table('todos').get(todo.id).delete().run(this._rdbConn);
        this.body = "";
    }
    catch(e) {
        this.status = 500;
        this.body = e.message || http.STATUS_CODES[this.status];
    }
    yield next;
}
```


_Close the connection to the database_

Close the connection to RethinkDB.

```js
function* closeConnection(next) {
    this._rdbConn.close();
}
```





## Initialize the database and start Koa.

Create the table and index then start Koa.

```js
r.connect(config.rethinkdb, function(err, conn) {
    if (err) {
        console.log("Could not open a connection to initialize the database");
        console.log(err.message);
        process.exit(1);
    }

    r.table('todos').indexWait('createdAt').run(conn).then(function(err, result) {
        console.log("Table and index are available, starting koa...");
        startKoa();
    }).error(function(err) {
        // The database/table/index was not available, create them
        r.dbCreate(config.rethinkdb.db).run(conn).finally(function() {
            return r.tableCreate('todos').run(conn)
        }).finally(function() {
            r.table('todos').indexCreate('createdAt').run(conn);
        }).finally(function(result) {
            r.table('todos').indexWait('createdAt').run(conn)
        }).then(function(result) {
            console.log("Table and index are available, starting koa...");
            startKoa();
            conn.close();
        }).error(function(err) {
            if (err) {
                console.log("Could not wait for the completion of the index `todos`");
                console.log(err);
                process.exit(1);
            }
            console.log("Table and index are available, starting koa...");
            startKoa();
            conn.close();
        });
    });
});
```
