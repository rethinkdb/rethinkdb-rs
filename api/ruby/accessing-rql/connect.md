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
r.connect(opts={}) &rarr; connection
{% endapibody %}

# Description #

Create a new connection to the database server.  Accepts the following
options:

- `host`: the host to connect to (default `localhost`).
- `port`: the port to connect on (default `28015`).
- `db`: the default database (default `test`).
- `authKey`: the authentication key (default none).

If the connection cannot be established, a `RqlDriverError` exception
will be thrown.

__Example:__ Opens a new connection to the database.

```rb
conn = r.connect(:host => 'localhost',
                 :port => 28015,
                 :db => 'heroes',
                 :auth_key => 'hunter2')
```
