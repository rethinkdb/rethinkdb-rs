---
layout: api-command 
language: JavaScript
permalink: api/javascript/div/
command: div
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/math-and-logic/div.md
io:
    -   - number
        - number
related_commands:
    add: add/
    sub: sub/
    mul: mul/
    mod: mod/
---

{% apibody %}
number.div(number) → number
{% endapibody %}

Divide two numbers.

__Example:__ It's as easy as 2 / 2 = 1.

```js
r.expr(2).div(2).run(conn, callback)
```

