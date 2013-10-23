---
layout: api-command 
language: Python
permalink: api/python/sub/
command: '-'
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/math-and-logic/sub.md
related_commands:
    '+': add/
    '*': mul/
    '/': div/
    '%': mod/
---

{% apibody %}
number - number → number
time - time → number
time - number → time
{% endapibody %}

Subtract two numbers.

__Example:__ It's as easy as 2 - 2 = 0.

```py
(r.expr(2) - 2).run(conn)
```


__Example:__ Create a date one year ago today.

```py
r.now() - 365*24*60*60
```


__Example:__ Retrieve how many seconds elapsed between today and date

```py
r.now() - date
```

