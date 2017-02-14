---
layout: api-command
language: JavaScript
permalink: api/javascript/split/
command: split
io:
    -   - string
        - array
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

```javascript
r.expr("foo  bar bax").split().run(conn, callback)
```

Result:

```javascript
["foo", "bar", "bax"]
```

__Example:__ Split the entries in a CSV file.

```javascript
r.expr("12,37,,22,").split(",").run(conn, callback)
```

Result:

```javascript
["12", "37", "", "22", ""]
```

__Example:__ Split a string into characters.

```javascript
r.expr("mlucy").split("").run(conn, callback)
```

Result:

```javascript
["m", "l", "u", "c", "y"]
```

__Example:__ Split the entries in a CSV file, but only at most 3
times.

```javascript
r.expr("12,37,,22,").split(",", 3).run(conn, callback)
```

Result:

```javascript
["12", "37", "", "22,"]
```

__Example:__ Split on whitespace at most once (i.e. get the first word).

```javascript
r.expr("foo  bar bax").split(null, 1).run(conn, callback)
```

Result:

```javascript
["foo", "bar bax"]
```
