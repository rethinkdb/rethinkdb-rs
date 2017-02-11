---
layout: api-command
language: Java
permalink: api/java/args/
command: args
related_commands:
    array: array/
---

# Command syntax #

{% apibody %}
r.args(array) &rarr; special
{% endapibody %}

# Description #

`r.args` is a special term that's used to splice an array of arguments
into another term.  This is useful when you want to call a variadic
term such as [getAll](/api/java/get_all/) with a set of arguments produced at runtime.

Note that `args` evaluates all its arguments before passing them into the parent term, even if the parent term otherwise allows lazy evaluation.

__Example:__ Get Alice and Bob from the table `people`.

```java
r.table("people").getAll("Alice", "Bob").run(conn);
// or
r.table("people").getAll(r.args(r.array("Alice", "Bob"))).run(conn);
```

__Example:__ Get all of Alice's children from the table `people`.

```java
// r.table("people").get("Alice") returns (in JSON)
// { "id": "Alice", "children": ["Bob, "Carol"] }
r.table("people").getAll(r.args(r.table("people").get("Alice").g("children"))).run(conn);
```
