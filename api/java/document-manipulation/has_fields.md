---
layout: api-command
language: JavaScript
permalink: api/javascript/has_fields/
command: hasFields
related_commands:
    '() (getField)': get_field/
    withFields: with_fields/
---

# Command syntax #

{% apibody %}
sequence.hasFields([selector1, selector2...]) &rarr; stream
array.hasFields([selector1, selector2...]) &rarr; array
object.hasFields([selector1, selector2...]) &rarr; boolean
{% endapibody %}

# Description #

Test if an object has one or more fields. An object has a field if it has that key and the key has a non-null value. For instance, the object `{'a': 1,'b': 2,'c': null}` has the fields `a` and `b`.

When applied to a single object, `hasFields` returns `true` if the object has the fields and `false` if it does not. When applied to a sequence, it will return a new sequence (an array or stream) containing the elements that have the specified fields.

__Example:__ Return the players who have won games.

```js
r.table('players').hasFields('games_won').run(conn)
```

__Example:__ Return the players who have *not* won games. To do this, use `hasFields` with [not](/api/javascript/not), wrapped with [filter](/api/javascript/filter).

```js
r.table('players').filter(
    r.row.hasFields('games_won').not()
).run(conn)
```

__Example:__ Test if a specific player has won any games.

```js
r.table('players').get('b5ec9714-837e-400c-aa74-dbd35c9a7c4c'
    ).hasFields('games_won').run(conn)
```

**Nested Fields**

`hasFields` lets you test for nested fields in objects. If the value of a field is itself a set of key/value pairs, you can test for the presence of specific keys.

__Example:__ In the `players` table, the `games_won` field contains one or more fields for kinds of games won:

```js
{
    games_won: {
        playoffs: 2,
        championships: 1
    }
}
```

Return players who have the "championships" field.

```js
r.table('players').hasFields({'games_won': {'championships': true}}).run(conn)
```

Note that `true` in the example above is testing for the existence of `championships` as a field, not testing to see if the value of the `championships` field is set to `true`. There's a more convenient shorthand form available. (See [pluck](/api/javascript/pluck) for more details on this.)

```js
r.table('players').hasFields({'games_won': 'championships'}
    ).run(conn)
```
