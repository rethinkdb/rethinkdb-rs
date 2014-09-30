---
layout: api-command
language: JavaScript
permalink: api/javascript/add/
command: add
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

# Command syntax #

{% apibody %}
number.add(number) &rarr; number
string.add(string) &rarr; string
array.add(array) &rarr; array
time.add(number) &rarr; time
{% endapibody %}

# Description #

Sum two numbers, concatenate two strings, or concatenate 2 arrays.

__Example:__ It's as easy as 2 + 2 = 4.

```js
> r.expr(2).add(2).run(conn, callback)
// result passed to callback
4
```

__Example:__ Strings can be concatenated too.

```js
> r.expr("foo").add("bar").run(conn, callback)
// result passed to callback
"foobar"
```


__Example:__ Arrays can be concatenated too.

```js
> r.expr(["foo", "bar"]).add(["buzz"]).run(conn, callback)
// result passed to callback
[ "foo", "bar", "buzz" ]
```


__Example:__ Create a date one year from now.

```js
r.now().add(365*24*60*60).run(conn, callback)
```

__Example:__ Use [args](/api/javascript/args) with `add` to sum multiple values.

```js
> r.add(r.args([10, 20, 30])).run(conn, callback);
// result passed to callback
60
```

__Example:__ Concatenate an array of strings with `args`.

```js
> r.add(r.args(['foo', 'bar', 'buzz'])).run(conn, callback);
// result passed to callback
"foobarbuzz"
```
