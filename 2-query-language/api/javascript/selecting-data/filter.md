---
layout: api-command 
language: JavaScript
permalink: api/javascript/filter/
command: filter
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/selecting-data/filter.md
---

{% apibody %}
sequence.filter(predicate) &rarr; selection
stream.filter(predicate) &rarr; stream
array.filter(predicate) &rarr; array
{% endapibody %}

Get all the documents for which the given predicate is true.

filter can be called on a sequence, selection, or a field containing an array of
elements. The return type is the same as the type on which the function was called on.
The body of every filter is wrapped in an implicit `.default(false)`, and the default
value can be changed by passing the optional argument `default`. Setting this optional
argument to `r.error()` will cause any non-existence errors to abort the filter.

__Example:__ Get all active users aged 30.

```js
r.table('users').filter({active: true, profile: {age: 30}}).run(conn, callback)
```

__Example:__ Filter supports the r.literal syntax if you want to get an exact match.

```js
r.table('users').filter({active: true, profile: r.literal({age: 30})}).run(conn, callback)
```

__Example:__ Select all documents where the 'magazines' field is greater than 5.

```js
r.table('marvel').filter(r.row('magazines').gt(5)).run(conn, callback)
```

__Example:__ Select all documents where the 'abilities' embedded document has an
attribute called 'super-strength'.

```js
r.table('marvel').filter(function(hero) {
    return hero('abilities').hasFields('super-strength')
}).run(conn, callback)
```

__Example:__ Select all documents where the field 'powers' containing an array has an
element equal to 10.

```js
r.table('marvel').filter(
    r.row('poweres').filter(
    function(powers_el) { return powers_el.eq(10) }
    ).count().gt(0)
).run(conn, callback)
```

