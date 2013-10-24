---
layout: api-command 
language: Ruby
permalink: api/ruby/connect/
command: connect
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/accessing-rql/connect.md
related_commands:
    use: use/
    repl: repl/
    close: close/
---


# Command syntax #

{% apibody %}
r.connect(opts) &rarr; connection
{% endapibody %}

# Description #

Create a new connection to the database server.

If the connection cannot be established, a `RqlDriverError` exception will be thrown

__Example:__ Opens a connection using the default host and port but specifying the default database.



```rb
conn = r.connect(:host => 'localhost', :port => 28015, :db => 'heroes', :auth_key => 'hunter2')
```
