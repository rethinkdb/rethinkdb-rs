---
layout: api-command 
language: JavaScript
permalink: api/javascript/ne/
command: ne 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/math-and-logic/ne.md
io:
    -   - value
        - bool
related_commands:
    and: and/
    or: or/
    eq: eq/
---

{% apibody %}
value.ne(value) → bool
{% endapibody %}

Test if two values are not equal.

__Example:__ Does 2 not equal 2?

```js
r.expr(2).ne(2).run(conn, callback)
```
