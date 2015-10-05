---
layout: api-command
language: Java
permalink: api/java/connect/
command: connection
related_commands:
    use: use/
    close: close/
---

# Command syntax #

{% apibody %}
r.connection() &rarr; builder
{% endapibody %}

# Description #

<img src="/assets/images/docs/api_illustrations/connect_javascript.png" class="api_command_illustration" />

Create a new connection to the database server. `connection` returns a builder object with the following methods:

- `hostname()`: the host to connect to (default `localhost`).
- `port()`: the port to connect on (default `28015`).
- `dbname()`: the default database (default `test`).
- `authKey()`: the authentication key (default none).
- `timeout()`: timeout period in seconds for the connection to be opened (default `20`).
- `connect()`: instantiate a connection object with the parameters previously passed to the builder.

If the connection cannot be established, a `ReqlDriverError` will be thrown.

The authentication key can be set from the RethinkDB command line tool. Once set, client connections must provide the key as an option to `run` in order to make the connection. For more information, read "Using the RethinkDB authentication system" in the documentation on [securing your cluster](http://rethinkdb.com/docs/security/).

__Example:__ Open a connection using the default host and port, specifying the default database.

```java
conn = r.connection().connect();
```

__Example:__ Open a new connection, specifying parameters.

```java
conn = r.connection()
        .hostname('localhost')
        .port(28015)
        .dbname('marvel')
        .authKey('hunter2')
        .connect();
```
