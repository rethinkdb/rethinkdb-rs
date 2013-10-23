---
layout: api-command 
language: JavaScript
permalink: api/javascript/type_of/
command: typeOf 
github_doc: https://github.com/rethinkdb/docs/blob/docs/2-query-language/api/javascript/control-structures/typeOf.md
io:
    -   - any
        - string
---

{% apibody %}
any.typeOf() &rarr; string
{% endapibody %}

Gets the type of a value.

__Example:__ Get the type of a string.

```js
r.expr("foo").typeOf().run(conn, callback)
```

