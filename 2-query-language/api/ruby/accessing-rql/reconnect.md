---
layout: api-command 
language: Ruby
permalink: api/ruby/reconnect/
command: reconnect 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/accessing-rql/reconnect.md
related_commands:
    connect: connect/
    use: use/
    repl: repl/
    close: close/
---

{% apibody %}
connection.reconnect
{% endapibody %}

Close and attempt to reopen a connection. Has the effect of canceling any outstanding
request while keeping the connection open.

__Example:__ Cancel outstanding requests/queries that are no longer needed.

```rb
conn.reconnect
```


