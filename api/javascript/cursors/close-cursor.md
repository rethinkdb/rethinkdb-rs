---
layout: api-command
language: JavaScript
permalink: api/javascript/close-cursor/
command: close
io:
    -   - cursor
        - undefined
related_commands:
    next: next/
    toArray: to_array/
    each: each/
---

# Command syntax #

{% apibody %}
cursor.close([callback])
cursor.close() &rarr; promise
{% endapibody %}

# Description #

Close a cursor. Closing a cursor cancels the corresponding query and frees the memory associated with the open request.

The `close` command can optionally take a callback, which will receive an error if one occurs, or return a promise which will be resolved when the connection is closed.

__Example:__ Close a cursor.

```javascript
cursor.close(function (err) {
    if (err) {
        console.log("An error occurred on cursor close");
    }
});
```

__Example:__ Close a cursor and execute a function upon close.

```javascript
cursor.close()
    .then(function () {
        console.log("The cursor has been closed");
    })
    .catch(r.Error.ReqlDriverError, function (err) {
        console.log("An error occurred on cursor close");
    });
```
