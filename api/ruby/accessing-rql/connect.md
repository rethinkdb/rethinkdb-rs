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

<img src="/assets/images/docs/api_illustrations/connect_ruby.png" class="api_command_illustration" />

Create a new connection to the database server.  Accepts the following
options:

- `host`: the host to connect to (default `localhost`).
- `port`: the port to connect on (default `28015`).
- `db`: the default database (default `test`).
- `auth_key`: the authentication key (default none).

If the connection cannot be established, a `RqlDriverError` exception will be thrown.

The authentication key can be set from the RethinkDB command line tool. Once set, client connections must provide the key as an option to `run` in order to make the connection. For more information, read "Using the RethinkDB authentication system" in the documentation on [securing your cluster](http://rethinkdb.com/docs/security/).

__Example:__ Opens a connection using the default host and port but specifying the default database.

```rb
conn = r.connect(:db => 'marvel')
```

__Example:__ Opens a new connection to the database.

```rb
conn = r.connect(:host => 'localhost',
                 :port => 28015,
                 :db => 'heroes',
                 :auth_key => 'hunter2')
```

__Example:__ Open a connection and immediately pass it to a Ruby block. Using this style, the connection will be automatically closed when execution reaches the end of the block.

```rb
r.connect(:db => 'marvel') { |conn|
    r.table('superheroes').run(conn)
}
```
