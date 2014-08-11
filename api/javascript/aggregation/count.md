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
sequence.count([value_or_predicate]) &rarr; number
binary.count() &rarr; number
{% endapibody %}

# Description #

Counts the number of elements in a sequence.  If called with a value,
counts the number of times that value occurs in the sequence.  If
called with a predicate function, counts the number of elements in the
sequence where that function returns `true`.

If `count` is called on a [binary](/api/javascript/binary) object, it will return the size of the object in bytes.

__Example:__ Count the number of users.

```js
r.table('users').count().run(conn, callback)
```

__Example:__ Count the number of 18 year old users.

```js
r.table('users')('age').count(18).run(conn, callback)
```

__Example:__ Count the number of users over 18.

```js
r.table('users')('age').count(function(age) { 
    return age.gt(18)
}).run(conn, callback)
```

```js
r.table('users').count(function(user) {
    return user('age').gt(18)
}).run(conn, callback)
```
