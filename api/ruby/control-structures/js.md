---
layout: api-command
language: Ruby
permalink: api/ruby/js/
command: js
---

# Command syntax #

{% apibody %}
r.js(jsString) &rarr; value
{% endapibody %}

# Description #

Create a javascript expression.

__Example:__ Concatenate two strings using Javascript'

```rb
r.js("'str1' + 'str2'").run(conn)
```

__Example:__ Select all documents where the 'magazines' field is greater than 5 by
running Javascript on the server.

```rb
r.table('marvel').filter(
r.js('(function (row) { return row.magazines > 5; })')).run(conn)
```


__Example:__ You may also specify a timeout in seconds (defaults to 5).

```rb
r.js('while(true) {}', :timeout => 1.3).run(conn)
```

