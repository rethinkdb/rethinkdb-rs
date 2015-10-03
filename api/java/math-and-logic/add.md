---
layout: api-command
language: Java
permalink: api/javascript/add/
command: add
related_commands:
    sub: sub/
    mul: mul/
    div: div/
    mod: mod/
---

# Command syntax #

{% apibody %}
value.add(value[, value, ...]) &rarr; value
time.add(number[, number, ...]) &rarr; time
{% endapibody %}

# Description #

Sum two or more numbers, or concatenate two or more strings or arrays. (Note that ReQL will not perform type coercion. You cannot, for example, `add` a string and a number together.) The `add` command can be called in either prefix or infix form; both forms are equivalent.

__Example:__ It's as easy as 2 + 2 = 4.

```js
> r.expr(2).add(2).run(conn)
// result passed to callback
4
```

__Example:__ Concatenate strings.

```js
> r.expr("foo").add("bar", "baz").run(conn)
// result passed to callback
"foobarbaz"
```


__Example:__ Concatenate arrays.

```js
> r.expr(["foo", "bar"]).add(["buzz"]).run(conn)
// result passed to callback
[ "foo", "bar", "buzz" ]
```


__Example:__ Create a date one year from now.

```js
r.now().add(365*24*60*60).run(conn)
```

__Example:__ Use [args](/api/javascript/args) with `add` to sum multiple values.

```js
> vals = [10, 20, 30];
> r.add(r.args(vals)).run(conn);
// result passed to callback
60
```

__Example:__ Concatenate an array of strings with `args`.

```js
> vals = ['foo', 'bar', 'buzz'];
> r.add(r.args(vals)).run(conn);
// result passed to callback
"foobarbuzz"
```
