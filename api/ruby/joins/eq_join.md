---
layout: api-command
language: Ruby
permalink: api/ruby/eq_join/
command: eq_join
related_commands:
    inner_join: inner_join/
    outer_join: outer_join/
    without: without/
    zip: zip/
---


{% apibody %}
sequence.eq_join(left_field, right_table[, :index => 'id']) &rarr; sequence
{% endapibody %}

# Description #

Join tables using a field on the left-hand sequence matching primary keys or secondary indexes on the right-hand table. `eq_join` is more efficient than other Re_qL join types, and operates much faster. Documents in the result set consist of pairs of left-hand and right-hand documents, matched when the field on the left-hand side exists and is non-null and an entry with that field's value exists in the specified index on the right-hand side.

The result set of `eq_join` is a stream or array of objects. Each object in the returned set will be an object of the form `{ left: <left-document>, right: <right-document> }`, where the values of `left` and `right` will be the joined documents. Use the <code><a href="/api/javascript/zip/">zip</a></code> command to merge the `left` and `right` fields together.

**Example:** Match players with the games they've played against one another.

The players table contains these documents:

```rb
[
    { :id => 1, :player => 'George', :game_id => 1 },
    { :id => 2, :player => 'Agatha', :game_id => 3 },
    { :id => 3, :player => 'Fred', :game_id => 2 },
    { :id => 4, :player => 'Marie', :game_id => 2 },
    { :id => 5, :player => 'Earnest', :game_id => 1 },
    { :id => 6, :player => 'Beth', :game_id => 3 }
]
```

The games table contains these documents:

```rb
[
    { :id => 1, :field => 'Little Delving' },
    { :id => 2, :field => 'Rushock Bog' },
    { :id => 3, :field => 'Bucklebury' }
]
```

Join these tables using `game_id` on the player table and `id` on the games table:

```rb
r.table('players').eq_join('game_id', r.table('games')).run(conn)
```

This will return a result set such as the following:

```rb
[
    {
        'left' => { 'game_id' => 3, 'id' => 2, 'player' => "Agatha" },
        'right' => { 'id' => 3, 'field' => "Bucklebury" }
    },
    {
        'left' => { 'game_id' => 2, 'id' => 3, 'player' => "Fred" },
        'right' => { 'id' => 2, 'field' => "Rushock Bog" }
    },
    ...
]
```

What you likely want is the result of using `zip` with that. For clarity, we'll use `without` to drop the `id` field from the games table (it conflicts with the `id` field for the players and it's redundant anyway), and we'll order it by the games.

```rb
r.table('players').eq_join('game_id', r.table('games')).without({:right => "id"}).zip().order_by('game_id').run(conn)

[
    { 'field' => "Little Delving", 'game_id' => 1, 'id' => 5, 'player' => "Earnest" },
    { 'field' => "Little Delving", 'game_id' => 1, 'id' => 1, 'player' => "George" },
    { 'field' => "Rushock Bog", 'game_id' => 2, 'id' => 3, 'player' => "Fred" },
    { 'field' => "Rushock Bog", 'game_id' => 2, 'id' => 4, 'player' => "Marie" },
    { 'field' => "Bucklebury", 'game_id' => 3, 'id' => 6, 'player' => "Beth" },
    { 'field' => "Bucklebury", 'game_id' => 3, 'id' => 2, 'player' => "Agatha" }
]
```

For more information, see [Table joins in Rethink_dB](/docs/table-joins/).

**Example:** Use a secondary index on the right table rather than the primary key. If players have a secondary index on their cities, we can get a list of arenas with players in the same area.

```rb
r.table('arenas').eq_join('city_id', r.table('arenas'), {:index => 'city_id'}).run(conn)
```

**Example:** Use a nested key as the join field. Suppose the documents in the players table were structured like this:

```rb
{ :id => 1, :player => 'George', :game => {:id => 1} },
{ :id => 2, :player => 'Agatha', :game => {:id => 3} },
...
```

Simply specify the field using a block instead of a string.

```rb
r.table('players').eq_join(
    lambda { |player| player['game']['id'] },
    r.table('games')).without({:right => 'id'}
).zip().run(conn)

[
    { 'field' => "Little Delving", 'game' => { 'id' => 1 }, 'id' => 5, 'player' => "Earnest" },
    { 'field' => "Little Delving", 'game' => { 'id' => 1 }, 'id' => 1, 'player' => "George" },
    ...
]
```

**Example:** Use a function instead of a field to join on a more complicated expression. Suppose the players have lists of favorite games ranked in order in a field such as `favorites: [3, 2, 1]`. Get a list of players and their top favorite:

```rb
r.table('players3').eq_join(
    lambda { |player| player['favorites'].nth(0) },
    r.table('games')
).without([{:left => ['favorites', 'game_id', 'id']}, {:right => 'id'}]).zip()
```

Result:

```rb
[
	{ 'field' => "Rushock Bog", 'name' => "Fred" },
	{ 'field' => "Little Delving", 'name' => "George" },
	...
]
```
