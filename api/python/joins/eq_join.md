---
layout: api-command
language: Python
permalink: api/python/eq_join/
command: eq_join
related_commands:
    inner_join: inner_join/
    outer_join: outer_join/
    without: without/
    zip: zip/
---

# Command syntax #

{% apibody %}
sequence.eq_join(left_field, right_table[, index='id', ordered=False]) &rarr; sequence
sequence.eq_join(predicate_function, right_table[, index='id', ordered=False]) &rarr; sequence
{% endapibody %}

# Description #

<img alt="Data Modeling Illustration" class="api_command_illustration" src="/assets/images/docs/api_illustrations/table-joins.png"/>

Join tables using a field or function on the left-hand sequence matching primary keys or secondary indexes on the right-hand table. `eq_join` is more efficient than other ReQL join types, and operates much faster. Documents in the result set consist of pairs of left-hand and right-hand documents, matched when the field on the left-hand side exists and is non-null and an entry with that field's value exists in the specified index on the right-hand side.

The result set of `eq_join` is a stream or array of objects. Each object in the returned set will be an object of the form `{ left: <left-document>, right: <right-document> }`, where the values of `left` and `right` will be the joined documents. Use the <code><a href="/api/python/zip/">zip</a></code> command to merge the `left` and `right` fields together.

The results from `eq_join` are, by default, not ordered. The optional `ordered=True` parameter will cause `eq_join` to order the output based on the left side input stream. (If there are multiple matches on the right side for a document on the left side, their order is not guaranteed even if `ordered` is `True`.) Requiring ordered results can significantly slow down `eq_join`, and in many circumstances this ordering will not be required. (See the first example, in which ordered results are obtained by using `order_by` after `eq_join`.)

Suppose the players table contains these documents:

```py
[
    { 'id': 1, 'player': 'George', 'game_id': 1 },
    { 'id': 2, 'player': 'Agatha', 'game_id': 3 },
    { 'id': 3, 'player': 'Fred', 'game_id': 2 },
    { 'id': 4, 'player': 'Marie', 'game_id': 2 },
    { 'id': 5, 'player': 'Earnest', 'game_id': 1 },
    { 'id': 6, 'player': 'Beth', 'game_id': 3 }
]
```

The games table contains these documents:

```py
[
    { 'id': 1, 'field': 'Little Delving' },
    { 'id': 2, 'field': 'Rushock Bog' },
    { 'id': 3, 'field': 'Bucklebury' }
]
```

__Example:__ Match players with the games they've played against one another.

Join these tables using `game_id` on the player table and `id` on the games table:

```py
r.table('players').eq_join('game_id', r.table('games')).run(conn)
```

This will return a result set such as the following:

```py
[
    {
        "left" : { "game_id" : 3, "id" : 2, "player" : "Agatha" },
        "right" : { "id" : 3, "field" : "Bucklebury" }
    },
    {
        "left" : { "game_id" : 2, "id" : 3, "player" : "Fred" },
        "right" : { "id" : 2, "field" : "Rushock Bog" }
    },
    ...
]
```

<!-- stop -->

What you likely want is the result of using `zip` with that. For clarity, we'll use `without` to drop the `id` field from the games table (it conflicts with the `id` field for the players and it's redundant anyway), and we'll order it by the games.

```py
r.table('players').eq_join('game_id', r.table('games')).without({'right': "id"}).zip().order_by('game_id').run(conn)

[
    { "field": "Little Delving", "game_id": 1, "id": 5, "player": "Earnest" },
    { "field": "Little Delving", "game_id": 1, "id": 1, "player": "George" },
    { "field": "Rushock Bog", "game_id": 2, "id": 3, "player": "Fred" },
    { "field": "Rushock Bog", "game_id": 2, "id": 4, "player": "Marie" },
    { "field": "Bucklebury", "game_id": 3, "id": 6, "player": "Beth" },
    { "field": "Bucklebury", "game_id": 3, "id": 2, "player": "Agatha" }
]
```

For more information, see [Table joins in RethinkDB](/docs/table-joins/).

__Example:__ Use a secondary index on the right table rather than the primary key. If players have a secondary index on their cities, we can get a list of arenas with players in the same area.

```py
r.table('players').eq_join('city_id', r.table('arenas'), index='city_id').run(conn)
```

__Example:__ Use a nested key as the join field. Suppose the documents in the players table were structured like this:

```py
{ 'id': 1, 'player': 'George', 'game': {'id': 1} },
{ 'id': 2, 'player': 'Agatha', 'game': {'id': 3} },
...
```

Simply specify the field using the `row` command instead of a string.

```py
r.table('players').eq_join(r.row['game']['id'], r.table('games')).without({'right': 'id'}).zip().run(conn)

[
    { "field": "Little Delving", "game": { "id": 1 }, "id": 5, "player": "Earnest" },
    { "field": "Little Delving", "game": { "id": 1 }, "id": 1, "player": "George" },
    ...
]
```

__Example:__ Use a function instead of a field to join on a more complicated expression. Suppose the players have lists of favorite games ranked in order in a field such as `"favorites": [3, 2, 1]`. Get a list of players and their top favorite:

```py
r.table('players').eq_join(
    lambda player: player['favorites'].nth(0),
    r.table('games')
).without([{'left': ['favorites', 'game_id', 'id']}, {'right': 'id'}]).zip()
```

Result:

```py
[
	{ "field": "Rushock Bog", "name": "Fred" },
	{ "field": "Little Delving", "name": "George" },
	...
]
```
