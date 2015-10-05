---
layout: api-command
language: Java
permalink: api/java/split/
command: split
related_commands:
    upcase: upcase/
    downcase: downcase/
    match: match/
---

# Command syntax #

{% apibody %}
string.split([separator, [max_splits]]) &rarr; array
{% endapibody %}

<img src="/assets/images/docs/api_illustrations/split.png" class="api_command_illustration" />

# Description #

Splits a string into substrings.  Splits on whitespace when called
with no arguments.  When called with a separator, splits on that
separator.  When called with a separator and a maximum number of
splits, splits on that separator at most `max_splits` times.  (Can be
called with `null` as the separator if you want to split on whitespace
while still specifying `max_splits`.)

Mimics the behavior of Python's `string.split` in edge cases, except
for splitting on the empty string, which instead produces an array of
single-character strings.

__Example:__ Split on whitespace.

```java
r.expr("foo  bar bax").split().run(conn)
```

Result:

```java
["foo", "bar", "bax"]
```

__Example:__ Split the entries in a CSV file.

```java
r.expr("12,37,,22,").split(",").run(conn)
```

Result:

```java
["12", "37", "", "22", ""]
```

__Example:__ Split a string into characters.

```java
r.expr("mlucy").split("").run(conn)
```

Result:

```java
["m", "l", "u", "c", "y"]
```

__Example:__ Split the entries in a CSV file, but only at most 3
times.

```java
r.expr("12,37,,22,").split(",", 3).run(conn)
```

Result:

```java
["12", "37", "", "22,"]
```

__Example:__ Split on whitespace at most once (i.e. get the first word).

```java
r.expr("foo  bar bax").split(null, 1).run(conn)
```

Result:

```java
["foo", "bar bax"]
```
