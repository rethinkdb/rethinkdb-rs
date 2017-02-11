---
layout: api-command
language: Java
permalink: api/java/count/
command: count
related_commands:
    map: map/
    reduce: reduce/
    sum: sum/
    avg: avg/
    min: min/
    max: max/
    group: group/
---

# Command syntax #

{% apibody %}
sequence.count([value | predicate_function]) &rarr; number
binary.count() &rarr; number
string.count() &rarr; number
object.count() &rarr; number
r.count(sequence | binary | string | object[, predicate_function]) &rarr; number
{% endapibody %}

# Description #

Counts the number of elements in a sequence or key/value pairs in an object, or returns the size of a string or binary object.

When `count` is called on a sequence with a predicate value or function, it returns the number of elements in the sequence equal to that value or where the function returns `true`. On a [binary](/api/java/binary) object, `count` returns the size of the object in bytes; on strings, `count` returns the string's length. This is determined by counting the number of Unicode codepoints in the string, counting combining codepoints separately.

__Example:__ Count the number of users.

```java
r.table("users").count().run(conn);
```

__Example:__ Count the number of 18 year old users.

```java
r.table("users").g("age").count(18).run(conn);
```

__Example:__ Count the number of users over 18.

```java
r.table("users").g("age").count(age -> age.gt(18)).run(conn);
```

Alternatively:

```java
r.table("users").count(user -> user.g("age").gt(18)).run(conn);
```

__Example:__ Return the length of a Unicode string.

```java
r.expr("こんにちは").count().run(conn);
// returns: 5
```
