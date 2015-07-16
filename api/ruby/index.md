---
layout: api
title: "ReQL command reference"
active: api
no_footer: true
permalink: api/ruby/
language: Ruby
---

{% apisection Accessing ReQL %}
All ReQL queries begin from the top-level module.

## [r](r/) ##

{% apibody %}
r &rarr; r
{% endapibody %}

The top-level ReQL namespace.

__Example:__ Set up your top-level namespace.

```rb
require 'rethinkdb'
include RethinkDB::Shortcuts
```

## [connect](connect/) ##

{% apibody %}
r.connect(opts={}) &rarr; connection
{% endapibody %}

Create a new connection to the database server.  Accepts the following options:

- `host`: the host to connect to (default `localhost`).
- `port`: the port to connect on (default `28015`).
- `db`: the default database (default `test`).
- `auth_key`: the authentication key (default none).

If the connection cannot be established, a `RqlDriverError` exception will be thrown.

__Example:__ Opens a new connection to the database.

```rb
conn = r.connect(:host => 'localhost',
                 :port => 28015,
                 :db => 'heroes',
                 :auth_key => 'hunter2')
```

[Read more about this command &rarr;](connect/)

## [repl](repl/) ##

{% apibody %}
conn.repl
{% endapibody %}

Set the default connection to make REPL use easier. Allows calling
`.run` on queries without specifying a connection.

__Note:__ Avoid using `repl` in application code. RethinkDB connection objects are not thread-safe, and calls to `connect` from multiple threads may change the global connection object used by `repl`. Applications should specify connections explicitly.

__Example:__ Set the default connection for the REPL, then call
`run` without specifying the connection.

```rb
r.connect(:db => 'marvel').repl
r.table('heroes').run
```

## [close](close/) ##

{% apibody %}
conn.close([{:noreply_wait => true}])
{% endapibody %}

Close an open connection.

__Example:__ Close an open connection, waiting for noreply writes to finish.

```rb
conn.close
```

[Read more about this command &rarr;](close/)

## [reconnect](reconnect/) ##

conn.reconnect([{:noreply_wait => true}])

Close and reopen a connection.

__Example:__ Cancel outstanding requests/queries that are no longer needed.

```rb
conn.reconnect(:noreply_wait => false)
```

[Read more about this command &rarr;](reconnect/)

## [use](use/) ##

{% apibody %}
conn.use(db_name)
{% endapibody %}

Change the default database on this connection.

__Example:__ Change the default database so that we don't need to
specify the database when referencing a table.

```rb
conn.use('marvel')
r.table('heroes').run(conn) # refers to r.db('marvel').table('heroes')
```

## [run](run/) ##

{% apibody %}
query.run(conn[, options]) &rarr; cursor
query.run(conn[, options]) &rarr; object
{% endapibody %}

Run a query on a connection, returning either a single JSON result or
a cursor, depending on the query.

__Example:__ Run a query on the connection `conn` and print out every
row in the result.

```rb
r.table('marvel').run(conn).each{|x| p x}
```

[Read more about this command &rarr;](run/)

## [em_run](em_run/) ##

{% apibody %}
query.em_run(conn, block) &rarr; cursor
query.em_run(conn, block) &rarr; object
{% endapibody %}

