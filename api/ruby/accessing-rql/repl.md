---
layout: api-command 
language: Ruby
permalink: api/ruby/repl/
command: repl 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/accessing-rql/repl.md
related_commands:
    connect: connect/
    use: use/
    close: close/
---


# Command syntax #

{% apibody %}
connection.repl
{% endapibody %}

# Description #

Set the default connection to make REPL use easier. Allows calling run() without specifying a connection. 

Connection objects are not thread safe and repl connections should not be used in multi-threaded environments.

__Example:__ Set the default connection in REPL, and call `run()` without specifying the connection.

```rb
r.connect().repl
r.table('users').run
```


