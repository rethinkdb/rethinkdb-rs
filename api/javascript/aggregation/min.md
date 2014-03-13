---
layout: api-command
language: JavaScript 
permalink: api/javascript/min/
command: min
io:
    -   - sequence
        - value
related_commands:
    map: map/
    reduce: reduce/
    count: count/
    sum: sum/
    avg: avg/
    max: max/
    group: group/
---

# Command syntax #

{% apibody %}
sequence.min([fieldOrFunction]) &rarr; element
{% endapibody %}

# Description #

Finds the minimum of a sequence.  If called with a field name, finds
the element of that sequence with the smallest value in that field.
If called with a function, calls that function on every element of the
sequence and returns the element which produced the smallest value,
ignoring any elements where the function returns `null` or produces a
non-existence error.

Produces a non-existence error when called on an empty sequence.  You
can handle this case with `default`.

__Example:__ What's the minimum of 3, 5, and 7?

```js
r.expr([3, 5, 7]).min().run(conn, callback)
```

__Example:__ Which user has scored the fewest points?

```js
r.table('users').min('points').run(conn, callback)
```

__Example:__ Which user has scored the fewest points, counting bonus points?

```js
r.table('users').min(function(user) {
    return user('points').add(user('bonus_points'))
}).run(conn, callback)
```

__Example:__ What is the smallest number of points any user has ever scored?

```js
r.table('users').min('points')('points').run(conn, callback)
```

__Example:__ Which user has scored the fewest points?  (But return
`null` instead of erroring if no users have ever scored points.)

```js
r.table('users').min('points').default(null).run(conn, callback)
```
