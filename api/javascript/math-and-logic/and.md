---
layout: api-command 
language: JavaScript
permalink: api/javascript/and/
command: and 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/math-and-logic/and.md
io:
    -   - bool
        - bool
related_commands:
    or: or/
    eq: eq/
    ne: ne/
---

# Command syntax #

{% apibody %}
bool.and(bool) &rarr; bool
{% endapibody %}

# Description #

Compute the logical and of two values.

__Example:__ True and false anded is false?

```js
r.expr(true).and(false).run(conn, callback)
```
