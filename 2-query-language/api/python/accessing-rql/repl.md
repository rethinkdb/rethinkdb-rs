---
layout: api-command 
language: Python
permalink: api/python/repl/
command: repl 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/accessing-rql/repl.md
related_commands:
    connect: connect/
    use: use/
    close: close/
---

{% apibody %}
connection.repl()
{% endapibody %}

Set the default connection to make REPL use easier. Allows calling run() without specifying a connection. 

Connection objects are not thread safe and repl connections should not be used in multi-threaded environments.

__Example:__ Set the default connection in REPL, and call `run()` without specifying the connection.

```py
r.connect().repl()
r.table('users').run()
```


