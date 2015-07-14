---
layout: api-command
language: JavaScript
permalink: api/javascript/map/
command: map
io:
    -   - sequence
        - stream
    -   - array
        - array
related_commands:
    concatMap: concat_map/
    reduce: reduce/
    do: do/
---

# Command syntax #

{% apibody %}
sequence1.map([sequence2, ...], mappingFunction) &rarr; stream
array1.map([array2, ...], mappingFunction) &rarr; array
r.map(sequence1[, sequence2, ...], mappingFunction) &rarr; stream
r.map(array1[, array2, ...], mappingFunction) &rarr; array
{% endapibody %}

# Description #

Transform each element of one or more sequences by applying a mapping function to them. If `map` is run with two or more sequences, it will iterate for as many items as there are in the shortest sequence.

Note that `map` can only be applied to sequences, not single values. If you wish to apply a function to a single value/selection (including an array), use the [do](/api/javascript/do) command.

__Example:__ Return the first five squares.

```js
r.expr([1, 2, 3, 4, 5]).map(function (val) {
    return val.mul(val);
}).run(conn, callback);
// Result passed to callback
[1, 4, 9, 16, 25]
```

__Example:__ Sum the elements of three sequences.

```js
var sequence1 = [100, 200, 300, 400];
var sequence2 = [10, 20, 30, 40];
var sequence3 = [1, 2, 3, 4];
r.map(sequence1, sequence2, sequence3, function (val1, val2, val3) {
    return val1.add(val2).add(val3);
}).run(conn, callback);
// Result passed to callback
[111, 222, 333, 444]
```

__Example:__ Rename a field when retrieving documents using `map` and [merge](/api/javascript/merge/).

This example renames the field `id` to `userId` when retrieving documents from the table `users`.

```js
r.table('users').map(function (doc) {
    return doc.merge({userId: doc('id')}).without('id');
}).run(conn, callback);
```

Note that in this case, [row](/api/javascript/row) may be used as an alternative to writing an anonymous function, as it returns the same value as the function parameter receives:

```js
r.table('users').map(
    r.row.merge({userId: r.row('id')}).without('id');
}).run(conn, callback);
```


__Example:__ Assign every superhero an archenemy.

```js
r.table('heroes').map(r.table('villains'), function (hero, villain) {
    return hero.merge({villain: villain});
}).run(conn, callback);
```
