---
layout: api-command 
language: JavaScript
permalink: api/javascript/ge/
command: ge
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/math-and-logic/ge.md
io:
    -   - value
        - bool
related_commands:
    eq: eq/
    ne: ne/
    gt: gt/
    lt: lt/
    le: le/
---

{% apibody %}
value.ge(value) → bool
{% endapibody %}

Test if the first value is greater than or equal to other.

__Example:__ Is 2 greater than or equal to 2?

```js
r.expr(2).ge(2).run(conn, callback)
```
