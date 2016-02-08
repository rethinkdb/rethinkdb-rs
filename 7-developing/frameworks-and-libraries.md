---
layout: documentation
title: Third-party libraries
docs_active: frameworks-and-libraries
permalink: docs/frameworks-and-libraries/
---

{% toctag %}

{% infobox %}
__Add your project:__ Have you written a cool library or tool for RethinkDB?
Shoot us an email at <a href="mailto:info@rethinkdb.com">info@rethinkdb.com</a>.
{% endinfobox %}

# Node.js libraries

## Drivers and extensions

- [rethinkdbdash](https://github.com/neumino/rethinkdbdash) by [@neumino](https://github.com/neumino)  
  An alternative Node.js driver with native promises and a connection pool.

- [RQL Promise](https://github.com/guillaumervls/rql-promise) by [@guillaumervls](https://github.com/guillaumervls)  
   Wraps the RethinkDB driver with [when](https://github.com/cujojs/when) to return promises.

- [rethinkdb-co](https://github.com/hden/rethinkdb-co) by [@hden](https://github.com/hden)  
  Allows using ECMAScript 6 generators with RethinkDB callbacks.

- [rdb-cursor-stream](https://github.com/guillaumervls/rdb-cursor-stream) by [@guillaumervls](https://github.com/guillaumervls)  
  Replaces cursors with streams.

- [connect-rethinkdb](https://github.com/guillaumervls/connect-rethinkdb) by [@guillaumervls](https://github.com/guillaumervls)  
  A RethinkDB session store for Connect, similar to connect-redis.

- [express-session-rethinkdb](https://www.npmjs.com/package/express-session-rethinkdb) by [@armenfilipetyan](https://github.com/armenfilipetyan)  
  Use RethinkDB as session store with Express 4.x framework. Adapted from connect-rethinkdb.

- [Rethinkdb-pool](https://github.com/hden/rethinkdb-pool) by [@hden](https://github.com/hden)  
  Connection pool for RethinkDB connections.

- [second-thought](https://github.com/robconery/second-thought) by [@robconery](https://github.com/robconery)  
  A light abstraction layer over RethinkDB adding methods you "wished you had."

- [rethinkdb-fixtures](https://github.com/athlite/rethinkdb-fixtures) by [@athlite](https://github.com/athlite)  
  Easily load fixtures into RethinkDB for testing purposes.

## ORMs

- [js-data-rethinkdb](https://github.com/js-data/js-data-rethinkdb) by [@jmdobry](https://github.com/jmdobry)  
  A RethinkDB adapter for [js-data](https://github.com/js-data/js-data), a database-agnostic ORM for Node.js and the browser.

- [Thinky][] by [@neumino](https://github.com/neumino)  
  JavaScript ORM for RethinkDB.

- [JugglingDB-RethinkDB](https://github.com/fuwaneko/jugglingdb-rethink) by [@fuwaneko](https://github.com/fuwaneko)  
  A RethinkDB adapter for [JugglingDB](https://github.com/1602/jugglingdb), a multi-database ORM for Node.js.

- [Osmos](https://github.com/mtabini/osmos) by [@mtabini](https://github.com/mtabini)  
  A store-agnostic object data mapper for Node.js with support for RethinkDB.

[Thinky]: https://github.com/neumino/thinky

## Integrations

- [rabbitMQ](/docs/rabbitmq/javascript)  
  Listen for changes in a RethinkDB table over RabbitMQ.

- [koa-rethinkdb](https://github.com/hden/koa-rethinkdb) by [@hden](https://github.com/hden)  
  Koa middleware that automatically manages connections via a connection pool.

- [sails-hook-thinky](https://github.com/mwielbut/sails-hook-thinky) by [@mwielbut](https://github.com/mwielbut)  
  A hook to use [Thinky][] with [Sails](http://sailsjs.org/).

- [hapi-rethinkdb-crud](https://github.com/athlite/hapi-rethinkdb-crud) by [@athlite](https://github.com/athlite)  
  Basic CRUD mapping between [hapi](http://hapijs.com) and RethinkDB.

# Python libraries


## ORMs

- [rwrapper](https://github.com/dparlevliet/rwrapper) by [@dparlevliet](https://github.com/dparlevliet)  
  An ORM designed to emulate the most common usages of Django's database abstraction.

- [pyRethinkORM](https://github.com/JoshAshby/pyRethinkORM) by [@JoshAshby](https://github.com/JoshAshby)  
  A Python ORM for RethinkDB.

- [rethink](https://github.com/caoimhghin/rethink) by [@caoimhghin](https://github.com/caoimhghin)  
  A Python object mapper in the style of [App Engine NDB](https://cloud.google.com/appengine/docs/python/ndb/).

- [remodel](https://github.com/linkyndy/remodel) by [@linkyndy](https://github.com/linkyndy)  
  A simple but powerful and extensible Object Document Mapper for RethinkDB. (Also see the [pip package](https://pypi.python.org/pypi/Remodel).)

## Integrations

- [rabbitMQ](/docs/rabbitmq/python)  
  Listen for changes in a RethinkDB table over RabbitMQ.

- [celery-backends-rethinkdb](https://github.com/pilwon/celery-backends-rethinkdb) by [@pilwon](https://github.com/pilwon)  
  [Celery](http://www.celeryproject.org/)'s custom result backend for RethinkDB.

- [flask-rethinkdb](https://github.com/linkyndy/flask-rethinkdb) by [@linkyndy](https://github.com/linkyndy)  
  A Flask extension that adds RethinkDB support (also see the [pip package](https://pypi.python.org/pypi/Flask-RethinkDB/)).

# Ruby libraries

## ORMs

- [NoBrainer](https://github.com/nviennot/nobrainer) by [@nviennot](https://github.com/nviennot)  
  A Ruby ORM designed for RethinkDB. (Read our [quickstart tutorial](/docs/rails).)

## Integrations

- [Epiphy](https://github.com/kureikain/epiphy) by [@kureikain](https://github.com/kureikain)   
  A light persistence framework for RethinkDB.

- [rabbitMQ](/docs/rabbitmq/ruby)  
  Listen for changes in a RethinkDB table over RabbitMQ.

- [lotus-rethinkdb](https://github.com/angeloashmore/lotus-rethinkdb) by [@angeloashmore](https://github.com/angeloashmore)  
  A RethinkDB adapter for [Lotus::Model](https://github.com/lotus/model), a persistence framework for Ruby.
