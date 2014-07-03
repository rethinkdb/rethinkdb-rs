---
layout: api-command
language: Python
permalink: api/python/js/
command: js
---

# Command syntax #

{% apibody %}
r.js(js_string[, timeout=<number>]) &rarr; value
{% endapibody %}

# Description #

Create a javascript expression.

__Example:__ Concatenate two strings using Javascript'

`timeout` is the number of seconds before `r.js` times out. The default value is 5 seconds.

```py
r.js("'str1' + 'str2'").run(conn)
```

__Example:__ Select all documents where the 'magazines' field is greater than 5 by running Javascript on the server.

```py
r.table('marvel').filter(
    r.js('(function (row) { return row.magazines.length > 5; })')
).run(conn)
```


__Example:__ You may also specify a timeout in seconds (defaults to 5).

```py
r.js('while(true) {}', timeout=1.3).run(conn)
```

