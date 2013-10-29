---
layout: api-command 
language: JavaScript
permalink: api/javascript/is_empty/
command: isEmpty 
io:
    -   - sequence
        - bool
---

# Command syntax #

{% apibody %}
sequence.isEmpty() &rarr; bool
{% endapibody %}

# Description #

Test if a sequence is empty.

__Example:__ Are there any documents in the marvel table?

```js
r.table('marvel').isEmpty().run(conn, callback)
```
