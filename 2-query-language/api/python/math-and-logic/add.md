---
layout: api-command 
language: Python
permalink: api/python/add/
command: +
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/math-and-logic/add.md
---

{% apibody %}
number + number &rarr; number
string + string &rarr; string
array + array &rarr; array
time + number &rarr; time
{% endapibody %}

Sum two numbers, concatenate two strings, or concatenate 2 arrays.

__Example:__ It's as easy as 2 + 2 = 4.

```py
(r.expr(2) + 2).run(conn)
```


__Example:__ Strings can be concatenated too.

```py
(r.expr("foo") + "bar").run(conn)
```


__Example:__ Arrays can be concatenated too.

```py
(r.expr(["foo", "bar"]) + ["buzz"]).run(conn)
```


__Example:__ Create a date one year from now.

```py
r.now() + 365*24*60*60
```

