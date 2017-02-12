---
layout: api-command
language: Java
permalink: api/java/array/
command: array
js: false
py: false
rb: false
related_commands:
    hashMap: hashmap/
---

# Command syntax #

{% apibody %}
r.array(value[, value...]) &rarr; array
{% endapibody %}

# Description #

Take one or more values as arguments and return an array. (Technically, return a [List][] object.)

[List]: https://docs.oracle.com/javase/8/docs/api/java/util/List.html

__Example:__ Create an array.

```java
r.expr(r.array(10, 20, 30)).run(conn);
```

This is a ReQL equivalent to:

```java
List<Integer> myArray = Arrays.asList(10, 20, 30);
```
