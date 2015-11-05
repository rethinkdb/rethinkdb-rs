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

Take one or more values as arguments and return an array.

__Example:__ Create an array.

```java
r.array(10, 20, 30).run(conn);
```

This is a ReQL equivalent to:

```java
int[] myArray = { 10, 20, 30 };
```
