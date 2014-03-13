---
layout: api-command
language: JavaScript
permalink: api/javascript/max/
command: max
io:
    -   - sequence
        - value
related_commands:
    map: map/
    reduce: reduce/
    count: count/
    sum: sum/
    avg: avg/
    min: min/
    group: group/
---

# Command syntax #

{% apibody %}
sequence.max([fieldOrFunction]) &rarr; element
{% endapibody %}

# Description #

Finds the maximum of a sequence.  If called with a field name, finds
the element of that sequence with the largest value in that field.  If
called with a function, calls that function on every element of the
sequence and returns the element which produced the largest value,
ignoring any elements where the function returns `null` or produces a
non-existence error.

Produces a non-existence error when called on an empty sequence.  You
can handle this case with `default`.

__Example:__ What's the maximum of 3, 5, and 7?

```js
r.expr([3, 5, 7]).max().run(conn, callback)
```

__Example:__ Which user has scored the most points?

```js
r.table('users').max('points').run(conn, callback)
```

__Example:__ Which user has scored the most points, counting bonus points?

```js
r.table('users').max(function(user) {
    return user('points').add(user('bonus_points'))
}).run(conn, callback)
```

__Example:__ What is the largest number of points any user has ever scored?

```js
r.table('users').max('points')('points').run(conn, callback)
```

__Example:__ Which user has scored the most points?  (But return
`null` instead of erroring if no users have ever scored points.)

```js
r.table('users').max('points').default(null).run(conn, callback)
```
