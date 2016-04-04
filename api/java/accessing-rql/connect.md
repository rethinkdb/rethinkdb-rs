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
- `user()`: the user account and password to connect as (default `"admin", ""`).
- `timeout()`: timeout period in seconds for the connection to be opened (default `20`).
- `connect()`: instantiate a connection object with the parameters previously passed to the builder.
- `certFile()`: a path to an SSL CA certificate.
- `sslContext()`: an instance of an [SSLContext](https://docs.oracle.com/javase/8/docs/api/javax/net/ssl/SSLContext.html) class to use for SSL connections.

Either `certFile` or `sslContext` must be supplied to make an SSL connection to the RethinkDB server. Only one should be used.

If the connection cannot be established, a `ReqlDriverError` will be thrown.

<!-- break -->

{% infobox %}
Using SSL with RethinkDB requires proxy software on the server, such as [Nginx][], [HAProxy][] or an SSL tunnel. RethinkDB will encrypt traffic and verify the CA certification to prevent [man-in-the-middle][mitm] attacks. Consult your proxy's documentation for more details.

[Nginx]: http://nginx.org/
[HAProxy]: http://www.haproxy.org/
[mitm]: http://en.wikipedia.org/wiki/Man-in-the-middle_attack

Alternatively, you may use RethinkDB's built-in [TLS support][tls].

[tls]: /docs/security/
{% endinfobox %}

__Example:__ Open a connection using the default host and port, specifying the default database.

```java
Connection conn = r.connection().connect();
```

__Example:__ Open a new connection, specifying parameters.

```java
Connection conn = r.connection()
    .hostname("localhost")
    .port(28015)
    .dbname("marvel")
    .connect();
```

__Example:__ Open a new connection, specifying a user/password combination for authentication.

```java
Connection conn = r.connection()
    .hostname("localhost")
    .port(28015)
    .dbname("marvel")
    .user("herofinder", "metropolis")
    .connect();
```

__Example:__ Open a new connection to the database using an SSL proxy.

```java
Connection conn = r.connection()
    .hostname("localhost")
    .port(28015)
    .dbname("marvel")
    .authKey("hunter2")
    .certFile("/path/to/ca.crt")
    .connect();
```
