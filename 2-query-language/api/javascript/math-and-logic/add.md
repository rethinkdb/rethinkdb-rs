---
layout: api-command 
permalink: api/javascript/add/
command: add
---

{% apibody %}
number.add(number) → number
string.add(string) → string
array.add(array) → array
time.add(number) → time
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

