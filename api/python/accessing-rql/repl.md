---
layout: api-command
language: Python
permalink: api/python/repl/
command: repl
js: false
related_commands:
    connect: connect/
    use: use/
    close: close/
---

# Command syntax #

{% apibody %}
conn.repl()
{% endapibody %}

# Description #

Set the default connection to make REPL use easier. Allows calling
`.run()` on queries without specifying a connection.

Connection objects are not thread-safe and REPL connections should not
be used in multi-threaded environments.

__Example:__ Set the default connection for the REPL, then call
`run()` without specifying the connection.

```py
r.connect(db='marvel').repl()
r.table('heroes').run()
```
