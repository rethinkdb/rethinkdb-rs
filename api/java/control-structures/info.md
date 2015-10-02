---
layout: api-command
language: JavaScript
permalink: api/javascript/info/
command: info
io:
    -   - any
        - object
---

# Command syntax #

{% apibody %}
any.info() &rarr; object
r.info(any) &rarr; object
{% endapibody %}

# Description #

Get information about a ReQL value.

__Example:__ Get information about a table such as primary key, or cache size.

```js
r.table('marvel').info().run(conn)
```
