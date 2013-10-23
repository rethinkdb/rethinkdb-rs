---
layout: api-command 
language: Ruby
permalink: api/ruby/or/
command: '|'
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/math-and-logic/or.md
related_commands:
    '&': and/
---

{% apibody %}
bool | bool → bool
{% endapibody %}

Compute the logical or of two values.

__Example:__ True or false ored is true?

```rb
(r.expr(True) | False).run(conn)
```


