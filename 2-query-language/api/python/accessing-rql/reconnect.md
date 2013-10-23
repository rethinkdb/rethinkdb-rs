---
layout: api-command 
language: Python
permalink: api/python/reconnect/
command: reconnect 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/accessing-rql/reconnect.md
related_commands:
    connect: connect/
    use: use/
    repl: repl/
---

# Command syntax #

{% apibody %}
connection.reconnect()
{% endapibody %}

# Description #

Close and attempt to reopen a connection. Has the effect of canceling any outstanding
request while keeping the connection open.

__Example:__ Cancel outstanding requests/queries that are no longer needed.

```py
conn.reconnect()
```

