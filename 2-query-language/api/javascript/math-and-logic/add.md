---
layout: api-command 
language: JavaScript
permalink: api/javascript/add/
command: add
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/math-and-logic/add.md
io:
    -   - number
        - number
    -   - string
        - string
    -   - array
        - array
    -   - time
        - time
    -   - time
        - number
related_commands:
    sub: sub/
    mul: mul/
    div: div/
    mod: mod/
---

{% apibody %}
number.add(number) &rarr; number
string.add(string) &rarr; string
array.add(array) &rarr; array
time.add(number) &rarr; time
{% endapibody %}

Sum two numbers, concatenate two strings, or concatenate 2 arrays.

__Example:__ It's as easy as 2 + 2 = 4.

```js
r.expr(2).add(2).run(conn, callback)
```

__Example:__ Strings can be concatenated too.

```js
r.expr("foo").add("bar").run(conn, callback)
```


__Example:__ Arrays can be concatenated too.

```js
r.expr(["foo", "bar"]).add(["buzz"]).run(conn, callback)
```


__Example:__ Create a date one year from now.

```js
r.now().add(365*24*60*60)
```

