---
layout: api-command 
language: JavaScript
permalink: api/javascript/toArray/
command: toArray 
py: false
rb: false
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/accessing-rql/toArray.md
io:
    -   - cursor
        - undefined
related_commands:
    next: next/
    hasNext: hasNext/
    each: each/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
cursor.toArray(callback)
{% endapibody %}

# Description #

Retrieve all results and pass them as an array to the given callback.

__Example:__ For small result sets it may be more convenient to process them at once as
an array.

```js
cur.toArray(function(err, results) {
    for(var i in results) {
        processRow(results[i]);
    }
});
```
