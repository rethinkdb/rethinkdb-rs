---
layout: api-command
language: Ruby
permalink: api/ruby/add/
command: '+'
related_commands:
    '-': sub/
    '*': mul/
    '/': div/
    '%': mod/
---

# Command syntax #

{% apibody %}
number + number &rarr; number
string + string &rarr; string
array + array &rarr; array
time + number &rarr; time
{% endapibody %}

# Description #

Sum two numbers, concatenate two strings, or concatenate 2 arrays.

__Example:__ It's as easy as 2 + 2 = 4.

```rb
> (r.expr(2) + 2).run(conn)

4
```

__Example:__ Strings can be concatenated too.

```rb
> (r("foo") + "bar").run(conn)

"foobar"
```


__Example:__ Arrays can be concatenated too.

```rb
> (r(["foo", "bar"]) + ["buzz"]).run(conn)

["foo", "bar", "buzz"]
```


__Example:__ Create a date one year from now.


```rb
r.now() + 365*24*60*60
```

__Example:__ Use [args](/api/ruby/args) with `add` to sum multiple values.

```rb
> r.add(r.args([10, 20, 30])).run(conn)

60
```

__Example:__ Concatenate an array of strings with `args`.

```rb
> r.add(r.args(['foo', 'bar', 'buzz'])).run(conn)

"foobarbuzz"
```
