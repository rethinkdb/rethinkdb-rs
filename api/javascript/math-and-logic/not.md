---
layout: api-command 
language: JavaScript
permalink: api/javascript/not/
command: not 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/math-and-logic/not.md
io:
    -   - bool
        - bool
related_commands:
    eq: eq
    ne: ne/
---

# Command syntax #

{% apibody %}
bool.not() &rarr; bool
{% endapibody %}

# Description #
Compute the logical inverse (not).

__Example:__ Not true is false.

```js
r.expr(true).not().run(conn, callback)
```
