---
layout: api-command
language: Java
permalink: api/java/range/
command: range
io:
    -   - r
        - stream
---
# Command syntax #

{% apibody %}
r.range() &rarr; stream
r.range([startValue, ]endValue) &rarr; stream
{% endapibody %}

# Description #

Generate a stream of sequential integers in a specified range. `range` takes 0, 1 or 2 arguments:

* With no arguments, `range` returns an "infinite" stream from 0 up to and including the maximum integer value;
* With one argument, `range` returns a stream from 0 up to but not including the end value;
* With two arguments, `range` returns a stream from the start value up to but not including the end value.

Note that the left bound (including the implied left bound of 0 in the 0- and 1-argument form) is always closed and the right bound is always open: the start value will always be included in the returned range and the end value will *not* be included in the returned range.

Any specified arguments must be integers, or a `ReqlRuntimeError` will be thrown. If the start value is equal or to higher than the end value, no error will be thrown but a zero-element stream will be returned.

__Example:__ Return a four-element range of `[0, 1, 2, 3]`.

```java
r.range(4).run(conn);
```

Result (shown as JSON):

```json
[0, 1, 2, 3]
```

You can also use the [limit](/api/java/limit) command with the no-argument variant to achieve the same result in this case:

```java
r.range().limit(4).run(conn);
```

__Example:__ Return a range from -5 through 5.

```java
r.range(-5, 6).run(conn);
```

```json
[-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5]
```
