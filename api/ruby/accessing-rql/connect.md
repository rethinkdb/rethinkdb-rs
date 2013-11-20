---
layout: api-command 
language: Ruby
permalink: api/ruby/connect/
command: connect
related_commands:
    use: use/
    repl: repl/
    close: close/
---


# Command syntax #

{% apibody %}
r.connect(options) &rarr; connection
r.connect(host) &rarr; connection
{% endapibody %}

# Description #

Create a new connection to the database server. The available options are:

- `host`: host of the RethinkDB instance. The default value is `localhost`.
- `port`: the driver port, by default `28015`.
- `db`: the database used if not explicitly specified in a query, by default `test`.
- `auth_key`: the authentification key, by default the empty string.


If the connection cannot be established, a `RqlDriverError` exception will be thrown


__Example:__ Opens a new connection to the database.

```rb
conn = r.connect(:host => 'localhost', :port => 28015, :db => 'marvel', :auth_key => 'hunter2')
```


__Example:__ Opens a new connection to the database by just specifying the host.

```js
conn = r.connect("localhost")
```
