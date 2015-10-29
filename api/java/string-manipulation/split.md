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

Split a string into substrings. With no arguments, will split on whitespace; when called with a string as the first argument, will split using that string as a separator. A maximum number of splits can also be specified. (To specify `max_splits` while still splitting on whitespace, use `null` as the separator argument.)

Mimics the behavior of Python's `string.split` in edge cases, except
for splitting on the empty string, which instead produces an array of
single-character strings.

__Example:__ Split on whitespace.

```java
r.expr("foo  bar bax").split().run(conn);
```

Result:

```json
["foo", "bar", "bax"]
```

__Example:__ Split the entries in a CSV file.

```java
r.expr("12,37,,22,").split(",").run(conn);
```

Result:

```json
["12", "37", "", "22", ""]
```

__Example:__ Split a string into characters.

```java
r.expr("mlucy").split("").run(conn);
```

Result:

```json
["m", "l", "u", "c", "y"]
```

__Example:__ Split the entries in a CSV file, but only at most 3
times.

```java
r.expr("12,37,,22,").split(",", 3).run(conn);
```

Result:

```json
["12", "37", "", "22,"]
```

__Example:__ Split on whitespace at most once (i.e. get the first word).

```java
r.expr("foo  bar bax").split(null, 1).run(conn);
```

Result:

```json
["foo", "bar bax"]
```
