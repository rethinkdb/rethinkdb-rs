---
layout: api-command 
language: JavaScript
permalink: api/javascript/close/
command: close
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/accessing-rql/close.md
io:
    -   - connection
        - undefined
related_commands:
    connect: connect/
    use: use/
---

# Command syntax #

{% apibody %}
conn.close()
{% endapibody %}

# Description #

Close an open connection. Closing a connection cancels all outstanding requests and frees
the memory associated with the open requests.

__Example:__ Close an open connection.

```js
conn.close()
```


