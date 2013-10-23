---
layout: api-command 
language: Ruby
permalink: api/ruby/mod/
command: '%'
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/math-and-logic/mod.md
related_commands:
    '+': add/
    '-': sub/
    '*': mul/
    '/': div/
---

# Command syntax #

{% apibody %}
number % number &rarr; number
{% endapibody %}

# Description #

Find the remainder when dividing two numbers.

__Example:__ It's as easy as 2 % 2 = 0.

```rb
(r.expr(2) % 2).run(conn)
```


