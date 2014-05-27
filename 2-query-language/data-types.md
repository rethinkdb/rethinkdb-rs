---
layout: documentation
title: ReQL data types
active: docs
docs_active: data-types
permalink: docs/data-types/
---

RethinkDB stores five basic kinds of values: *numbers, strings, times, boolean* values, and the *null* value. In addition, it stores several composite data types: *objects,* *streams* and *arrays,* which are immutable, and *selections* and *tables,* which are mutable.

# Basic Data Types #

* **Numbers** are any real number: `5`, `3.14159`, `-42`. RethinkDB stores all number types as double precision (64-bit) floating point.

* **Strings** are Unicode strings: `"superhero"`, `"the quick brown fox"`, `"ünnëcëssärÿ ümläüts"`.

* **Times** are RethinkDB's native date/time type, stored with millisecond precision. You can use native date/time types in supported languages, as the conversion will be done by the driver. See [Dates and times in RethinkDB](/docs/dates-and-times/) for details.

* **Booleans** are `true` and `false`.

* **Null** is`null`. (Or `nil` or `None`, as the case may be.)

* **Objects** are JSON data objects, standard key-value pairs.

		{ username: 'bob', posts: 23, favorites: {color: 'blue', food: 'tacos'},
		  friends: ['agatha', 'jason'] }
	
	Any valid JSON object is a valid RethinkDB object, so values can be any of the basic values, arrays, or other objects. Documents in a RethinkDB database are objects. Like JSON, key names must be strings, not integers.

* **Arrays** are lists of one or more elements.

		[1, 2, 3]
		[{user: 'Bob', posts: 23}, {user: 'Jason', posts: 10}]

	Again, anything valid in a JSON list is valid in RethinkDB: the elements in an array may be any of the basic values, objects, or other arrays. Arrays in RethinkDB are loaded fully into memory before they're returned to the user, so they're inefficient at large sizes. RethinkDB supports arrays of up to 100,000 elements. (This may be a configurable option in a future release; see [Github Issue #2318](https://github.com/rethinkdb/rethinkdb/issues/2318) for details.)

# Composite Data Types #

* **Streams** are lists like arrays, but they're loaded in a lazy fashion. Operations that return streams return a *cursor.* A cursor is a pointer into the result set. Instead of reading the results all at once like an array, you loop over the results, retrieving the next member of the set with each iteration. This makes it possible to efficiently work with large result sets. (See "Working with Streams," below, for some tips.)

	Arrays, objects and streams are immutable—they can't be changed. You can't chain a ReQL command that returns a stream to another command that modifies the database, like `update` or `delete`.

* **Selections** *are* mutable. There are two kinds of selections: **Selection&lt;Object&gt;** and  **Selection&lt;Stream&gt;**. They behave as you'd expect from their names—like objects or streams—but they can be passed as inputs to ReQL commands that *do* modify the database. The `get` command, for instance, returns a Selection&lt;Object&gt;.

* **Tables** are (surprise) RethinkDB database tables. They behave like selections—obviously, a table has to be mutable. Some ReQL methods, like `getAll`, are only available on tables.

In the ReQL API documentation you'll often come across the term **Sequence.** Sequences aren't their own data type—instead, that's a collective word we use for all the list data types: arrays, streams, and selections. You may also see **Any** used for commands that work with any immutable data type. (So "any" isn't strictly true: you can't use selections or tables with these commands.)

You can use the `typeOf` command at the end of a ReQL command to find out what you get as a result set. For instance:

```js
r.table('users').get(1).typeOf().run(conn, callback)
```

Returns `"SELECTION<OBJECT>"`.

# Working with Streams #

Streams use "lazy loading," a concept you may have run across in other database interfaces. Instead of returning an entire result set from a query, streams return an [iterator](http://en.wikipedia.org/wiki/Iterator) and a "cursor," a pointer into the data set. 

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
