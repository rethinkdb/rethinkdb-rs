---
layout: api-command
language: JavaScript
permalink: api/javascript/coerce_to/
command: coerceTo
io:
    -   - sequence
        - array
    -   - value
        - string
    -   - array
        - object
    -   - object
        - array
related_commands:
    object: object/
---

# Command syntax #

{% apibody %}
sequence.coerceTo('array') &rarr; array
value.coerceTo('string') &rarr; string
string.coerceTo('number') &rarr; number
array.coerceTo('object') &rarr; object
object.coerceTo('array') &rarr; array
{% endapibody %}

# Description #

Convert value of one type into another.

* a sequence, selection or object can be coerced to an array
* an array of key-value pairs can be coerced to an object
* a string can be coerced to a number
* any datum (single value) can be coerced to to a string

__Example:__ Coerce a stream to an array to store its output in a field. (A stream cannot be stored in a field directly.)

```js
r.table('posts').map(function (post) {
    post.merge({ comments: r.table('comments').getAll(post('id'), {index: 'postId'}).coerceTo('array')});
}).run(conn, callback)
```

__Example:__ Coerce an array of key-value pairs into an object.


```js
r.expr([['name', 'Ironman'], ['victories', 2000]]).coerceTo('object').run(conn, callback)
```

__Note:__ To coerce a list of key-value pairs like `['name', 'Ironman', 'victories', 2000]` to an object, use the [object](/api/javascript/object) command.

__Example:__ Coerce a number to a string.

```js
r.expr(1).coerceTo('string').run(conn, callback)
```
