---
layout: api-command 
language: JavaScript
permalink: api/javascript/sub/
command: sub
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/math-and-logic/sub.md
io:
    -   - number
        - number
    -   - time
        - time
    -   - time
        - number
related_commands:
    add: add/
    mul: mul/
    div: div/
    mod: mod/
---

{% apibody %}
number.sub(number) → number
time.sub(time) → number
time.sub(number) → time
{% endapibody %}

Subtract two numbers.

__Example:__ It's as easy as 2 - 2 = 0.

```js
r.expr(2).sub(2).run(conn, callback)
```


__Example:__ Create a date one year ago today.

```js
r.now().sub(365*24*60*60)
```


__Example:__ Retrieve how many seconds elapsed between today and date

```js
r.now().sub(date)
```

