---
layout: example-app 
title: "Node.js TODO with Angular.js, Express and RethinkDB"
github_url: "https://github.com/rethinkdb/rethinkdb-example-nodejs/tree/master/todo-angular-express"
active: docs
docs_active: examples
permalink: docs/examples/node-todo/
---


# About

You can find all the code for this example on GitHub in the
[rethinkdb-example-nodejs repository](https://github.com/rethinkdb/rethinkdb-example-nodejs/tree/master/todo-angular-express).

This application was originally an example to illustrate AngularJS taken from
[todomvc.com](http://todomvc.com/). The code was slightly modified to persist data with
a server running on Node.js with ExpressJS.

This article discusses only the server-side code and more precisely the file
[app.js](https://github.com/rethinkdb/rethinkdb-example-nodejs/blob/master/todo-angular-express/app.js)


# Note

This example is built with Express 4.0 and does not work with previous version of Express.

Express will be started by default on the port `3000`. The Node server will try to connect to
RethinkDB on `localhost:28015`.   
You can change these parameters in `config.js`.



# Code and comments

The server has two functions:

- Serve static files (HTML, CSS, JavaScript files)
- Provide a REST API to retrieve/save/update/delete todos


## Import modules

We first have to import some modules &mdash; `express`, `rethinkdb` and `body-parser`.
The `body-parser` module is used to parse the parameters of a HTTP request.

We also import the file `config.js` that contains some parameters for RethinkDB and Express.

```js
var express = require('express');
var bodyParser = require('body-parser');
var r = require('rethinkdb');

var config = require(__dirname+"/config.js")
```

Create an HTTP server with Express.

```js
var app = express();
```


## Sequence of middleware

Express will pass each request through the middleware. A middleware usually has one function
like parsing the cookie, parsing the header, opening a connection to the database, reading a static file
etc.

In our example, for each request we look if there is a file in the directory `public`
that matches the route. If we find such file, we serve it. If we do not, we will pass
the request to the next middleware.

The next middleware will form the REST API. Each request will go through:

- `bodyParser`: Parse the body and save it in `req.body`
- `createConnection`: Create a connection to RethinKDB and save it in `req._rdbConn`
- `get`/`create`/`update`/`del`: Perform an operation on the database depending on the route
- `closeConnection`: Close the connection to the database


This is how the sequence of middleware is defined.

```js
app.use(express.static(__dirname + '/public')); // Serve static content

app.use(bodyParser());                          // Parse data sent to the server
app.use(createConnection);                      // Create a RethinkDB connection

// Define the main routes
app.route('/todo/get').get(get);                // Retrieve all the todos
app.route('/todo/new').put(create);             // Create a new todo
app.route('/todo/update').post(update);         // Update a todo
app.route('/todo/delete').post(del);            // Delete a todo

app.use(closeConnection);                       // Close the RethinkDB connection previously opened
```


## Implement the middleware

Each middleware is a function that takes three arguments:

- `req`: The request we are currently serving.
- `res`: The response object that we will send back to the user.
- `next`: The function to call when the current middleware has finished his work.


_Create a connection_

Create a RethinkDB connection, and save it in `req._rdbConn`.

```js
function createConnection(req, res, next) {
    r.connect(config.rethinkdb, function(error, conn) {
        if (error) {
            handleError(res, error);
        }
        else {
            // Save the connection in `req`
            req._rdbConn = conn;
            // Pass the current request to the next middleware
            next();
        }
    });
}
```

`handleError` is a function that returns a 500 error to the client.

```js
function handleError(res, error) {
    return res.send(500, {error: error.message});
}
```


_Retrieve all the todos_

The next middleware retrieves all the todos from the database ordered by date.

The ReQL query consists of 3 parts:

- `r.table('todos')`: selects all the documents from the table `todos`.
- `orderBy({index: "createdAt"})`: order the todos with the index `createdAt`.
- `run(...)`: Execute the query

Because there can be many todos, the server returns a cursor. In our case, we just
retrieve all the elements of the cursor and save it in an array using the `toArray` command.

```js
function get(req, res, next) {
    r.table('todos').orderBy({index: "createdAt"}).run(req._rdbConn, function(error, cursor) {
        if (error) {
            handleError(res, error) 
            next();
        }
        else {
            // Retrieve all the todos in an array
            cursor.toArray(function(error, result) {
                if (error) {
                    handleError(res, error) 
                }
                else {
                    // Send back the data
                    res.send(JSON.stringify(result));
                }
            });
        }
    });
}
```

_Insert a todo_

Insert a new document in the table `todos` with the `insert` command.

```js
function create(req, res, next) {
    var todo = req.body;         // req.body was created by `bodyParser`
    todo.createdAt = r.now();    // Set the field `createdAt` to the current time

    r.table('todos').insert(todo, {returnVals: true}).run(req._rdbConn, function(error, result) {
        if (error) {
            handleError(res, error) 
        }
        else if (result.inserted !== 1) {
            handleError(res, new Error("Document was not inserted.")) 
        }
        else {
            res.send(JSON.stringify(result.new_val));
        }
        next();
    });
}
```


_Update a todo_

Update an existing todo.

We first select the todo with the `get` command, then call `update` on it.

```js
function update(req, res, next) {
    var todo = req.body;
    if ((todo != null) && (todo.id != null)) {
        r.table('todos').get(todo.id).update(todo, {returnVals: true}).run(req._rdbConn, function(error, result) {
            if (error) {
                handleError(res, error) 
            }
            else {
                res.send(JSON.stringify(result.new_val));
            }
            next();
        });
    }
    else {
        handleError(res, new Error("The todo must have a field `id`."))
        next();
    }
}
```



_Delete a todo_

Delete a todo given its `id`.

In a similar way as for the `update`, we first select the todo to delete, and then call `delete` on it.

```js
function del(req, res, next) {
    var todo = req.body;
    if ((todo != null) && (todo.id != null)) {
        r.table('todos').get(todo.id).delete().run(req._rdbConn, function(error, result) {
            if (error) {
                handleError(res, error) 
            }
            else {
                res.send(JSON.stringify(result));
            }
            next();
        });
    }
    else {
        handleError(res, new Error("The todo must have a field `id`."))
        next();
    }
}
```


_Close the connection to the database_

Close the connection to RethinkDB.

```js
function closeConnection(req, res, next) {
    req._rdbConn.close();
    next();
}
```





## Initialize the database and start Express.

Create the table and index then start express.

```js
r.connect(config.rethinkdb, function(err, conn) {
    if (err) {
        console.log("Could not open a connection to initialize the database");
        console.log(err.message);
        process.exit(1);
    }
    r.table('todos').indexWait('createdAt').run(conn, function(err, result) {
        if (err) {
            // The database/table/index was not available, create them

            r.dbCreate(config.rethinkdb.db).run(conn, function(err, result) {
                if ((err) && (!err.message.match(/Database `.*` already exists/))) {
                    console.log("Could not create the database `"+config.db+"`");
                    console.log(err);
                    process.exit(1);
                }
                console.log('Database `'+config.rethinkdb.db+'` created.');

                r.tableCreate('todos').run(conn, function(err, result) {
                    if ((err) && (!err.message.match(/Table `.*` already exists/))) {
                        console.log("Could not create the table `todos`");
                        console.log(err);
                        process.exit(1);
                    }
                    console.log('Table `todos` created.');

                    r.table('todos').indexCreate('createdAt').run(conn, function(err, result) {
                        if ((err) && (!err.message.match(/Index `.*` already exists/))) {
                            console.log("Could not create the index `todos`");
                            console.log(err);
                            process.exit(1);
                        }

                        console.log('Index `createdAt` created.');

                        r.table('todos').indexWait('createdAt').run(conn, function(err, result) {
                            if (err) {
                                console.log("Could not wait for the completion of the index `todos`");
                                console.log(err);
                                process.exit(1);
                            }
                            console.log('Index `createdAt` ready.');
                            console.log("Table and index are available, starting express...");

                            startExpress();
                            conn.close();
                        });
                    });
                });
            });
        }
        else {
            console.log("Table and index are available, starting express...");
            startExpress();
        }
    });

});

function startExpress() {
    app.listen(config.express.port);
    console.log('Listening on port '+config.express.port);
}
```
