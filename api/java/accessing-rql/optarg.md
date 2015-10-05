---
layout: api-command
language: Java
permalink: api/java/optarg/
command: optArg
---

# Command syntax #

{% apibody %}
term.optArg(option, value)
{% endapibody %}

# Description #

Specify an optional argument to a Java ReQL term.

Some terms in ReQL accept optional arguments. Since Java doesn't support named arguments, the RethinkDB Java driver allows you to pass them by chaining the `optArg` command after them.

__Example:__ Pass the `rightBound` optional argument to [between](/api/java/between/).

```java
r.table('marvel').between(10, 20).optArg('rightBound', 'closed').run(conn);
```

To pass more than one optional argument, chain `optArg` once for each argument.


__Example:__ Pass the `rightBound` and `index` optional arguments to [between](/api/java/between/).

```java
r.table('marvel').between(10, 20).optArg('rightBound', 'closed')
 .optArg('index', 'power').run(conn);
```
