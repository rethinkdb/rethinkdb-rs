---
layout: api-command 
language: Python
permalink: api/python/div/
command: '/'
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/math-and-logic/div.md
related_commands:
    '+': add/
    '-': sub/
    '*': mul/
    '%': mod/
---

{% apibody %}
number / number &rarr; number
{% endapibody %}

Divide two numbers.

__Example:__ It's as easy as 2 / 2 = 1.

```py
(r.expr(2) / 2).run(conn)
```
