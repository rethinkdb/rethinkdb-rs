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
{% endapibody %}

# Description #

Counts the number of elements in a sequence.  If called with a value,
counts the number of times that value occurs in the sequence.  If
called with a predicate function, counts the number of elements in the
sequence where that function returns `true`.

If `count` is called on a [binary](/api/java/binary) object, it will return the size of the object in bytes.

__Example:__ Count the number of users.

```java
r.table('users').count().run(conn)
```

__Example:__ Count the number of 18 year old users.

```java
r.table('users')('age').count(18).run(conn)
```

__Example:__ Count the number of users over 18.

```java
r.table('users')('age').count(function(age) { 
    return age.gt(18)
}).run(conn)
```

```java
r.table('users').count(function(user) {
    return user('age').gt(18)
}).run(conn)
```
