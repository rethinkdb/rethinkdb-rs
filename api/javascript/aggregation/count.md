---
layout: api-command
language: JavaScript
permalink: api/javascript/count/
command: count
io:
    -   - sequence
        - number
    -   - binary
        - number
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
{% endapibody %}

# Description #

Counts the number of elements in a sequence or key/value pairs in an object, or returns the size of a string or binary object.

When `count` is called on a sequence with a predicate value or function, it returns the number of elements in the sequence equal to that value or where the function returns `true`. On a [binary](/api/javascript/binary) object, `count` returns the size of the object in bytes; on strings, `count` returns the string's length. This is determined by counting the number of Unicode codepoints in the string, counting combining codepoints separately.

__Example:__ Count the number of users.

```js
r.table('users').count().run(conn, callback);
```

__Example:__ Count the number of 18 year old users.

```js
r.table('users')('age').count(18).run(conn, callback);
```

__Example:__ Count the number of users over 18.

```js
r.table('users')('age').count(function(age) { 
    return age.gt(18)
}).run(conn, callback);
```

```js
r.table('users').count(function(user) {
    return user('age').gt(18)
}).run(conn, callback)
```

__Example:__ Return the length of a Unicode string.

```js
r.expr("こんにちは").count().run(conn, callback);
// Result passed to callback
5
```
