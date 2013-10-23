---
layout: api-command 
language: JavaScript
permalink: api/javascript/eq/
command: eq
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/math-and-logic/eq.md
io:
    -   - value
        - bool
related_commands:
    and: and/
    or: or/
    ne: ne/
---

{% apibody %}
value.eq(value) &rarr; bool
{% endapibody %}

Test if two values are equal.

__Example:__ Does 2 equal 2?

```js
r.expr(2).eq(2).run(conn, callback)
```


