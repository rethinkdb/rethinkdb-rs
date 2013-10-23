---
layout: api-command 
language: JavaScript
permalink: api/javascript/le/
command: le
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/math-and-logic/le.md
io:
    -   - value
        - bool
related_commands:
    eq: eq/
    ne: ne/
    gt: gt/
    ge: ge/
    lt: lt/
---

{% apibody %}
value.le(value) &rarr; bool
{% endapibody %}

Test if the first value is less than or equal to other.

__Example:__ Is 2 less than or equal to 2?

```js
r.expr(2).le(2).run(conn, callback)
```
