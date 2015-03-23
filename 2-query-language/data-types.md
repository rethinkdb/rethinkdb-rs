---
layout: documentation
title: ReQL data types
docs_active: data-types
permalink: docs/data-types/
---

RethinkDB stores six basic single-value data types: *numbers, strings, binary objects, times, boolean* values, and the *null* value. In addition, it stores two basic composite data types. *Objects* and *arrays* are key/value pairs and lists, respectively, with direct counterparts in most programming languages. *Streams, selections* and *tables* are RethinkDB-specific composite data types. Lastly, there are geometry data types: *point, line* and *polygon.*

# Basic data types #

* **Numbers** are any real number: `5`, `3.14159`, `-42`. RethinkDB uses double precision (64-bit) floating point numbers internally. (Neither infinity nor [NaN](http://en.wikipedia.org/wiki/NaN) are allowed.)

* **Strings** are any UTF-8 string that does not contain the `null` code point (U+0000): `"superhero"`, <code>&quot;&uuml;nn&euml;c&euml;ss&auml;r&yuml; &uuml;ml&auml;&uuml;ts&quot;</code>. Currently RethinkDB does not enforce UTF-8 encoding, but most string operations assume they are operating on UTF-8 strings. Future versions of RethinkDB will enforce UTF-8 encoding and allow `null` to be included.

* **Binary objects** are similar to BLOBs in SQL databases: files, images and other binary data. See [Storing binary objects](/docs/storing-binary/) for details.

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

	Again, anything valid in a JSON array is valid in RethinkDB: the elements may be any of the basic values, objects, or other arrays. Arrays in RethinkDB are loaded fully into memory before they're returned to the user, so they're inefficient at large sizes. RethinkDB defaults to supporting arrays of up to 100,000 elements; this may be set to a different value at runtime for reading by using the `array_limit` option to [run](/api/javascript/run).

# RethinkDB-specific data types #

* **Streams** are lists like arrays, but they're loaded in a lazy fashion. Operations that return streams return a *cursor.* A cursor is a pointer into the result set. Instead of reading the results all at once like an array, you loop over the results, retrieving the next member of the set with each iteration. This makes it possible to efficiently work with large result sets. (See "Working with Streams," below, for some tips.) Streams are read-only; you can't pass one as an input to an ReQL command meant to modify its input like `update` or `delete`.

* **Selections** represent subsets of tables, for example, the return values of `filter` or `get`. There are two kinds of selections, **Selection&lt;Object&gt;** and  **Selection&lt;Stream&gt;**, which behave like objects or streams respectively. The difference between selections and objects/streams are that selections are writable&mdash;their return values can be passed as inputs to ReQL commands that modify the database. For instance, the `get` command will return a Selection&lt;Object&gt; that could then be passed to an `update` or `delete` command.

* **Tables** are RethinkDB database tables. They behave like selections&mdash;they're writable, as you can insert and delete documents in them. ReQL methods that use an [index](/docs/secondary-indexes), like `getAll`, are only available on tables.

In the ReQL API documentation you'll often come across the term **Sequence.** Sequences aren't their own data type&mdash;instead, that's a collective word for all the list data types: arrays, streams, selections, and tables. You may also see **Any** used for commands that work with any data type.

You can use the `typeOf` command at the end of a ReQL command to find out what you get as a result set. For instance (in JavaScript):

```js
r.table('users').get(1).typeOf().run(conn, callback)
```

Returns `"SELECTION<OBJECT>"`.

# Geometry data types #

For more information on these data types, read about RethinkDB's [geospatial support][geo].

[geo]: /docs/geo-support/

* **Points** are denoted by a single coordinate pair, two floating point numbers indicating longitude (&minus;180 through 180) and latitude (&minus;90 through 90).

* **Lines** are a sequence of two or more points, denoted by either point objects or coordinate pairs.

* **Polygons** are multipoint lines denoted by a sequence of three or more points, denoted by either point objects or coordinate pairs, which do not intersect with themselves. The first and last coordinate pairs of a polygon are equal.

    Polygons can also have holes in them, created when a polygon entirely enclosed by another one is "punched out" of the larger one using the `polygonSub` command. (This is the only way to create a polygon with a hole in it.)

In the ReQL API documentation you'll also see a "pseudotype" called **Geometry,** which is a collective for all of the geometry data types. Those commands will work with points, lines and polygons.

# Working with streams #

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

JavaScript has no native iterator, but ReQL implements an [each](/api/javascript/each) command similar to [jQuery](http://api.jquery.com/each/)'s.

```js
r.table('players').run(conn, function(err, cursor) {
	cursor.each(function(err, player) {
		if (err) throw err;
		console.log(player);
	});
});
```

Smaller result sets can be turned into an array directly. In the examples above, you would use `list(players)` in Python, `players.to_a` in Ruby,  or `players.toArray()` in JavaScript (a ReQL command; see [toArray](/api/javascript/to_array/)).

# Grouped data #

The `group` command partitions a stream into multiple groups based on specified fields or functions. It returns a pseudotype named `GROUPED_DATA`. ReQL comments called on `GROUPED_DATA` operate on each group individually. For a thorough discussion about groups with examples, read the [group](/api/javascript/group) documentation.