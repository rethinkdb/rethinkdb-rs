---
layout: api-command
language: Python
permalink: api/python/has_fields/
command: has_fields
related_commands:
    '[] (get_field)': get_field/
    with_fields: with_fields/
---

# Command syntax #

{% apibody %}
sequence.has_fields([selector1, selector2...]) &rarr; stream
array.has_fields([selector1, selector2...]) &rarr; array
object.has_fields([selector1, selector2...]) &rarr; boolean
{% endapibody %}

# Description #

Test if an object has one or more fields. An object has a field if it has that key and the key has a non-null value. For instance, the object `{'a': 1,'b': 2,'c': null}` has the fields `a` and `b`.

When applied to a single object, `has_fields` returns `true` if the object has the fields and `false` if it does not. When applied to a sequence, it will return a new sequence (an array or stream) containing the elements that have the specified fields.

__Example:__ Return the players who have won games.

```py
r.table('players').has_fields('games_won').run(conn)
```

__Example:__ Return the players who have *not* won games. To do this, use `has_fields` with [not](/api/python/not), wrapped with [filter](/api/python/filter).

```py
r.table('players').filter(~r.row.has_fields('games_won')).run(conn)
```

__Example:__ Test if a specific player has won any games.

```py
r.table('players').get(
    'b5ec9714-837e-400c-aa74-dbd35c9a7c4c').has_fields('games_won').run(conn)
```

**Nested Fields**

`has_fields` lets you test for nested fields in objects. If the value of a field is itself a set of key/value pairs, you can test for the presence of specific keys.

__Example:__ In the `players` table, the `games_won` field contains one or more fields for kinds of games won:

```py
{
    'games_won': {
        'playoffs': 2,
        'championships': 1
    }
}
```

Return players who have the "championships" field.

```py
r.table('players').has_fields({'games_won': {'championships': true}}).run(conn)
```

Note that `true` in the example above is testing for the existence of `championships` as a field, not testing to see if the value of the `championships` field is set to `true`. There's a more convenient shorthand form available. (See [pluck](/api/python/pluck) for more details on this.)

```py
r.table('players').has_fields({'games_won': 'championships'}).run(conn)
```
