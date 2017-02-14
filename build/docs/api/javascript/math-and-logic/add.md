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
value.add(value[, value, ...]) &rarr; value
time.add(number[, number, ...]) &rarr; time
{% endapibody %}

# Description #

Sum two or more numbers, or concatenate two or more strings or arrays.

The `add` command can be called in either prefix or infix form; both forms are equivalent. Note that ReQL will not perform type coercion. You cannot, for example, `add` a string and a number together.

__Example:__ It's as easy as 2 + 2 = 4.

```javascript
> r.expr(2).add(2).run(conn, callback)
// result passed to callback
4
```

__Example:__ Concatenate strings.

```javascript
> r.expr("foo").add("bar", "baz").run(conn, callback)
// result passed to callback
"foobarbaz"
```


__Example:__ Concatenate arrays.

```javascript
> r.expr(["foo", "bar"]).add(["buzz"]).run(conn, callback)
// result passed to callback
[ "foo", "bar", "buzz" ]
```


__Example:__ Create a date one year from now.

```javascript
r.now().add(365*24*60*60).run(conn, callback)
```

__Example:__ Use [args](/api/javascript/args) with `add` to sum multiple values.

```javascript
> vals = [10, 20, 30];
> r.add(r.args(vals)).run(conn, callback);
// result passed to callback
60
```

__Example:__ Concatenate an array of strings with `args`.

```javascript
> vals = ['foo', 'bar', 'buzz'];
> r.add(r.args(vals)).run(conn, callback);
// result passed to callback
"foobarbuzz"
```
