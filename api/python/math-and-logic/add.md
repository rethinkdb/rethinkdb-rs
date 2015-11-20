---
layout: api-command
language: Python
permalink: api/python/add/
command: +
related_commands:
    '-': sub/
    '*': mul/
    '/': div/
    '%': mod/
---

# Command syntax #

{% apibody %}
value + value &rarr; value
time + number &rarr; time
value.add(value[, value, ...]) &rarr; value
time.add(number[, number, ...]) &rarr; time
{% endapibody %}

# Description #

Sum two or more numbers, or concatenate two or more strings or arrays. (Note that ReQL will not perform type coercion. You cannot, for example, `add` a string and a number together.) The `add` command can be called in either prefix or infix form; both forms are equivalent.

__Example:__ It's as easy as 2 + 2 = 4.

```py
> (r.expr(2) + 2).run(conn)

4
```

__Example:__ Concatenate strings.

```py
> (r.expr("foo") + "bar" + "baz").run(conn)

"foobarbaz"
```


__Example:__ Concatenate arrays.

```py
> (r.expr(["foo", "bar"]) + ["buzz"]).run(conn)

["foo", "bar", "buzz"]
```

__Example:__ Create a date one year from now.


```py
(r.now() + 365*24*60*60).run(conn)
```

__Example:__ Use [args](/api/python/args) with `add` to sum multiple values.

```py
> vals = [10, 20, 30]
> r.add(r.args(vals)).run(conn)

60
```

__Example:__ Concatenate an array of strings with `args`.

```py
> vals = ['foo', 'bar', 'buzz']
> r.add(r.args(vals)).run(conn)

"foobarbuzz"
```
