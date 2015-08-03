---
layout: documentation
title: Working with Node.js and RethinkDB
docs_active: nodejs
permalink: docs/nodejs/
---

# Libraries

## Drivers


RethinkDB provides an official Node.js driver. You can install it with npm:

```
npm install rethinkdb
```


An alternative community supported Node.js driver is
[rethinkdbdash](https://github.com/neumino/rethinkdbdash).
The main difference with the official driver is the presence of a connection pool.


## ORMs

- [thinky](https://github.com/neumino/thinky) by [@neumino](https://github.com/neumino)  
  JavaScript ORM for RethinkDB, using the same syntax as the driver.

- [reheat](https://github.com/jmdobry/reheat) by [@jmdobry](https://github.com/jmdobry)  
  JavaScript ORM for RethinkDB.

- [JugglingDB-RethinkDB](https://github.com/fuwaneko/jugglingdb-rethink) by [@fuwaneko](https://github.com/fuwaneko)  
  A RethinkDB adapter for [JugglingDB](https://github.com/1602/jugglingdb), a multi-database ORM for Node.js.

- [Osmos](https://github.com/mtabini/osmos) by [@mtabini](https://github.com/mtabini)  
  A store-agnostic object data mapper for Node.js with support for RethinkDB.



# Examples

You can find three guides to build a classic TODO application:

- A simple implementation with ExpressJS, AngularJS and RethinkDB.  
Take a look at [the article](/docs/examples/node-todo/) or the
[source code](https://github.com/rethinkdb/rethinkdb-example-nodejs/tree/master/todo-angular-express)
- A implementation using __promises__, with ExpressJS, AngularJS and RethinkDB.  
Take a look at [the article](/docs/examples/node-todo-promises/) or the
[source code](https://github.com/rethinkdb/rethinkdb-example-nodejs/tree/master/todo-angular-express-promise)
- A implementation using __generators__, with KoaJS, AngularJS and RethinkDB.  
Take a look at [the article](/docs/examples/node-todo-koa/) or the
[source code](https://github.com/rethinkdb/rethinkdb-example-nodejs/tree/master/todo-angular-koa)
