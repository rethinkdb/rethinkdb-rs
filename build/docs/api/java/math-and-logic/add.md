---
layout: api-command
language: Java
permalink: api/java/add/
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

Sum two or more numbers, or concatenate two or more strings or arrays.

The `add` command can be called in either prefix or infix form; both forms are equivalent. Note that ReQL will not perform type coercion. You cannot, for example, `add` a string and a number together.

__Example:__ It's as easy as 2 + 2 = 4.

```java
r.expr(2).add(2).run(conn);

// Result:
4
```

__Example:__ Concatenate strings.

```java
r.expr("foo").add("bar", "baz").run(conn);

// Result:
"foobarbaz"
```


__Example:__ Concatenate arrays.

```java
r.expr(["foo", "bar"]).add(["buzz"]).run(conn);

// Result:
[ "foo", "bar", "buzz" ]
```


__Example:__ Create a date one year from now.

```java
r.now().add(365*24*60*60).run(conn);
```

__Example:__ Use [args](/api/java/args) with `add` to sum multiple values.

```java
int[] vals = { 10, 20, 30 };
r.add(r.args(vals)).run(conn);

// Result:
60
```

__Example:__ Concatenate an array of strings with `args`.

```java
String[] vals = { "foo", "bar", "buzz" };
r.add(r.args(vals)).run(conn);

// Result:
"foobarbuzz"
```
