---
layout: documentation
title: ReQL data types
active: docs
docs_active: data-types
permalink: docs/data-types/
---

RethinkDB stores five basic kinds of values: *numbers, strings, times, boolean* values, and the *null* value. In addition, it stores several composite data types. *Objects* and *arrays* are key/value pairs and lists, respectively, with direct counterparts in most programming languages. *Streams, selections* and *tables* are RethinkDB-specific data types.

# Basic Data Types #

* **Numbers** are any real number: `5`, `3.14159`, `-42`. RethinkDB uses double precision (64-bit) floating point numbers internally. (Neither infinity nor [NaN](http://en.wikipedia.org/wiki/NaN) are allowed.)

* **Strings** are Unicode strings: `"superhero"`, `"the quick brown fox"`, <code>&quot;&uuml;nn&euml;c&euml;ss&auml;r&yuml; &uuml;ml&auml;&uuml;ts&quot;</code>.

* **Times** are RethinkDB's native date/time type, stored with millisecond precision. You can use native date/time types in supported languages, as the conversion will be done by the driver. See [Dates and times in RethinkDB](/docs/dates-and-times/) for details.

* **Booleans** are `true` and `false`.

* **Null** is a value distinct from the number zero, an empty set, or a zero-length string. Natively this might be `null`, `nil` or `None`, depending on the language. it is often used to explicitly denote the absence of any other value. The root node of a tree structure might have a parent of `null`, or a required but as yet non-initialized key might be given a value of `null`.

* **Objects** are JSON data objects, standard key-value pairs.

	```
	{ username: 'bob', posts: 23, favorites: {color: 'blue', food: 'tacos'},
	friends: ['agatha', 'jason'] }
	```
	
	Any valid JSON object is a valid RethinkDB object, so values can be any of the basic values, arrays, or other objects. Documents in a RethinkDB database are objects. Like JSON, key names must be strings, not integers.

* **Arrays** are lists of zero or more elements.

	```
	[1, 2, 3]
	[]
	[{user: 'Bob', posts: 23}, {user: 'Jason', posts: 10}]
	```

	Again, anything valid in a JSON array is valid in RethinkDB: the elements may be any of the basic values, objects, or other arrays. Arrays in RethinkDB are loaded fully into memory before they're returned to the user, so they're inefficient at large sizes. RethinkDB supports arrays of up to 100,000 elements. (This may be a configurable option in a future release; see [Github Issue #2318](https://github.com/rethinkdb/rethinkdb/issues/2318) for details.)

# Composite Data Types #

* **Streams** are lists like arrays, but they're loaded in a lazy fashion. Operations that return streams return a *cursor.* A cursor is a pointer into the result set. Instead of reading the results all at once like an array, you loop over the results, retrieving the next member of the set with each iteration. This makes it possible to efficiently work with large result sets. (See "Working with Streams," below, for some tips.) Streams are read-only; you can't pass one as an input to an ReQL command meant to modify its input like `update` or `delete`.

* **Selections** represent subsets of tables, for example, the return values of `filter` or `get`. There are two kinds of selections, **Selection&lt;Object&gt;** and  **Selection&lt;Stream&gt;**, which behave like objects or streams respectively. The difference between selections and objects/streams are that selections are writable--their return values can be passed as inputs to ReQL commands that modify the database. For instance, the `get` command will return a Selection&lt;Object&gt; that could then be passed to an `update` or `delete` command.

* **Tables** are RethinkDB database tables. They behave like selections&mdash;they're writable, as you can insert and delete documents in them. ReQL methods that use an [index](/docs/secondary-indexes), like `getAll`, are only available on tables.

In the ReQL API documentation you'll often come across the term **Sequence.** Sequences aren't their own data type&mdash;instead, that's a collective word for all the list data types: arrays, streams, selections, and tables. You may also see **Any** used for commands that work with any data type.

You can use the `typeOf` command at the end of a ReQL command to find out what you get as a result set. For instance (in Javascript):

```js
r.table('users').get(1).typeOf().run(conn, callback)
```

Returns `"SELECTION<OBJECT>"`.

# Working with Streams #

Streams use "lazy loading," a concept you may have run across in other database interfaces. Instead of returning an entire result set from a query, streams return an [iterator](http://en.wikipedia.org/wiki/Iterator) referred to as a "cursor," a pointer into the data set. 

Different languages support iterators in different ways, but the fundamental concept is always the same: the result set is traversed in a loop that returns one result set at a time. In Python, you might loop through a stream this way:

```py
players = r.table('players').run(conn)
for player in players:
	print player
```

In Ruby, you would use a block:

```rb
players = r.table('players').run(conn)
players.each do |player|
	puts player
end
```

Javascript has no native iterator, but ReQL implements an [each](/api/javascript/each) command similar to [jQuery](http://api.jquery.com/each/)'s.

```js
r.table('players').run(conn, function(err, cursor) {
	cursor.each(function(err, player) {
		if (err) throw err;
		console.log(player);
	});
});
```

Smaller result sets can be turned into an array directly. In the examples above, you would use `list(players)` in Python, `players.to_a` in Ruby,  or `players.toArray()` in Javascript (a ReQL command; see [toArray](/api/javascript/to_array/)).

# Grouped Data #

The `group` command partitions a stream into multiple groups based on specified fields or functions. It returns a pseudotype named `GROUPED_DATA`. ReQL comments called on `GROUPED_DATA` operate on each group individually. For a thorough discussion about groups with examples, read the [group](/api/javascript/group) documentation.