Run a query asynchronously on a connection using [EventMachine](http://rubyeventmachine.com). If the query returns a sequence (including a stream), the block will be called once with each element of the sequence. Otherwise, the block will be called just once with the returned value.

__Example:__ return a list of users in an EventMachine loop.

```rb
EventMachine.run {
  r.table('users').order_by(:index => 'username').em_run(conn) { |row|
    # do something with returned row data
    p row
  }
}
```

[Read more about this command &rarr;](em_run/)

## [noreply_wait](noreply_wait/) ##

{% apibody %}
conn.noreply_wait
{% endapibody %}

`noreply_wait` ensures that previous queries with the `noreply` flag have been processed
by the server. Note that this guarantee only applies to queries run on the given connection.

__Example:__ We have previously run queries with the `noreply` argument set to `true`. Now
wait until the server has processed them.

```rb
conn.noreply_wait
```

{% endapisection %}

{% apisection Cursors %}

## [next](next/) ##

{% apibody %}
cursor.next([true])
{% endapibody %}

Get the next element in the cursor.

__Example:__ Retrieve the next element.

```rb
cursor = r.table('superheroes').run(conn)
doc = cursor.next()
```

[Read more about this command &rarr;](next/)

## [each](each/) ##

{% apibody %}
cursor.each { ... }
array.each { ... }
feed.each { ... }
{% endapibody %}

Lazily iterate over a result set one element at a time.

__Example:__ Let's process all the elements!

```rb
cursor = r.table('users').run(conn)
cursor.each { |doc|
    process_row(doc)
}
```

[Read more about this command &rarr;](each/)

## [to_a](to_array/) ##

{% apibody %}
cursor.to_a()
{% endapibody %}

Retrieve all results as an array.

__Example:__ For small result sets it may be more convenient to process them at once as an array.

```rb
cursor = r.table('users').run()
users = cursor.to_a()
process_results(users)
```

[Read more about this command &rarr;](to_array/)

## [close (cursor)](close-cursor/) ##

{% apibody %}
cursor.close
{% endapibody %}


Close a cursor. Closing a cursor cancels the corresponding query and frees the memory
associated with the open request.

__Example:__ Close a cursor.

```rb
cursor.close
```


{% endapisection %}

{% apisection Manipulating databases %}
## [db_create](db_create/) ##

{% apibody %}
r.db_create(db_name) &rarr; object
{% endapibody %}

Create a database. A RethinkDB database is a collection of tables, similar to
relational databases.

If successful, the operation returns an object: `{"created": 1}`. If a database with the
same name already exists the operation throws `RqlRuntimeError`.

Note: that you can only use alphanumeric characters and underscores for the database name.

__Example:__ Create a database named 'superheroes'.

```rb
r.db_create('superheroes').run(conn)
```


## [db_drop](db_drop/) ##

{% apibody %}
r.db_drop(db_name) &rarr; object
{% endapibody %}

Drop a database. The database, all its tables, and corresponding data will be deleted.

If successful, the operation returns the object `{"dropped": 1}`. If the specified database
doesn't exist a `RqlRuntimeError` is thrown.

__Example:__ Drop a database named 'superheroes'.

```rb
r.db_drop('superheroes').run(conn)
```


## [db_list](db_list/) ##

{% apibody %}
r.db_list() &rarr; array
{% endapibody %}

List all database names in the system. The result is a list of strings.

__Example:__ List all databases.

```rb
r.db_list.run(conn)
```

{% endapisection %}




{% apisection Manipulating tables %}
## [table_create](table_create/) ##

{% apibody %}
db.table_create(table_name[, options]) &rarr; object
{% endapibody %}

Create a table. A RethinkDB table is a collection of JSON documents.

__Example:__ Create a table named 'dc_universe' with the default settings.

```rb
r.db('test').table_create('dc_universe').run(conn)
```

[Read more about this command &rarr;](table_create/)

## [table_drop](table_drop/) ##

{% apibody %}
db.table_drop(table_name) &rarr; object
{% endapibody %}

Drop a table. The table and all its data will be deleted.

__Example:__ Drop a table named 'dc_universe'.

```rb
r.db('test').table_drop('dc_universe').run(conn)
```

[Read more about this command &rarr;](table_drop/)

## [table_list](table_list/) ##

{% apibody %}
db.table_list() &rarr; array
{% endapibody %}

List all table names in a database. The result is a list of strings.

__Example:__ List all tables of the 'test' database.

```rb
r.db('test').table_list().run(conn)
```


## [index_create](index_create/) ##

{% apibody %}
table.index_create(index_name[, index_function][, :multi => false, :geo => false]) &rarr; object
{% endapibody %}

Create a new secondary index on a table.

__Example:__ Create a simple index based on the field `post_id`.

```rb
r.table('comments').index_create('post_id').run(conn)
```

[Read more about this command &rarr;](index_create/)


## [index_drop](index_drop/) ##

{% apibody %}
table.index_drop(index_name) &rarr; object
{% endapibody %}

Delete a previously created secondary index of this table.

__Example:__ Drop a secondary index named 'code_name'.

```rb
r.table('dc').index_drop('code_name').run(conn)
```


## [index_list](index_list/) ##

{% apibody %}
table.index_list() &rarr; array
{% endapibody %}

List all the secondary indexes of this table.

__Example:__ List the available secondary indexes for this table.

```rb
r.table('marvel').index_list().run(conn)
```

## [index_rename](index_rename/) ##

{% apibody %}
table.index_rename(old_index_name, new_index_name[, {:overwrite => false}]) &rarr; object
{% endapibody %}

Rename an existing secondary index on a table. If the optional argument `overwrite` is specified as `true`, a previously existing index with the new name will be deleted and the index will be renamed. If `overwrite` is `false` (the default) an error will be raised if the new index name already exists.

__Example:__ Rename an index on the comments table.

```rb
r.table('comments').index_rename('post_id', 'message_id').run(conn)
```

## [index_status](index_status/) ##

{% apibody %}
table.index_status([, index...]) &rarr; array
{% endapibody %}

Get the status of the specified indexes on this table, or the status
of all indexes on this table if no indexes are specified.

__Example:__ Get the status of all the indexes on `test`:

```rb
r.table('test').index_status.run(conn)
```

__Example:__ Get the status of the `timestamp` index:

```rb
r.table('test').index_status('timestamp').run(conn)
```

## [index_wait](index_wait/) ##

{% apibody %}
table.index_wait([, index...]) &rarr; array
{% endapibody %}

Wait for the specified indexes on this table to be ready, or for all
indexes on this table to be ready if no indexes are specified.

__Example:__ Wait for all indexes on the table `test` to be ready:

```rb
r.table('test').index_wait.run(conn)
```

__Example:__ Wait for the index `timestamp` to be ready:

```rb
r.table('test').index_wait('timestamp').run(conn)
```

## [changes](changes/) ##

{% apibody %}
stream.changes({:squash => false, :include_states => false}) &rarr; stream
singleSelection.changes({:squash => false, :include_states => false}) &rarr; stream
{% endapibody %}

Return a changefeed, an infinite stream of objects representing changes to a query. A changefeed may return changes to a table or an individual document (a "point" changefeed), and document transformation commands such as `filter` or `map` may be used before the `changes` command to affect the output.

__Example:__ Subscribe to the changes on a table.

```rb
r.table('games').changes().run(conn).each{|change| p(change)}
```
[Read more about this command &rarr;](changes/)

{% endapisection %}


{% apisection Writing data %}

## [insert](insert/) ##

{% apibody %}
table.insert(object | [object1, object2, ...][, :durability => "hard", :return_changes => false, :conflict => "error"])
    &rarr; object
{% endapibody %}

Insert documents into a table. Accepts a single document or an array of
documents.


__Example:__ Insert a document into the table `posts`.

```rb
r.table("posts").insert({
    :id => 1,
    :title => "Lorem ipsum",
    :content => "Dolor sit amet"
}).run(conn)
```


[Read more about this command &rarr;](insert/)


## [update](update/) ##

{% apibody %}
table.update(object | expr
    [, :durability => "hard", :return_changes => false, :non_atomic => false])
        &rarr; object
selection.update(object | expr
    [, :durability => "hard", :return_changes => false, :non_atomic => false])
        &rarr; object
singleSelection.update(object | expr
    [, :durability => "hard", :return_changes => false, :non_atomic => false])
        &rarr; object
{% endapibody %}

Update JSON documents in a table. Accepts a JSON document, a ReQL expression, or a
combination of the two.

__Example:__ Update the status of the post with `id` of `1` to `published`.

```rb
r.table("posts").get(1).update({status: "published"}).run(conn)
```

[Read more about this command &rarr;](update/)


## [replace](replace/) ##

{% apibody %}
table.replace(object | expr
    [, :durability => "hard", :return_changes => false, :non_atomic => false])
        &rarr; object
selection.replace(object | expr
    [, :durability => "hard", :return_changes => false, :non_atomic => false])
        &rarr; object
singleSelection.replace(object | expr
    [, :durability => "hard", :return_changes => false, :non_atomic => false])
        &rarr; object
{% endapibody %}

Replace documents in a table. Accepts a JSON document or a ReQL expression, and replaces
the original document with the new one. The new document must have the same primary key
as the original document.

__Example:__ Replace the document with the primary key `1`.

```rb
r.table("posts").get(1).replace({
    :id => 1,
    :title => "Lorem ipsum",
    :content => "Aleas jacta est",
    :status => "draft"
}).run(conn)
```

[Read more about this command &rarr;](replace/)


## [delete](delete/) ##

{% apibody %}
table.delete[({:durability => "hard", :return_changes => false})]
    &rarr; object
selection.delete[({:durability => "hard", :return_changes => false})]
    &rarr; object
singleSelection.delete[({:durability => "hard", :return_changes => false})]
    &rarr; object
{% endapibody %}

Delete one or more documents from a table.

__Example:__ Delete a single document from the table `comments`.

```rb
r.table("comments").get("7eab9e63-73f1-4f33-8ce4-95cbea626f59").delete.run(conn)
```

[Read more about this command &rarr;](delete/)

## [sync](sync/) ##

{% apibody %}
table.sync
    &rarr; object
{% endapibody %}

`sync` ensures that writes on a given table are written to permanent storage. Queries
that specify soft durability (`{:durability => soft}`) do not give such guarantees, so
`sync` can be used to ensure the state of these queries. A call to `sync` does not return
until all previous writes to the table are persisted.


__Example:__ After having updated multiple heroes with soft durability, we now want to wait
until these changes are persisted.

```rb
r.table('marvel').sync.run(conn)
```

{% endapisection %}


{% apisection Selecting data %}

## [db](db/) ##

{% apibody %}
r.db(db_name) &rarr; db
{% endapibody %}

Reference a database.

__Example:__ Explicitly specify a database for a query.

```rb
r.db('heroes').table('marvel').run(conn)
```

[Read more about this command &rarr;](db/)

## [table](table/) ##

{% apibody %}
db.table(name[, opts]) &rarr; table
{% endapibody %}

Select all documents in a table. This command can be chained with other commands to do
further processing on the data.

__Example:__ Return all documents in the table 'marvel' of the default database.

```rb
r.table('marvel').run(conn)
```

[Read more about this command &rarr;](table/)

## [get](get/) ##

{% apibody %}
table.get(key) &rarr; singleRowSelection
{% endapibody %}

Get a document by primary key.

If no document exists with that primary key, `get` will return `nil`.

__Example:__ Find a document by UUID.

```rb
r.table('posts').get('a9849eef-7176-4411-935b-79a6e3c56a74').run(conn)
```

[Read more about this command &rarr;](get/)

## [get_all](get_all/) ##

{% apibody %}
table.get_all(key[, key2...], [, :index => 'id']) &rarr; selection
{% endapibody %}

Get all documents where the given value matches the value of the requested index.

__Example:__ Secondary index keys are not guaranteed to be unique so we cannot query via [get](/api/ruby/get/) when using a secondary index.

```rb
r.table('marvel').get_all('man_of_steel', :index => 'code_name').run(conn)
```

[Read more about this command &rarr;](get_all/)

## [between](between/) ##

{% apibody %}
table.between(lower_key, upper_key
    [, :index => 'id', :left_bound => 'closed', :right_bound => 'open'])
        &rarr; selection
{% endapibody %}

Get all documents between two keys. Accepts three optional arguments: `index`,
`left_bound`, and `right_bound`. If `index` is set to the name of a secondary index,
`between` will return all documents where that index's value is in the specified range
(it uses the primary key by default). `left_bound` or `right_bound` may be set to `open`
or `closed` to indicate whether or not to include that endpoint of the range (by default,
`left_bound` is closed and `right_bound` is open).

__Example:__ Find all users with primary key >= 10 and < 20 (a normal half-open interval).

```rb
r.table('marvel').between(10, 20).run(conn)
```

[Read more about this command &rarr;](between/)

## [filter](filter/) ##

{% apibody %}
selection.filter(predicate[, :default => false]) &rarr; selection
stream.filter(predicate[, :default => false]) &rarr; stream
array.filter(predicate[, :default => false]) &rarr; array
{% endapibody %}


Get all the documents for which the given predicate is true.

`filter` can be called on a sequence, selection, or a field containing an array of
elements. The return type is the same as the type on which the function was called on.

The body of every filter is wrapped in an implicit `.default(false)`, which means that
if a non-existence errors is thrown (when you try to access a field that does not exist
in a document), RethinkDB will just ignore the document.
The `default` value can be changed by passing the symbol `default`.
Setting this optional argument to `r.error()` will cause any non-existence errors to
return a `RqlRuntimeError`.


__Example:__ Get all the users that are 30 years old.

```rb
r.table('users').filter({:age => 30}).run(conn)
```

[Read more about this command &rarr;](filter/)

{% endapisection %}


{% apisection Joins %}
These commands allow the combination of multiple sequences into a single sequence

## [inner_join](inner_join/) ##

{% apibody %}
sequence.inner_join(other_sequence, predicate) &rarr; stream
array.inner_join(other_sequence, predicate) &rarr; array
{% endapibody %}

Returns an inner join of two sequences.

__Example:__ Return a list of all matchups between Marvel and DC heroes in which the DC hero could beat the Marvel hero in a fight.

```rb
r.table('marvel').inner_join(r.table('dc')) {|marvel_row, dc_row|
    marvel_row[:strength] < dc_row[:strength]
}.zip().run(conn)
```

[Read more about this command &rarr;](inner_join/)

## [outer_join](outer_join/) ##

{% apibody %}
sequence.outer_join(other_sequence, predicate) &rarr; stream
array.outer_join(other_sequence, predicate) &rarr; array
{% endapibody %}

Returns a left outer join of two sequences.

__Example:__ Return a list of all Marvel heroes, paired with any DC heroes who could beat them in a fight.

```rb
r.table('marvel').outer_join(r.table('dc')) {|marvel_row, dc_row|
    marvel_row[:strength] < dc_row[:strength]
}.run(conn)
```

[Read more about this command &rarr;](outer_join/)

## [eq_join](eq_join/) ##

{% apibody %}
sequence.eq_join(left_field, right_table[, :index => 'id']) &rarr; sequence
sequence.eq_join(left_field, right_table[, :index => 'id']) &rarr; sequence
{% endapibody %}

Join tables using a field or function on the left-hand sequence matching primary keys or secondary indexes on the right-hand table. `eq_join` is more efficient than other ReQL join types, and operates much faster. Documents in the result set consist of pairs of left-hand and right-hand documents, matched when the field on the left-hand side exists and is non-null and an entry with that field's value exists in the specified index on the right-hand side.

**Example:** Match players with the games they've played against one another.

```rb
r.table('players').eq_join('game_id', r.table('games')).run(conn)
```

[Read more about this command &rarr;](eq_join/)


## [zip](zip/) ##

{% apibody %}
stream.zip() &rarr; stream
array.zip() &rarr; array
{% endapibody %}

Used to 'zip' up the result of a join by merging the 'right' fields into 'left' fields of each member of the sequence.

__Example:__ 'zips up' the sequence by merging the left and right fields produced by a join.

```
r.table('marvel').eq_join(:main_dc_collaborator, r.table('dc')).zip.run(conn)
```



{% endapisection %}

{% apisection Transformations %}
These commands are used to transform data in a sequence.

## [map](map/) ##

{% apibody %}
sequence1.map([sequence2, ...], mapping_function) &rarr; stream
array1.map([array2, ...], mapping_function) &rarr; array
r.map(sequence1[, sequence2, ...], mapping_function) &rarr; stream
r.map(array1[, array2, ...], mapping_function) &rarr; array
{% endapibody %}

Transform each element of one or more sequences by applying a mapping function to them. If `map` is run with two or more sequences, it will iterate for as many items as there are in the shortest sequence.

__Example:__ Return the first five squares.

```rb
> r.expr([1, 2, 3, 4, 5]).map{ |val| (val * val) }.run(conn)

[1, 4, 9, 16, 25]
```

[Read more about this command &rarr;](map/)

## [with_fields](with_fields/) ##

{% apibody %}
sequence.with_fields([selector1, selector2...]) &rarr; stream
array.with_fields([selector1, selector2...]) &rarr; array
{% endapibody %}

Plucks one or more attributes from a sequence of objects, filtering out any objects in the sequence that do not have the specified fields. Functionally, this is identical to `has_fields` followed by `pluck` on a sequence.

__Example:__ Get a list of users and their posts, excluding any users who have not made any posts.

```rb
r.table('users').with_fields('id', 'user', 'posts').run(conn)
```

[Read more about this command &rarr;](with_fields/)


## [concat_map](concat_map/) ##

{% apibody %}
stream.concat_map(mapping_function) &rarr; stream
array.concat_map(mapping_function) &rarr; array
{% endapibody %}

Concatenate one or more elements into a single sequence using a mapping function.

__Example:__ Construct a sequence of all monsters defeated by Marvel heroes. The field "defeatedMonsters" is an array of one or more monster names.

```rb
r.table('marvel').concat_map {|hero|
    hero[:defeated_monsters]
}.run(conn)

```

[Read more about this command &rarr;](concat_map/)


## [order_by](order_by/) ##

{% apibody %}
table.order_by([key1...], :index => index_name) -> selection<stream>
selection.order_by(key1, [key2...]) -> selection<array>
sequence.order_by(key1, [key2...]) -> array
{% endapibody %}

Sort the sequence by document values of the given key(s). To specify
the ordering, wrap the attribute with either `r.asc` or `r.desc`
(defaults to ascending).

Sorting without an index requires the server to hold the sequence in
memory, and is limited to 100,000 documents (or the setting of the `array_limit` option for [run](/api/ruby/run)). Sorting with an index can
be done on arbitrarily large tables, or after a `between` command
using the same index.

__Example:__ Order all the posts using the index `date`.

```rb
r.table('posts').order_by(:index => 'date').run(conn)
```

The index must have been previously created with [index_create](/api/ruby/index_create/).

```rb
r.table('posts').index_create('date').run(conn)
```

You can also select a descending ordering:

```rb
r.table('posts').order_by(:index => r.desc('date')).run(conn, callback)
```

[Read more about this command &rarr;](order_by/)


## [skip](skip/) ##

{% apibody %}
sequence.skip(n) &rarr; stream
array.skip(n) &rarr; array
{% endapibody %}

Skip a number of elements from the head of the sequence.

__Example:__ Here in conjunction with `order_by` we choose to ignore the most successful heroes.

```rb
r.table('marvel').order_by(:success_metric).skip(10).run(conn)
```


## [limit](limit/) ##

{% apibody %}
sequence.limit(n) &rarr; stream
array.limit(n) &rarr; array
{% endapibody %}


End the sequence after the given number of elements.

__Example:__ Only so many can fit in our Pantheon of heroes.

```rb
r.table('marvel').order_by(:belovedness).limit(10).run(conn)
```

## [slice](slice/) ##

{% apibody %}
selection.slice(start_index[, end_index, :left_bound => 'closed', :right_bound =>'open']) &rarr; selection
stream.slice(start_index[, end_index, :left_bound => 'closed', :right_bound =>'open']) &rarr; stream
array.slice(start_index[, end_index, :left_bound => 'closed', :right_bound =>'open']) &rarr; array
{% endapibody %}

Return the elements of a sequence within the specified range.

**Example:** Return the fourth, fifth and sixth youngest players. (The youngest player is at index 0, so those are elements 3&ndash;5.)

```rb
r.table('players').order_by(:index => 'age').slice(3,6).run(conn)
```

## [nth](nth/) ##

{% apibody %}
sequence.nth(index) &rarr; object
selection.nth(index) &rarr; selection&lt;object&gt;
{% endapibody %}

Get the *nth* element of a sequence, counting from zero. If the argument is negative, count from the last element.

__Example:__ Select the second element in the array.

```rb
r.expr([1,2,3]).nth(1).run(conn)
```

## [offsets_of](offsets_of/) ##

{% apibody %}
sequence.offsets_of(datum | predicate) &rarr; array
{% endapibody %}

Get the indexes of an element in a sequence. If the argument is a predicate, get the indexes of all elements matching it.

__Example:__ Find the position of the letter 'c'.

```rb
r.expr(['a','b','c']).offsets_of('c').run(conn)
```

[Read more about this command &rarr;](offsets_of/)



## [is_empty](is_empty/) ##

{% apibody %}
sequence.is_empty() &rarr; bool
{% endapibody %}

Test if a sequence is empty.

__Example:__ Are there any documents in the marvel table?

```rb
r.table('marvel').is_empty().run(conn)
```

## [union](union/) ##

{% apibody %}
stream.union(sequence[, sequence, ...]) &rarr; stream
array.union(sequence[, sequence, ...]) &rarr; array
{% endapibody %}

Concatenate two or more sequences.

__Example:__ Construct a stream of all heroes.

```rb
r.table('marvel').union(r.table('dc')).run(conn)
```


## [sample](sample/) ##

{% apibody %}
sequence.sample(number) &rarr; selection
stream.sample(number) &rarr; array
array.sample(number) &rarr; array
{% endapibody %}

Select a given number of elements from a sequence with uniform random distribution. Selection is done without replacement.

__Example:__ Select 3 random heroes.

```rb
r.table('marvel').sample(3).run(conn)
```


{% endapisection %}


{% apisection Aggregation %}
These commands are used to compute smaller values from large sequences.


## [group](group/) ##

{% apibody %}
sequence.group(field_or_function..., [:index => 'index_name', :multi => true]) &rarr; grouped_stream
{% endapibody %}

Takes a stream and partitions it into multiple groups based on the
fields or functions provided.  Commands chained after `group` will be
called on each of these grouped sub-streams, producing grouped data.

__Example:__ What is each player's best game?

```rb
r.table('games').group('player').max('points').run(conn)
```

[Read more about this command &rarr;](group/)

## [ungroup](ungroup/) ##

{% apibody %}
grouped_stream.ungroup() &rarr; array
grouped_data.ungroup() &rarr; array
{% endapibody %}

Takes a grouped stream or grouped data and turns it into an array of
objects representing the groups.  Any commands chained after `ungroup`
will operate on this array, rather than operating on each group
individually.  This is useful if you want to e.g. order the groups by
the value of their reduction.

The format of the array returned by `ungroup` is the same as the
default native format of grouped data in the javascript driver and
data explorer.

__Example:__ What is the maximum number of points scored by each
player, with the highest scorers first?

```rb
r.table('games')
   .group('player').max('points')['points']
   .ungroup().order_by(r.desc('reduction')).run(conn)
```

[Read more about this command &rarr;](ungroup/)



## [reduce](reduce/) ##

{% apibody %}
sequence.reduce(reduction_function) &rarr; value
{% endapibody %}

Produce a single value from a sequence through repeated application of a reduction
function.

__Example:__ Return the number of documents in the table `posts.

```rb
r.table("posts").map{|doc|
    1
}.reduce{ |left, right|
    left+right
}.run(conn);
```

[Read more about this command &rarr;](reduce/)

## [count](count/) ##

{% apibody %}
sequence.count([filter]) &rarr; number
{% endapibody %}

Count the number of elements in the sequence. With a single argument, count the number
of elements equal to it. If the argument is a function, it is equivalent to calling
filter before count.

__Example:__ Just how many super heroes are there?

```rb
(r.table('marvel').count() + r.table('dc').count()).run(conn)
```

[Read more about this command &rarr;](count/)


## [sum](sum/) ##

{% apibody %}
sequence.sum([field_or_function]) &rarr; number
{% endapibody %}

Sums all the elements of a sequence.  If called with a field name,
sums all the values of that field in the sequence, skipping elements
of the sequence that lack that field.  If called with a function,
calls that function on every element of the sequence and sums the
results, skipping elements of the sequence where that function returns
`nil` or a non-existence error.

__Example:__ What's 3 + 5 + 7?

```rb
r([3, 5, 7]).sum().run(conn)
```

[Read more about this command &rarr;](sum/)



## [avg](avg/) ##

{% apibody %}
sequence.avg([field_or_function]) &rarr; number
{% endapibody %}

Averages all the elements of a sequence.  If called with a field name,
averages all the values of that field in the sequence, skipping
elements of the sequence that lack that field.  If called with a
function, calls that function on every element of the sequence and
averages the results, skipping elements of the sequence where that
function returns `nil` or a non-existence error.


__Example:__ What's the average of 3, 5, and 7?

```rb
r([3, 5, 7]).avg().run(conn)
```


[Read more about this command &rarr;](avg/)


## [min](min/) ##

{% apibody %}
sequence.min(field_or_function) &rarr; element
sequence.min({:index => 'index'}) &rarr; element
{% endapibody %}

Finds the minimum element of a sequence.

__Example:__ Return the minimum value in the list `[3, 5, 7]`.

```rb
r([3, 5, 7]).min().run(conn)
```

[Read more about this command &rarr;](min/)


## [max](max/) ##

{% apibody %}
sequence.max(field_or_function) &rarr; element
sequence.max({:index => 'index'}) &rarr; element
{% endapibody %}

Finds the maximum element of a sequence.

__Example:__ Return the maximum value in the list `[3, 5, 7]`.

```rb
r([3, 5, 7]).max().run(conn)
```

[Read more about this command &rarr;](max/)


## [distinct](distinct/) ##

{% apibody %}
sequence.distinct() &rarr; array
table.distinct([:index => <indexname>]) &rarr; stream
{% endapibody %}

Remove duplicate elements from the sequence.

__Example:__ Which unique villains have been vanquished by marvel heroes?

```rb
r.table('marvel').concat_map{|hero| hero[:villain_list]}.distinct.run(conn)
```

## [contains](contains/) ##

{% apibody %}
sequence.contains(value|predicate[, value|predicate, ...]) &rarr; bool
{% endapibody %}

Returns whether or not a sequence contains all the specified values, or if functions are
provided instead, returns whether or not a sequence contains values matching all the
specified functions.

__Example:__ Has Iron Man ever fought Superman?

```rb
r.table('marvel').get('ironman')[:opponents].contains('superman').run(conn)
```

[Read more about this command &rarr;](contains/)





{% endapisection %}


{% apisection Document manipulation %}

## [pluck](pluck/) ##

{% apibody %}
sequence.pluck([selector1, selector2...]) &rarr; stream
array.pluck([selector1, selector2...]) &rarr; array
object.pluck([selector1, selector2...]) &rarr; object
singleSelection.pluck([selector1, selector2...]) &rarr; object
{% endapibody %}

Plucks out one or more attributes from either an object or a sequence of objects
(projection).

__Example:__ We just need information about IronMan's reactor and not the rest of the
document.

```rb
r.table('marvel').get('IronMan').pluck('reactorState', 'reactorPower').run(conn)
```

[Read more about this command &rarr;](pluck/)


## [without](without/) ##

{% apibody %}
sequence.without([selector1, selector2...]) &rarr; stream
array.without([selector1, selector2...]) &rarr; array
singleSelection.without([selector1, selector2...]) &rarr; object
object.without([selector1, selector2...]) &rarr; object
{% endapibody %}

The opposite of pluck; takes an object or a sequence of objects, and returns them with
the specified paths removed.

__Example:__ Since we don't need it for this computation we'll save bandwidth and leave
out the list of IronMan's romantic conquests.

```rb
r.table('marvel').get('IronMan').without('personalVictoriesList').run(conn)
```

[Read more about this command &rarr;](without/)


## [merge](merge/) ##

{% apibody %}
singleSelection.merge(object|function[, object|function, ...]) &rarr; object
object.merge(object|function[, object|function, ...]) &rarr; object
sequence.merge(object|function[, object|function, ...]) &rarr; stream
array.merge(object|function[, object|function, ...]) &rarr; array
{% endapibody %}

Merge two or more objects together to construct a new object with properties from all. When there is a conflict between field names, preference is given to fields in the rightmost object in the argument list.

__Example:__ Equip Thor for battle.

```rb
r.table('marvel').get('thor').merge(
    r.table('equipment').get('hammer'),
    r.table('equipment').get('pimento_sandwich')
).run(conn)
```

[Read more about this command &rarr;](merge/)

## [append](append/) ##

{% apibody %}
array.append(value) &rarr; array
{% endapibody %}

Append a value to an array.

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```rb
r.table('marvel').get('IronMan')[:equipment].append('new_boots').run(conn)
```


## [prepend](prepend/) ##

{% apibody %}
array.prepend(value) &rarr; array
{% endapibody %}

Prepend a value to an array.

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```rb
r.table('marvel').get('IronMan')[:equipment].prepend('new_boots').run(conn)
```


## [difference](difference/) ##

{% apibody %}
array.difference(array) &rarr; array
{% endapibody %}

Remove the elements of one array from another array.

__Example:__ Retrieve Iron Man's equipment list without boots.

```rb
r.table('marvel').get('IronMan')[:equipment].difference(['Boots']).run(conn)
```


## [set_insert](set_insert/) ##

{% apibody %}
array.set_insert(value) &rarr; array
{% endapibody %}

Add a value to an array and return it as a set (an array with distinct values).

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```rb
r.table('marvel').get('IronMan')[:equipment].set_insert('new_boots').run(conn)
```


## [set_union](set_union/) ##

{% apibody %}
array.set_union(array) &rarr; array
{% endapibody %}

Add a several values to an array and return it as a set (an array with distinct values).

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots and an arc reactor.

```rb
r.table('marvel').get('IronMan')[:equipment].set_union(['newBoots', 'arc_reactor']).run(conn)
```


## [set_intersection](set_intersection/) ##

{% apibody %}
array.set_intersection(array) &rarr; array
{% endapibody %}

Intersect two arrays returning values that occur in both of them as a set (an array with
distinct values).

__Example:__ Check which pieces of equipment Iron Man has from a fixed list.

```rb
r.table('marvel').get('IronMan')[:equipment].set_intersection(['newBoots', 'arc_reactor']).run(conn)
```


## [set_difference](set_difference/) ##

{% apibody %}
array.set_difference(array) &rarr; array
{% endapibody %}

Remove the elements of one array from another and return them as a set (an array with
distinct values).

__Example:__ Check which pieces of equipment Iron Man has, excluding a fixed list.

```rb
r.table('marvel').get('IronMan')[:equipment].set_difference(['newBoots', 'arc_reactor']).run(conn)
```

## [\[\]](bracket/) ##

{% apibody %}
sequence[attr] &rarr; sequence
singleSelection[attr] &rarr; value
object[attr] &rarr; value
array[index] &rarr; value
{% endapibody %}

Get a single field from an object or a single element from a sequence.

__Example:__ What was Iron Man's first appearance in a comic?

```rb
r.table('marvel').get('IronMan')[:first_appearance].run(conn)
```

[Read more about this command &rarr;](bracket/)

## [get_field](get_field/) ##

{% apibody %}
sequence.get_field(attr) &rarr; sequence
singleSelection.get_field(attr) &rarr; value
object.get_field(attr) &rarr; value
{% endapibody %}

Get a single field from an object. If called on a sequence, gets that field from every
object in the sequence, skipping objects that lack it.

__Example:__ What was Iron Man's first appearance in a comic?

```rb
r.table('marvel').get('IronMan').get_field['first_appearance'].run(conn)
```


## [has_fields](has_fields/) ##

{% apibody %}
sequence.has_fields([selector1, selector2...]) &rarr; stream
array.has_fields([selector1, selector2...]) &rarr; array
object.has_fields([selector1, selector2...]) &rarr; boolean
{% endapibody %}

Test if an object has one or more fields. An object has a field if it has that key and the key has a non-null value. For instance, the object `{'a': 1,'b': 2,'c': null}` has the fields `a` and `b`.

__Example:__ Return the players who have won games.

```rb
r.table('players').has_fields(:games_won).run(conn)
```

[Read more about this command &rarr;](has_fields/)


## [insert_at](insert_at/) ##

{% apibody %}
array.insert_at(index, value) &rarr; array
{% endapibody %}

Insert a value in to an array at a given index. Returns the modified array.

__Example:__ Hulk decides to join the avengers.

```rb
r.expr(["Iron Man", "Spider-Man"]).insert_at(1, "Hulk").run(conn)
```


## [splice_at](splice_at/) ##

{% apibody %}
array.splice_at(index, array) &rarr; array
{% endapibody %}

Insert several values in to an array at a given index. Returns the modified array.

__Example:__ Hulk and Thor decide to join the avengers.

```rb
r.expr(["Iron Man", "Spider-Man"]).splice_at(1, ["Hulk", "Thor"]).run(conn)
```


## [delete_at](delete_at/) ##

{% apibody %}
array.delete_at(index [,endIndex]) &rarr; array
{% endapibody %}

Remove one or more elements from an array at a given index. Returns the modified array.

__Example:__ Delete the second element of an array.

```rb
> r.expr(['a','b','c','d','e','f']).delete_at(1).run(conn)

['a', 'c', 'd', 'e', 'f']
```

[Read more about this command &rarr;](delete_at/)

## [change_at](change_at/) ##

{% apibody %}
array.change_at(index, value) &rarr; array
{% endapibody %}

Change a value in an array at a given index. Returns the modified array.

__Example:__ Bruce Banner hulks out.

```rb
r.expr(["Iron Man", "Bruce", "Spider-Man"]).change_at(1, "Hulk").run(conn)
```

## [keys](keys/) ##

{% apibody %}
singleSelection.keys() &rarr; array
object.keys() &rarr; array
{% endapibody %}

Return an array containing all of the object's keys.

__Example:__ Get all the keys of a row.

```rb
r.table('marvel').get('ironman').keys.run(conn)
```

## [literal](literal/) ##

{% apibody %}
r.literal(object) &rarr; special
{% endapibody %}

Replace an object in a field instead of merging it with an existing object in a `merge` or `update` operation.

```rb
r.table('users').get(1).update({ :data => r.literal({ :age => 19, :job => 'Engineer' }) }).run(conn)
```

[Read more about this command &rarr;](literal/)

## [object](object/) ##

{% apibody %}
r.object([key, value,]...) &rarr; object
{% endapibody %}

Creates an object from a list of key-value pairs, where the keys must
be strings.  `r.object(A, B, C, D)` is equivalent to
`r.expr([[A, B], [C, D]]).coerce_to('OBJECT')`.

__Example:__ Create a simple object.

```rb
> r.object('id', 5, 'data', ['foo', 'bar']).run(conn)
{data: ["foo", "bar"], id: 5}
```

{% endapisection %}


{% apisection String manipulation %}
These commands provide string operators.

## [match](match/) ##

{% apibody %}
string.match(regexp) &rarr; nil/object
{% endapibody %}

Matches against a regular expression. If there is a match, returns an object with the fields:

- `str`: The matched string
- `start`: The matched string's start
- `end`: The matched string's end
- `groups`: The capture groups defined with parentheses

If no match is found, returns `nil`.

__Example:__ Get all users whose name starts with "A".

```rb
r.table('users').filter{ |doc|
    doc['name'].match("^A")
}.run(conn)
```


[Read more about this command &rarr;](match/)

## [split](split/) ##

{% apibody %}
string.split([separator, [max_splits]]) &rarr; array
{% endapibody %}

Splits a string into substrings.  Splits on whitespace when called
with no arguments.  When called with a separator, splits on that
separator.  When called with a separator and a maximum number of
splits, splits on that separator at most `max_splits` times.  (Can be
called with `nil` as the separator if you want to split on whitespace
while still specifying `max_splits`.)

Mimics the behavior of Python's `string.split` in edge cases, except
for splitting on the empty string, which instead produces an array of
single-character strings.

__Example:__ Split on whitespace.

```rb
> r.expr("foo  bar bax").split().run(conn)
["foo", "bar", "bax"]
```

[Read more about this command &rarr;](split/)

## [upcase](upcase/) ##

{% apibody %}
string.upcase() &rarr; string
{% endapibody %}


Uppercases a string.

__Example:__

```rb
> r.expr("Sentence about LaTeX.").upcase().run(conn)
"SENTENCE ABOUT LATEX."
```

## [downcase](downcase/) ##

{% apibody %}
string.downcase() &rarr; string
{% endapibody %}

Lowercases a string.

__Example:__

```rb
> r.expr("Sentence about LaTeX.").downcase().run(conn)
"sentence about latex."
```

{% endapisection %}


{% apisection Math and logic %}

## [+](add/) ##

{% apibody %}
number + number &rarr; number
string + string &rarr; string
array + array &rarr; array
time + number &rarr; time
{% endapibody %}

Sum two numbers, concatenate two strings, or concatenate 2 arrays.

__Example:__ It's as easy as 2 + 2 = 4.

```rb
(r.expr(2) + 2).run(conn)
```

[Read more about this command &rarr;](add/)


## [-](sub/) ##

{% apibody %}
number - number &rarr; number
time - time &rarr; number
time - number &rarr; time
{% endapibody %}

Subtract two numbers.

__Example:__ It's as easy as 2 - 2 = 0.

```rb
(r.expr(2) - 2).run(conn)
```

[Read more about this command &rarr;](sub/)

## [*](mul/) ##

{% apibody %}
number * number &rarr; number
array * number &rarr; array
{% endapibody %}

Multiply two numbers, or make a periodic array.

__Example:__ It's as easy as 2 * 2 = 4.

```rb
(r.expr(2) * 2).run(conn)
```

[Read more about this command &rarr;](mul/)

## [/](div/) ##

{% apibody %}
number / number &rarr; number
{% endapibody %}

Divide two numbers.

__Example:__ It's as easy as 2 / 2 = 1.

```rb
(r.expr(2) / 2).run(conn)
```



## [%](mod/) ##

{% apibody %}
number % number &rarr; number
{% endapibody %}

Find the remainder when dividing two numbers.

__Example:__ It's as easy as 2 % 2 = 0.

```rb
(r.expr(2) % 2).run(conn)
```

## [&, and](and/) ##

{% apibody %}
bool & bool &rarr; bool
bool.and(bool) &rarr; bool
r.and(bool, bool) &rarr; bool
{% endapibody %}

Compute the logical "and" of two or more values.
__Example:__ Return whether both `a` and `b` evaluate to true.

```rb
> a = true
> b = false
> (r.expr(a) & b).run(conn)

false
```


## [|, or](or/) ##

{% apibody %}
bool | bool &rarr; bool
bool.or(bool) &rarr; bool
r.or(bool, bool) &rarr; bool
{% endapibody %}

Compute the logical "or" of two or more values.

__Example:__ Return whether either `a` or `b` evaluate to true.

```rb
> a = true
> b = false
> (r.expr(a) | b).run(conn)

true
```

## [eq](eq/) ##

{% apibody %}
value.eq(value) &rarr; bool
{% endapibody %}

Test if two values are equal.

__Example:__ Does 2 equal 2?

```rb
r.expr(2).eq(2).run(conn)
```


## [ne](ne/) ##

{% apibody %}
value.ne(value) &rarr; bool
{% endapibody %}

Test if two values are not equal.

__Example:__ Does 2 not equal 2?

```rb
r.expr(2).ne(2).run(conn)
```


## [>, gt](gt/) ##

{% apibody %}
value > value &rarr; bool
value.gt(value) &rarr; bool
{% endapibody %}

Test if the first value is greater than other.

__Example:__ Is 2 greater than 2?

```rb
(r.expr(2) > 2).run(conn)
r.expr(2).gt(2).run(conn)
```

## [>=, ge](ge/) ##

{% apibody %}
value >= value &rarr; bool
value.ge(value) &rarr; bool
{% endapibody %}

Test if the first value is greater than or equal to other.

__Example:__ Is 2 greater than or equal to 2?

```rb
(r.expr(2) >= 2).run(conn)
r.expr(2).ge(2).run(conn)
```

## [<, lt](lt/) ##

{% apibody %}
value < value &rarr; bool
value.lt(value) &rarr; bool
{% endapibody %}

Test if the first value is less than other.

__Example:__ Is 2 less than 2?

```rb
(r.expr(2) < 2).run(conn)
r.expr(2).lt(2).run(conn)
```

## [<=, le](le/) ##

{% apibody %}
value <= value &rarr; bool
value.le(value) &rarr; bool
{% endapibody %}

Test if the first value is less than or equal to other.

__Example:__ Is 2 less than or equal to 2?

```rb
(r.expr(2) <= 2).run(conn)
r.expr(2).le(2).run(conn)
```


## [not](not/) ##

{% apibody %}
bool.not() &rarr; bool
not(bool) &rarr; bool
{% endapibody %}

Compute the logical inverse (not) of an expression.

`not` can be called either via method chaining, immediately after an expression that evaluates as a boolean value, or by passing the expression as a parameter to `not`.

__Example:__ Not true is false.

```ruby
r(true).not().run(conn)
r.not(true).run(conn)
```

[Read more about this command &rarr;](not/)

## [random](random/) ##

{% apibody %}
r.random() &rarr; number
r.random(number[, number], :float => true) &rarr; number
r.random(integer[, integer]) &rarr; integer
{% endapibody %}

Generate a random number between given (or implied) bounds. `random` takes zero, one or two arguments.

__Example:__ Generate a random number in the range `[0,1)`

```rb
r.random().run(conn)
```

[Read more about this command &rarr;](random/)

{% endapisection %}


{% apisection Dates and times %}

## [now](now/) ##

{% apibody %}
r.now() &rarr; time
{% endapibody %}

Return a time object representing the current time in UTC. The command now() is computed once when the server receives the query, so multiple instances of r.now() will always return the same time inside a query.

__Example:__ Add a new user with the time at which he subscribed.

```rb
r.table("users").insert({
    :name => "John",
    :subscription_date => r.now()
}).run(conn)
```

## [time](time/) ##

{% apibody %}
r.time(year, month, day[, hour, minute, second], timezone)
    &rarr; time
{% endapibody %}

Create a time object for a specific time.

A few restrictions exist on the arguments:

- `year` is an integer between 1400 and 9,999.
- `month` is an integer between 1 and 12.
- `day` is an integer between 1 and 31.
- `hour` is an integer.
- `minutes` is an integer.
- `seconds` is a double. Its value will be rounded to three decimal places
(millisecond-precision).
- `timezone` can be `'Z'` (for UTC) or a string with the format `±[hh]:[mm]`.


__Example:__ Update the birthdate of the user "John" to November 3rd, 1986 UTC.

```rb
r.table("user").get("John").update(:birthdate => r.time(1986, 11, 3, 'Z')).run(conn)
```



## [epoch_time](epoch_time/) ##

{% apibody %}
r.epoch_time(epoch_time) &rarr; time
{% endapibody %}

Create a time object based on seconds since epoch. The first argument is a double and
will be rounded to three decimal places (millisecond-precision).

__Example:__ Update the birthdate of the user "John" to November 3rd, 1986.

```rb
r.table("user").get("John").update(:birthdate => r.epoch_time(531360000)).run(conn)
```


## [iso8601](iso8601/) ##

{% apibody %}
r.iso8601(iso8601Date[, {default_timezone:''}]) &rarr; time
{% endapibody %}

Create a time object based on an ISO 8601 date-time string (e.g. '2013-01-01T01:01:01+00:00'). We support all valid ISO 8601 formats except for week dates. If you pass an ISO 8601 date-time without a time zone, you must specify the time zone with the `default_timezone` argument. Read more about the ISO 8601 format at [Wikipedia](http://en.wikipedia.org/wiki/ISO_8601).

__Example:__ Update the time of John's birth.

```rb
r.table("user").get("John").update(:birth => r.iso8601('1986-11-03T08:30:00-07:00')).run(conn)
```


## [in_timezone](in_timezone/) ##

{% apibody %}
time.in_timezone(timezone) &rarr; time
{% endapibody %}

Return a new time object with a different timezone. While the time stays the same, the results returned by methods such as hours() will change since they take the timezone into account. The timezone argument has to be of the ISO 8601 format.

__Example:__ Hour of the day in San Francisco (UTC/GMT -8, without daylight saving time).

```rb
r.now().in_timezone('-08:00').hours().run(conn)
```



## [timezone](timezone/) ##

{% apibody %}
time.timezone() &rarr; string
{% endapibody %}

Return the timezone of the time object.

__Example:__ Return all the users in the "-07:00" timezone.

```rb
r.table("users").filter{ |user|
    user["subscriptionDate"].timezone().eq("07:00")
}
```


## [during](during/) ##

{% apibody %}
time.during(start_time, end_time
    [, left_bound="open/closed", right_bound="open/closed"])
        &rarr; bool
{% endapibody %}

Return if a time is between two other times (by default, inclusive for the start, exclusive for the end).

__Example:__ Retrieve all the posts that were posted between December 1st, 2013 (inclusive) and December 10th, 2013 (exclusive).

```rb
r.table("posts").filter{ |post|
    post['date'].during(r.time(2013, 12, 1), r.time(2013, 12, 10))
}.run(conn)
```

[Read more about this command &rarr;](during/)



## [date](date/) ##

{% apibody %}
time.date() &rarr; time
{% endapibody %}

Return a new time object only based on the day, month and year (ie. the same day at 00:00).

__Example:__ Retrieve all the users whose birthday is today

```rb
r.table("users").filter{ |user|
    user["birthdate"].date().eq(r.now().date())
}.run(conn)
```



## [time\_of\_day](time_of_day/) ##

{% apibody %}
time.time_of_day() &rarr; number
{% endapibody %}

Return the number of seconds elapsed since the beginning of the day stored in the time object.

__Example:__ Retrieve posts that were submitted before noon.

```rb
r.table("posts").filter{ |post|
    post["date"].time_of_day() <= 12*60*60
}.run(conn)
```


## [year](year/) ##

{% apibody %}
time.year() &rarr; number
{% endapibody %}

Return the year of a time object.

__Example:__ Retrieve all the users born in 1986.

```rb
r.table("users").filter{ |user|
    user["birthdate"].year().eq(1986)
}.run(conn)

```


## [month](month/) ##

{% apibody %}
time.month() &rarr; number
{% endapibody %}

Return the month of a time object as a number between 1 and 12. For your convenience, the terms r.january, r.february etc. are defined and map to the appropriate integer.

__Example:__ Retrieve all the users who were born in November.

```rb
r.table("users").filter{ |user|
    user["birthdate"].month().eq(11)
}
```

[Read more about this command &rarr;](month/)



## [day](day/) ##

{% apibody %}
time.day() &rarr; number
{% endapibody %}

Return the day of a time object as a number between 1 and 31.

__Example:__ Return the users born on the 24th of any month.

```rb
r.table("users").filter{ |user|
    user["birthdate"].day().eq(24)
}
```



## [day\_of\_week](day_of_week/) ##

{% apibody %}
time.day_of_week() &rarr; number
{% endapibody %}

Return the day of week of a time object as a number between 1 and 7 (following ISO 8601 standard). For your convenience, the terms r.monday, r.tuesday etc. are defined and map to the appropriate integer.

__Example:__ Return today's day of week.

```rb
r.now().day_of_week().run(conn)
```

[Read more about this command &rarr;](day_of_week/)


## [day\_of\_year](day_of_year/) ##

{% apibody %}
time.day_of_year() &rarr; number
{% endapibody %}

Return the day of the year of a time object as a number between 1 and 366 (following ISO 8601 standard).

__Example:__ Retrieve all the users who were born the first day of a year.

```rb
r.table("users").filter{ |user|
    user["birthdate"].day_of_year().eq(1)
}
```


## [hours](hours/) ##

{% apibody %}
time.hours() &rarr; number
{% endapibody %}

Return the hour in a time object as a number between 0 and 23.

__Example:__ Return all the posts submitted after midnight and before 4am.

```rb
r.table("posts").filter{ |post|
    post["date"].hours() < 4
}
```


## [minutes](minutes/) ##

{% apibody %}
time.minutes() &rarr; number
{% endapibody %}

Return the minute in a time object as a number between 0 and 59.

__Example:__ Return all the posts submitted during the first 10 minutes of every hour.

```rb
r.table("posts").filter{ |post|
    post["date"].minutes() < 10
}
```



## [seconds](seconds/) ##

{% apibody %}
time.seconds() &rarr; number
{% endapibody %}

Return the seconds in a time object as a number between 0 and 59.999 (double precision).

__Example:__ Return the post submitted during the first 30 seconds of every minute.

```rb
r.table("posts").filter{ |post|
    post["date"].seconds() < 30
}
```


## [to_iso8601](to_iso8601/) ##

{% apibody %}
time.to_iso8601() &rarr; string
{% endapibody %}

Convert a time object to a string in ISO 8601 format.

__Example:__ Return the current ISO 8601 time.

```rb
> r.now().to_iso8601().run(conn)

"2015-04-20T18:37:52.690+00:00"
```


## [to\_epoch\_time](to_epoch_time/) ##

{% apibody %}
time.to_epoch_time() &rarr; number
{% endapibody %}

Convert a time object to its epoch time.

__Example:__ Return the current time in seconds since the Unix Epoch with millisecond-precision.

```rb
r.now().to_epoch_time()
```



{% endapisection %}


{% apisection Control structures %}

## [args](args/) ##

{% apibody %}
r.args(array) &rarr; special
{% endapibody %}

`r.args` is a special term that's used to splice an array of arguments
into another term.  This is useful when you want to call a variadic
term such as `get_all` with a set of arguments produced at runtime.

This is analogous to the **splat operator** in Ruby.

__Example:__ Get Alice and Bob from the table `people`.

```rb
r.table('people').get_all('Alice', 'Bob').run(conn)
# or
r.table('people').get_all(r.args(['Alice', 'Bob'])).run(conn)
```

## [binary](binary/) ##

{% apibody %}
r.binary(data) &rarr; binary
{% endapibody %}

Encapsulate binary data within a query.

__Example:__ Save an avatar image to a existing user record.

```rb
f = File.open('./default_avatar.png', 'rb')
avatar_image = f.read()
f.close()
r.table('users').get(100).update({:avatar => r.binary(avatar_image)}).run(conn)
```

## [do](do/) ##

{% apibody %}
any.do(function) &rarr; any
r.do([args]*, function) &rarr; any
any.do(expr) &rarr; any
r.do([args]*, expr) &rarr; any
{% endapibody %}

Call an anonymous function using return values from other ReQL commands or queries as arguments.

__Example:__ Compute a golfer's net score for a game.


```rb
r.table('players').get('f19b5f16-ef14-468f-bd48-e194761df255').do { |player|
    player['gross_score'] - player['course_handicap']
}.run(conn)
```

[Read more about this command &rarr;](do/)

## [branch](branch/) ##

{% apibody %}
r.branch(test, true_branch, false_branch) &rarr; any
{% endapibody %}

If the `test` expression returns `false` or `nil`, the `false_branch` will be evaluated.
Otherwise, the `true_branch` will be evaluated.

The `branch` command is effectively an `if` renamed due to language constraints.
The type of the result is determined by the type of the branch that gets executed.

__Example:__ Return heroes and superheroes.

```rb
r.table('marvel').map{ |hero|
    r.branch(
        hero['victories'] > 100,
        hero['name'].add(' is a superhero'),
        hero['name'].add(' is a hero')
    )
}.run(conn)
```

## [for_each](for_each/) ##

{% apibody %}
sequence.for_each(write_query) &rarr; object
{% endapibody %}

Loop over a sequence, evaluating the given write query for each element.

__Example:__ Now that our heroes have defeated their villains, we can safely remove them from the villain table.

```rb
r.table('marvel').for_each {|hero|
    r.table('villains').get(hero[:villain_defeated]).delete
}.run(conn)
```

## [range](range/) ##

{% apibody %}
r.range() &rarr; stream
r.range([start_value, ]end_value) &rarr; stream
{% endapibody %}

Generate a stream of sequential integers in a specified range.

__Example:__ Return a four-element range of `[0, 1, 2, 3]`.

```rb
> r.range(4).run(conn)

[0, 1, 2, 3]
```


## [error](error/) ##

{% apibody %}
r.error(message) &rarr; error
{% endapibody %}

Throw a runtime error. If called with no arguments inside the second argument to `default`, re-throw the current error.

__Example:__ Iron Man can't possibly have lost a battle:

```rb
r.table('marvel').get('IronMan').do { |ironman|
    r.branch(ironman[:victories] < ironman[:battles],
    r.error('impossible code path'),
    ironman)
}.run(conn)
```

## [default](default/) ##

{% apibody %}
value.default(default_value) &rarr; any
sequence.default(default_value) &rarr; any
{% endapibody %}

Handle non-existence errors. Tries to evaluate and return its first argument. If an
error related to the absence of a value is thrown in the process, or if its first
argument returns `nil`, returns its second argument. (Alternatively, the second argument
may be a function which will be called with either the text of the non-existence error
or `nil`.)

__Example:__ Suppose we want to retrieve the titles and authors of the table `posts`.
In the case where the author field is missing or `nil`, we want to retrieve the string
`Anonymous`.


```rb
r.table("posts").map{ |post|
    {
        :title => post["title"],
        :author => post["author"].default("Anonymous")
    }
}.run(conn)
```

[Read more about this command &rarr;](default/)

## [expr](expr/) ##

{% apibody %}
r.expr(value) &rarr; value
{% endapibody %}

Construct a ReQL JSON object from a native object.

__Example:__ Objects wrapped with `expr` can then be manipulated by ReQL API functions.

```rb
r.expr({:a => 'b'}).merge({:b => [1,2,3]}).run(conn)
```

[Read more about this command &rarr;](expr/)

## [js](js/) ##

{% apibody %}
r.js(js_string[, :timeout => <number>]) &rarr; value
{% endapibody %}

Create a javascript expression.

__Example:__ Concatenate two strings using JavaScript.

```rb
r.js("'str1' + 'str2'").run(conn)
```

[Read more about this command &rarr;](js/)

## [coerce_to](coerce_to/) ##

{% apibody %}
sequence.coerce_to('array') &rarr; array
value.coerce_to('string') &rarr; string
string.coerce_to('number') &rarr; number
array.coerce_to('object') &rarr; object
object.coerce_to('array') &rarr; array
{% endapibody %}

Convert a value of one type into another.

__Example:__ Coerce a stream to an array.

```rb
r.table('posts').map { |post|
    { :comments => r.table('comments').get_all(post['id'], {:index => 'post_id'}).coerce_to('array') }
}.run(conn)
```

[Read more about this command &rarr;](coerce_to/)

## [type_of](type_of/) ##

{% apibody %}
any.type_of() &rarr; string
{% endapibody %}

Gets the type of a value.

__Example:__ Get the type of a string.

```rb
r.expr("foo").type_of().run(conn)
```


## [info](info/) ##

{% apibody %}
any.info() &rarr; object
{% endapibody %}

Get information about a ReQL value.

__Example:__ Get information about a table such as primary key, or cache size.

```rb
r.table('marvel').info().run(conn)
```

## [json](json/) ##

{% apibody %}
r.json(json_string) &rarr; value
{% endapibody %}

Parse a JSON string on the server.

__Example:__ Send an array to the server.

```rb
r.json("[1,2,3]").run(conn)
```

## [to_json_string](to_json_string/) ##

{% apibody %}
value.to_json_string() &rarr; string
{% endapibody %}

Convert a ReQL value or object to a JSON string.

__Example:__ Get a ReQL document as a JSON string.

```rb
> r.table('hero').get(1).to_json_string()

'{"id": 1, "name": "Batman", "city": "Gotham", "powers": ["martial arts", "cinematic entrances"]}'
```

## [http](http/) ##

{% apibody %}
r.http(url [, options]) &rarr; value
{% endapibody %}

Retrieve data from the specified URL over HTTP.  The return type depends on the `result_format` option, which checks the `Content-Type` of the response by default.

__Example:__ Perform a simple HTTP `GET` request, and store the result in a table.

```rb
r.table('posts').insert(r.http('http://httpbin.org/get')).run(conn)
```

[Read more about this command &rarr;](http/)

## [uuid](uuid/) ##

{% apibody %}
r.uuid() &rarr; string
{% endapibody %}

Return a UUID (universally unique identifier), a string that can be used as a unique ID.

__Example:__ Generate a UUID.

```rb
> r.uuid().run(conn)

27961a0e-f4e8-4eb3-bf95-c5203e1d87b9
```

{% endapisection %}

{% apisection Geospatial commands %}

## [circle](circle/) ##

{% apibody %}
r.circle([longitude, latitude], radius[, {:num_vertices => 32, :geo_system => 'WGS84', :unit => 'm', :fill => true}]) &rarr; geometry
r.circle(point, radius[, {:num_vertices => 32, :geo_system => 'WGS84', :unit => 'm', :fill => true}]) &rarr; geometry
{% endapibody %}

Construct a circular line or polygon. A circle in RethinkDB is a polygon or line *approximating* a circle of a given radius around a given center, consisting of a specified number of vertices (default 32).

__Example:__ Define a circle.

```rb
r.table('geo').insert({
    :id => 300,
    :name => 'Hayes Valley',
    :neighborhood => r.circle([-122.423246,37.779388], 1000)
}).run(conn)
```

[Read more about this command &rarr;](circle/)

## [distance](distance/) ##

{% apibody %}
geometry.distance(geometry[, {:geo_system => 'WGS84', :unit => 'm'}]) &rarr; number
{% endapibody %}

Compute the distance between a point and another geometry object. At least one of the geometry objects specified must be a point.

__Example:__ Compute the distance between two points on the Earth in kilometers.

```rb
> point1 = r.point(-122.423246,37.779388)
> point2 = r.point(-117.220406,32.719464)
> r.distance(point1, point2, {:unit => 'km'}).run(conn)

734.1252496021841
```

[Read more about this command &rarr;](distance/)

## [fill](fill/) ##

{% apibody %}
line.fill() &rarr; polygon
{% endapibody %}

Convert a Line object into a Polygon object. If the last point does not specify the same coordinates as the first point, `polygon` will close the polygon by connecting them.

__Example:__ Create a line object and then convert it to a polygon.

```rb
r.table('geo').insert({
    :id => 201,
    :rectangle => r.line(
        [37.779388,-122.423246],
        [37.329898,-122.423246],
        [37.329898,-121.886420],
        [37.779388,-121.886420]
    )
}).run(conn)

r.table('geo').get(201).update({
    :rectangle => r.row('rectangle').fill()
}).run(conn)
```

[Read more about this command &rarr;](fill/)

## [geojson](geojson/) ##

{% apibody %}
r.geojson(geojson) &rarr; geometry
{% endapibody %}

Convert a [GeoJSON][] object to a ReQL geometry object.

[GeoJSON]: http://geojson.org

__Example:__ Convert a GeoJSON object to a ReQL geometry object.

```rb
geo_json = {
    :type => 'Point',
    :coordinates => [ -122.423246, 37.779388 ]
}
r.table('geo').insert({
    :id => 'sfo',
    :name => 'San Francisco',
    :location => r.geojson(geo_json)
}).run(conn)
```

[Read more about this command &rarr;](geojson/)

## [to_geojson](to_geojson/) ##

{% apibody %}
geometry.to_geojson() &rarr; object
{% endapibody %}

Convert a ReQL geometry object to a [GeoJSON][] object.

__Example:__ Convert a ReQL geometry object to a GeoJSON object.

```rb
> r.table(geo).get('sfo')['location'].to_geojson.run(conn)

{
    :type => 'Point',
    :coordinates => [ -122.423246, 37.779388 ]
}
```

[Read more about this command &rarr;](to_geojson/)

## [get_intersecting](get_intersecting/) ##

{% apibody %}
table.get_intersecting(geometry, {:index => 'indexname'}) &rarr; selection<stream>
{% endapibody %}

Get all documents where the given geometry object intersects the geometry object of the requested geospatial index.

__Example:__ Which of the locations in a list of parks intersect `circle1`?

```rb
circle1 = r.circle([-117.220406,32.719464], 10, {:unit => 'mi'})
r.table('parks').get_intersecting(circle1, {:index => 'area'}).run(conn)
```

[Read more about this command &rarr;](get_intersecting/)

## [get_nearest](get_nearest/) ##

{% apibody %}
table.get_nearest(point, {:index => 'indexname'[, :max_results => 100, :max_dist => 100000, :unit => 'm', :geo_system => 'WGS84']}) &rarr; selection<array>
{% endapibody %}

Get all documents where the specified geospatial index is within a certain distance of the specified point (default 100 kilometers).

__Example:__ Return a list of enemy hideouts within 5000 meters of the secret base.

```rb
secret_base = r.point(-122.422876,37.777128)
r.table('hideouts').get_nearest(secret_base, {:index => 'location',
    :max_dist => 5000}).run(conn)
```

[Read more about this command &rarr;](get_nearest/)

## [includes](includes/) ##

{% apibody %}
sequence.includes(geometry) &rarr; sequence
geometry.includes(geometry) &rarr; bool
{% endapibody %}

Tests whether a geometry object is completely contained within another. When applied to a sequence of geometry objects, `includes` acts as a [filter](/api/ruby/filter), returning a sequence of objects from the sequence that include the argument.

__Example:__ Is `point2` included within a 2000-meter circle around `point1`?

```rb
> point1 = r.point(-117.220406,32.719464)
> point2 = r.point(-117.206201,32.725186)
> r.circle(point1, 2000).includes(point2).run(conn)

true
```

[Read more about this command &rarr;](includes/)

## [intersects](intersects/) ##

{% apibody %}
sequence.intersects(geometry) &rarr; sequence
geometry.intersects(geometry) &rarr; bool
{% endapibody %}

Tests whether two geometry objects intersect with one another. When applied to a sequence of geometry objects, `intersects` acts as a [filter](/api/ruby/filter), returning a sequence of objects from the sequence that intersect with the argument.

__Example:__ Is `point2` within a 2000-meter circle around `point1`?

```rb
> point1 = r.point(-117.220406,32.719464)
> point2 = r.point(-117.206201,32.725186)
> r.circle(point1, 2000).intersects(point2).run(conn)

true
```

[Read more about this command &rarr;](intersects/)

## [line](line/) ##

{% apibody %}
r.line([lon1, lat1], [lon2, lat2], ...) &rarr; line
r.line(point1, point2, ...) &rarr; line
{% endapibody %}

Construct a geometry object of type Line. The line can be specified in one of two ways:

* Two or more two-item arrays, specifying longitude and latitude numbers of the line's vertices;
* Two or more [Point](/api/ruby/point) objects specifying the line's vertices.

__Example:__ Define a line.

```rb
r.table('geo').insert({
    :id => 101,
    :route => r.line([-122.423246,37.779388], [-121.886420,37.329898])
}).run(conn)
```

[Read more about this command &rarr;](line/)

## [point](point/) ##

{% apibody %}
r.point(longitude, latitude) &rarr; point
{% endapibody %}

Construct a geometry object of type Point. The point is specified by two floating point numbers, the latitude (&minus;90 to 90) and longitude (&minus;180 to 180) of the point on a perfect sphere.

__Example:__ Define a point.

```rb
r.table('geo').insert({
    :id => 1,
    :name => 'San Francisco',
    :location => r.point(-122.423246,37.779388)
}).run(conn)
```

[Read more about this command &rarr;](point/)

## [polygon](polygon/) ##

{% apibody %}
r.polygon([lat1, lon2], [lat2, lon2], ...) &rarr; polygon
r.polygon(point1, point2, ...) &rarr; polygon
{% endapibody %}

Construct a geometry object of type Polygon. The Polygon can be specified in one of two ways:

* Three or more two-item arrays, specifying latitude and longitude numbers of the polygon's vertices;
* Three or more [Point](/api/ruby/point) objects specifying the polygon's vertices.

__Example:__ Define a polygon.

```rb
r.table('geo').insert({
    :id => 101,
    :rectangle => r.polygon(
        [37.779388,-122.423246],
        [37.329898,-122.423246],
        [37.329898,-121.886420],
        [37.779388,-121.886420]
    )
}).run(conn)
```

[Read more about this command &rarr;](polygon/)

## [polygon_sub](polygon_sub/) ##

{% apibody %}
polygon1.polygon_sub(polygon2) &rarr; polygon
{% endapibody %}

Use `polygon2` to "punch out" a hole in `polygon1`. `polygon2` must be completely contained within `polygon1` and must have no holes itself (it must not be the output of `polygon_sub` itself).


__Example:__ Define a polygon with a hole punched in it.

```rb
outer_polygon = r.polygon(
    [37.7,-122.4],
    [37.3,-122.4],
    [37.3,-121.8],
    [37.7,-121.8]
)
inner_polygon = r.polygon(
    [37.4,-122.3],
    [37.6,-122.3],
    [37.6,-122.0],
    [37.4,-122.0]
)
outer_polygon.polygon_sub(inner_polygon).run(conn)
```

[Read more about this command &rarr;](polygon_sub/)

{% endapisection %}

{% apisection Administration %}

## [config](config/) ##

{% apibody %}
table.config() &rarr; selection&lt;object&gt;
database.config() &rarr; selection&lt;object&gt;
{% endapibody %}

Query (read and/or update) the configurations for individual tables or databases.

__Example:__ Get the configuration for the `users` table.

```rb
> r.table('users').config().run(conn)
```

[Read more about this command &rarr;](config/)

## [rebalance](rebalance/) ##

{% apibody %}
table.rebalance() &rarr; object
database.rebalance() &rarr; object
{% endapibody %}

Rebalances the shards of a table. When called on a database, all the tables in that database will be rebalanced.

__Example:__ Rebalance a table.

```rb
> r.table('superheroes').rebalance().run(conn)
```

[Read more about this command &rarr;](rebalance/)

## [reconfigure](reconfigure/) ##

{% apibody %}
table.reconfigure({:shards => <s>, :replicas => <r>[, :primary_replica_tag => <t>, :dry_run => false]}) &rarr; object
database.reconfigure({:shards => <s>, :replicas => <r>[, :primary_replica_tag => <t>, :dry_run => false]}) &rarr; object
{% endapibody %}

Reconfigure a table's sharding and replication.

__Example:__ Reconfigure a table.

```rb
> r.table('superheroes').reconfigure({:shards => 2, :replicas => 1}).run(conn)
```

[Read more about this command &rarr;](reconfigure/)

## [status](status/) ##

{% apibody %}
table.status() &rarr; selection&lt;object&gt;
{% endapibody %}

Return the status of a table.

__Example:__ Get a table's status.

```rb
> r.table('superheroes').status().run(conn)
```

[Read more about this command &rarr;](status/)

## [wait](wait/) ##

{% apibody %}
table.wait([{:wait_for => 'ready_for_writes', :timeout => <sec>}]) &rarr; object
database.wait([{:wait_for => 'ready_for_writes', :timeout => <sec>}]) &rarr; object
r.wait([{:wait_for => 'ready_for_writes', :timeout => <sec>}]) &rarr; object
{% endapibody %}

Wait for a table or all the tables in a database to be ready. A table may be temporarily unavailable after creation, rebalancing or reconfiguring. The `wait` command blocks until the given table (or database) is fully up to date.

__Example:__ Wait for a table to be ready.

```rb
> r.table('superheroes').wait().run(conn)
```

[Read more about this command &rarr;](wait/)

{% endapisection %}
