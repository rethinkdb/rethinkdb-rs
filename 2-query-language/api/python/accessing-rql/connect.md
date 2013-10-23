---
layout: api-command 
language: Python
permalink: api/python/connect/
command: connect
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/accessing-rql/connect.md
related_commands:
    use: use/
    repl: repl/
    close: close/
---

# Command syntax #

{% apibody %}
r.connect(host='localhost', port=28015, db='test', auth_key='') &rarr; connection
{% endapibody %}

# Description #

Create a new connection to the database server.

If the connection cannot be established, a `RqlDriverError` exception will be thrown

__Example:__ Opens a connection using the default host and port but specifying the default database.


