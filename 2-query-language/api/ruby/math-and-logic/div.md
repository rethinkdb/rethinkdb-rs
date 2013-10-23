---
layout: api-command 
language: Ruby
permalink: api/ruby/div/
command: '/'
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/math-and-logic/div.md
related_commands:
    '+': add/
    '-': sub/
    '*': mul/
    '%': mod/
---

# Command syntax #

{% apibody %}
number / number &rarr; number
{% endapibody %}

# Description #

Divide two numbers.

__Example:__ It's as easy as 2 / 2 = 1.

```rb
(r.expr(2) / 2).run(conn)
```


