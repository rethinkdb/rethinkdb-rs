---
layout: api-command 
language: Python
permalink: api/python/not/
command: '~'
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/math-and-logic/not.md
related_commands:
    '==': eq/
    '!=': ne/
---

{% apibody %}
~bool &rarr; bool
{% endapibody %}
Compute the logical inverse (not).

__Example:__ Not true is false.

```py
(~r.expr(True)).run(conn)
```
