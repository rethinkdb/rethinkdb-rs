---
layout: api-command 
language: JavaScript
permalink: api/javascript/connect/
command: connect
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/accessing-rql/connect.md
io:
    -   - r
        - undefined
related_commands:
    use: use/
    close: close/
---

# Command syntax #

{% apibody %}
r.connect(opts, callback)
{% endapibody %}

# Description #

Create a new connection to the database server.

If the connection cannot be established, a `RqlDriverError` exception will be thrown

__Example:__ Opens a new connection to the database.

```js
r.connect({host:'localhost', port:28015, db:'marvel', authKey:'hunter2'},
   function(err, conn) { ... })
```


