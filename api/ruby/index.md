---
layout: api
title: "ReQL command reference"
active: api
no_footer: true
permalink: api/ruby/
language: Ruby
---


{% apisection Accessing RQL%}
All RQL queries begin from the top level module.

## [r](r/) ##

{% apibody %}
r &rarr; r
{% endapibody %}

The top-level RQL namespace.

__Example:__ Setup your top level namespace.

```rb
require 'rethinkdb'
include RethinkDB::Shortcuts
```


## [connect](connect/) ##

{% apibody %}
r.connect(opts) &rarr; connection
{% endapibody %}

Create a new connection to the database server.

If the connection cannot be established, a `RqlDriverError` exception will be thrown

__Example:__ Opens a connection using the default host and port but specifying the default database.



```rb
conn = r.connect(:host => 'localhost', :port => 28015, :db => 'heroes', :auth_key => 'hunter2')
```

## [repl](repl/) ##

{% apibody %}
connection.repl
{% endapibody %}

Set the default connection to make REPL use easier. Allows calling run() without specifying a connection. 

Connection objects are not thread safe and repl connections should not be used in multi-threaded environments.

__Example:__ Set the default connection in REPL, and call `run()` without specifying the connection.

```rb
r.connect().repl
r.table('users').run
```


## [close](close/) ##

{% apibody %}
connection.close
{% endapibody %}

Close an open connection. Closing a connection cancels all outstanding requests and frees
the memory associated with the open requests.

__Example:__ Close an open connection.

```rb
conn.close
```


## [reconnect](reconnect/) ##

{% apibody %}
connection.reconnect
{% endapibody %}

Close and attempt to reopen a connection. Has the effect of canceling any outstanding
request while keeping the connection open.

__Example:__ Cancel outstanding requests/queries that are no longer needed.

```rb
conn.reconnect
```


## [use](use/) ##

{% apibody %}
connection.use(db_name)
{% endapibody %}

Change the default database on this connection.

__Example:__ Change the default database so that we don't need to specify the database
when referencing a table.

```rb
conn.use('heroes')
```


## [run](run/) ##

{% apibody %}
query.run(conn[, opts]) &rarr; cursor
{% endapibody %}

Run a query on a connection.

__Example:__ Call run on the connection with a query to execute the query.

```rb
r.table('marvel').run(conn).each{|x| p x}
```

[Read more about this command &rarr;](run/)

{% endapisection %}

{% apisection Manipulating databases%}
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




{% apisection Manipulating tables%}
## [table_create](table_create/) ##

{% apibody %}
db.table_create(table_name[, options]) &rarr; object
{% endapibody %}

Create a table. A RethinkDB table is a collection of JSON documents. 

If successful, the operation returns an object: `{created: 1}`. If a table with the same
name already exists, the operation throws `RqlRuntimeError`.
Note: that you can only use alphanumeric characters and underscores for the table name.

When creating a table you can specify the following options:

- `primary_key`: the name of the primary key. The default primary key is id;
- `durability`: if set to `soft`, this enables _soft durability_ on this table:
writes will be acknowledged by the server immediately and flushed to disk in the
background. Default is `hard` (acknowledgement of writes happens after data has been
written to disk);
- `cache_size`: set the cache size (in bytes) to be used by the table. The
default is 1073741824 (1024MB);
- `datacenter`: the name of the datacenter this table should be assigned to.


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

If succesful, the operation returns an object: {"dropped": 1}. If the specified table
doesn't exist a `RqlRuntimeError` is thrown.

__Example:__ Drop a table named 'dc_universe'.

```rb
r.db('test').table_drop('dc_universe').run(conn)
```


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
table.index_create(index_name[, index_function]) &rarr; object
{% endapibody %}

Create a new secondary index on this table.

__Example:__ To efficiently query our heros by code name we have to create a secondary
index.

```rb
r.table('dc').index_create('code_name').run(conn)
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


{% endapisection %}


{% apisection Writing data%}

## [insert](insert/) ##

{% apibody %}
table.insert(json | [json]
    [, { :durability => 'soft', :return_vals => true :upsert => true}])
        &rarr; object
{% endapibody %}

Insert JSON documents into a table. Accepts a single JSON document or an array of
documents.

Insert returns an object that contains the following attributes:

- `inserted`: the number of documents that were succesfully inserted
- `replaced`: the number of documents that were updated when upsert is used
- `unchanged`: the number of documents that would have been modified, except that the
new value was the same as the old value when doing an upsert
- `errors`: the number of errors encountered while inserting; if errors where
encountered while inserting, `first_error` contains the text of the first error
- `generated_keys`: a list of generated primary key values
- `deleted` and `skipped`: 0 for an insert operation.

__Example:__ Insert a row into a table named 'marvel'.

```rb
r.table('marvel').insert(
    { :superhero => 'Iron Man', :superpower => 'Arc Reactor' }).run(conn)
```

[Read more about this command &rarr;](insert/)


## [update](update/) ##

{% apibody %}
table.update(json | expr[, durability => 'soft', return_vals => true])
    &rarr; object
selection.update(json | expr[, durability => 'soft', return_vals => true])
    &rarr; object
singleSelection.update(json | expr[, durability => 'soft', return_vals => true])
    &rarr; object
{% endapibody %}

Update JSON documents in a table. Accepts a JSON document, a RQL expression, or a
combination of the two. You can pass options like `returnVals` that will return the old
and new values of the row you have modified. 

Update returns an object that contains the following attributes:

- `replaced`: the number of documents that were updated
- `unchanged`: the number of documents that would have been modified except the new
value was the same as the old value;
- `skipped`: the number of documents that were left unmodified because there was nothing
to do: either the row didn't exist or the new value is null;
- `errors`: the number of errors encountered while performing the update; if errors
occured, first_error contains the text of the first error;
- `deleted` and `inserted`: 0 for an update operation.

__Example:__ Update Superman's age to 30. If attribute 'age' doesn't exist, adds it to
the document.

```rb
r.table('marvel').get('superman').update{ {:age => 30} }.run(conn)
```

[Read more about this command &rarr;](update/)


## [replace](replace/) ##

{% apibody %}
table.replace(json | expr[, durability => 'soft', return_vals => true])
    &rarr; object
selection.replace(json | expr[, durability => 'soft', return_vals => true])
    &rarr; object
singleSelection.replace(json | expr[, durability => 'soft', return_vals => true])
    &rarr; object
{% endapibody %}

Replace documents in a table. Accepts a JSON document or a RQL expression, and replaces
the original document with the new one. The new document must have the same primary key
as the original document. The optional argument durability with value 'hard' or 'soft'
will override the table or query's default durability setting. The optional argument
`return_vals` will return the old and new values of the row you're modifying when set to
true (only valid for single-row replacements). The optional argument `non_atomic` lets you
permit non-atomic updates.

Replace returns an object that contains the following attributes:

- `replaced`: the number of documents that were replaced
- `unchanged`: the number of documents that would have been modified, except that the
new value was the same as the old value
- `inserted`: the number of new documents added. You can have new documents inserted if
you do a point-replace on a key that isn't in the table or you do a replace on a
selection and one of the documents you are replacing has been deleted
- `deleted`: the number of deleted documents when doing a replace with null
- `errors`: the number of errors encountered while performing the replace; if errors
occurred performing the replace, first_error contains the text of the first error encountered
- `skipped`: 0 for a replace operation


__Example:__ Remove all existing attributes from Superman's document, and add an attribute 'age'.

```rb
r.table('marvel').get('superman').replace({
    :id => 'superman',
    :age => 30
}).run(conn)
```

[Read more about this command &rarr;](replace/)


## [delete](delete/) ##

{% apibody %}
table.delete[({:durability => soft, :return_vals => true})]
    &rarr; object
selection.delete[({:durability => soft, :return_vals => true})]
    &rarr; object
singleSelection.delete[({:durability => soft, :return_vals => true})]
    &rarr; object
{% endapibody %}

Delete one or more documents from a table. The optional argument return_vals will return
the old value of the row you're deleting when set to true (only valid for single-row
deletes). The optional argument durability with value 'hard' or 'soft' will override the
table or query's default durability setting.

Delete returns an object that contains the following attributes:

- `deleted`: the number of documents that were deleted
- `skipped`: the number of documents from the selection that were left unmodified because
there was nothing to do. For example, if you delete a row that has already been deleted,
that row will be skipped
- `errors`: the number of errors encountered while deleting
if errors occured, first_error contains the text of the first error
- `inserted`, `replaced`, and `unchanged`: all 0 for a delete operation.


__Example:__ Delete superman from the database.

```rb
r.table('marvel').get('superman').delete.run(conn)
```

[Read more about this command &rarr;](delete/)

{% endapisection %}


{% apisection Selecting data%}

## [db](db/) ##

{% apibody %}
r.db(db_name) &rarr; db
{% endapibody %}

Reference a database.

__Example:__ Before we can query a table we have to select the correct database.

```rb
r.db('heroes').table('marvel').run(conn)
```


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

__Example:__ Find a document with the primary key 'superman'.

```rb
r.table('marvel').get('superman').run(conn)
```


## [get_all](get_all/) ##

{% apibody %}
table.get_all(key[, key2...], [, :index => 'id']) &rarr; selection
{% endapibody %}

Get all documents where the given value matches the value of the requested index.

__Example:__ Secondary index keys are not guaranteed to be unique so we cannot query via
"get" when using a secondary index.

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
sequence.filter(predicate[, :default => false]) &rarr; selection
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


{% apisection Joins%}
These commands allow the combination of multiple sequences into a single sequence

## [inner_join](inner_join/) ##

{% apibody %}
sequence.inner_join(other_sequence, predicate) &rarr; stream
array.inner_join(other_sequence, predicate) &rarr; array
{% endapibody %}

Returns the inner product of two sequences (e.g. a table, a filter result) filtered by
the predicate. The query compares each row of the left sequence with each row of the
right sequence to find all pairs of rows which satisfy the predicate. When the predicate
is satisfied, each matched pair of rows of both sequences are combined into a result row.

__Example:__ Construct a sequence of documents containing all cross-universe matchups where a marvel hero would lose.

```rb
r.table('marvel').inner_join(r.table('dc')) {|marvel_row, dc_row|
    marvel_row[:strength] < dc_row[:strength]
}.run(conn)
```


## [outer_join](outer_join/) ##

{% apibody %}
sequence.outer_join(other_sequence, predicate) &rarr; stream
array.outer_join(other_sequence, predicate) &rarr; array
{% endapibody %}

Computes a left outer join by retaining each row in the left table even if no match was
found in the right table.

__Example:__ Construct a sequence of documents containing all cross-universe matchups
where a marvel hero would lose, but keep marvel heroes who would never lose a matchup in
the sequence.

```rb
r.table('marvel').outer_join(r.table('dc')) {|marvel_row, dc_row|
    marvel_row[:strength] < dc_row[:strength]
}.run(conn)
```


## [eq_join](eq_join/) ##

{% apibody %}
sequence.eq_join(left_attr, other_table[, index='id']) &rarr; stream
array.eq_join(left_attr, other_table[, index='id']) &rarr; array
{% endapibody %}

An efficient join that looks up elements in the right table by primary key.

__Example:__ Let our heroes join forces to battle evil!

```rb
r.table('marvel').eq_join(:main_dc_collaborator, r.table('dc')).run(conn)
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

{% apisection Transformations%}
These commands are used to transform data in a sequence.

## [map](map/) ##

{% apibody %}
sequence.map(mapping_function) &rarr; stream
array.map(mapping_function) &rarr; array
{% endapibody %}

Transform each element of the sequence by applying the given mapping function.

__Example:__ Construct a sequence of hero power ratings.

```rb
r.table('marvel').map {|hero|
    hero[:combat_power] + hero[:compassion_power] * 2
}.run(conn)
```


## [with_fields](with_fields/) ##

{% apibody %}
sequence.with_selectors([selector1, selector2...]) &rarr; stream
array.with_selectors([selector1, selector2...]) &rarr; array
{% endapibody %}

Takes a sequence of objects and a list of fields. If any objects in the sequence don't
have all of the specified fields, they're dropped from the sequence. The remaining
objects have the specified fields plucked out. (This is identical to `has_fields`
followed by `pluck` on a sequence.)

__Example:__ Get a list of heroes and their nemeses, excluding any heroes that lack one.

```rb
r.table('marvel').with_fields('id', 'nemesis')
```

[Read more about this command &rarr;](with_fields/)


## [concat_map](concat_map/) ##

{% apibody %}
sequence.concat_map(mapping_function) &rarr; stream
array.concat_map(mapping_function) &rarr; array
{% endapibody %}

Flattens a sequence of arrays returned by the mappingFunction into a single sequence.

__Example:__ Construct a sequence of all monsters defeated by Marvel heroes. Here the field
'defeatedMonsters' is a list that is concatenated to the sequence.

```rb
r.table('marvel').concat_map {|hero|
    hero[:defeated_monsters]
}.run(conn)

```


## [order_by](order_by/) ##

{% apibody %}
sequence.order_by(key1, [key2...]) &rarr; stream
array.order_by(key1, [key2...]) &rarr; array
{% endapibody %}

Sort the sequence by document values of the given key(s). `orderBy` defaults to ascending
ordering. To explicitly specify the ordering, wrap the attribute with either `r.asc` or
`r.desc`.

__Example:__ Order our heroes by a series of performance metrics.

```rb
r.table('marvel').order_by(:enemies_vanquished, :damsels_saved).run(conn)
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

## [\[\]](slice/) ##

{% apibody %}
sequence[start_index[, end_index]] &rarr; stream
array[start_index[, end_index]] &rarr; array
{% endapibody %}

Trim the sequence to within the bounds provided.

__Example:__ For this fight, we need heroes with a good mix of strength and agility.

```rb
r.table('marvel').order_by(:strength)[5..10].run(conn)
```

## [\[\]](nth/) ##

{% apibody %}
sequence[index] &rarr; object
{% endapibody %}

Get the nth element of a sequence.

__Example:__ Select the second element in the array.

```rb
r.expr([1,2,3])[1].run(conn)
```


## [indexes_of](indexes_of/) ##

{% apibody %}
sequence.indexes_of(datum | predicate) &rarr; array
{% endapibody %}

Get the indexes of an element in a sequence. If the argument is a predicate, get the indexes of all elements matching it.

__Example:__ Find the position of the letter 'c'.

```rb
r.expr(['a','b','c']).indexes_of('c').run(conn)
```

[Read more about this command &rarr;](indexes_of/)



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
sequence.union(sequence) &rarr; array
{% endapibody %}

Concatenate two sequences.

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


{% apisection Aggregation%}
These commands are used to compute smaller values from large sequences.

## [reduce](reduce/) ##

{% apibody %}
sequence.reduce(reduction_function[, base]) &rarr; value
{% endapibody %}

Produce a single value from a sequence through repeated application of a reduction
function.

The reduce function gets invoked repeatedly not only for the input values but also for
results of previous reduce invocations. The type and format of the object that is passed
in to reduce must be the same with the one returned from reduce.

__Example:__ How many enemies have our heroes defeated?

```rb
r.table('marvel').order_by(:strength)[5..10].run(conn)
```


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

## [distinct](distinct/) ##

{% apibody %}
sequence.distinct() &rarr; array
{% endapibody %}

Remove duplicate elements from the sequence.

__Example:__ Which unique villains have been vanquished by marvel heroes?

```rb
r.table('marvel').concat_map{|hero| hero[:villain_list]}.distinct.run(conn)
```


## [grouped\_map\_reduce](grouped_map_reduce) ##

{% apibody %}
sequence.grouped_map_reduce(grouping, mapping, reduction, base)
    &rarr; value
{% endapibody %}

Partition the sequence into groups based on the `grouping` function. The elements of each
group are then mapped using the `mapping` function and reduced using the `reduction`
function.

`grouped_map_reduce` is a generalized form of group by.

__Example:__ It's only fair that heroes be compared against their weight class.

```rb
r.table('marvel').grouped_map_reduce(
    lambda {|hero| hero[:weight_class]},  # grouping
    lambda {|hero| hero.pluck(:name, :strength)},  #  mapping
    {:name => 'none', :strength => 0},  # reduction base
    lambda {|acc, hero| r.branch(acc[:strength] < hero[:strength], hero, acc)}
).run(conn)
```


## [group_by](group_by/) ##

{% apibody %}
sequence.group_by(selector1[, selector2...], reduction_object) &rarr; array
{% endapibody %}

Groups elements by the values of the given attributes and then applies the given
reduction. Though similar to `groupedMapReduce`, `groupBy` takes a standardized object
for specifying the reduction. Can be used with a number of predefined common reductions.

__Example:__ Using a predefined reduction we can easily find the average strength of members of each weight class.

```rb
r.table('marvel').group_by(:weight_class, r.avg(:strength)).run(conn)
```

[Read more about this command &rarr;](group_by/)


## [contains](contains/) ##

{% apibody %}
sequence.contains(value1[, value2...]) &rarr; bool
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


{% apisection Aggregators%}
These standard aggregator objects are to be used in conjunction with groupBy.

## [count]](count-aggregator/) ##

{% apibody %}
r.count
{% endapibody %}

Count the total size of the group.

__Example:__ Just how many heroes do we have at each strength level?

```rb
r.table('marvel').group_by(:strength, r.count).run(conn)
```


## [sum](sum/) ##

{% apibody %}
r.sum(attr)
{% endapibody %}

Compute the sum of the given field in the group.

__Example:__ How many enemies have been vanquished by heroes at each strength level?

```rb
r.table('marvel').group_by(:strength, r.sum(:enemies_vanquished)).run(conn)
```


## [avg](avg/) ##

{% apibody %}
r.avg(attr)
{% endapibody %}

Compute the average value of the given attribute for the group.

__Example:__ What's the average agility of heroes at each strength level?

```rb
r.table('marvel').group_by(:strength, r.avg(:agility)).run(conn)
```



{% endapisection %}


{% apisection Document manipulation%}

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
singleSelection.merge(object) &rarr; object
object.merge(object) &rarr; object
sequence.merge(object) &rarr; stream
array.merge(object) &rarr; array
{% endapibody %}

Merge two objects together to construct a new object with properties from both. Gives preference to attributes from other when there is a conflict.

__Example:__ Equip IronMan for battle.

```rb
r.table('marvel').get('IronMan').merge(
    r.table('loadouts').get('alienInvasionKit')).run(conn)
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


## [\[\]](get_field/) ##

{% apibody %}
sequence[attr] &rarr; sequence
singleSelection[attr] &rarr; value
object[attr] &rarr; value
{% endapibody %}

Get a single field from an object. If called on a sequence, gets that field from every
object in the sequence, skipping objects that lack it.

__Example:__ What was Iron Man's first appearance in a comic?

```rb
r.table('marvel').get('IronMan')[:first_appearance].run(conn)
```


## [has_fields](has_fields/) ##

{% apibody %}
sequence.has_fields([selector1, selector2...]) &rarr; stream
array.has_fields([selector1, selector2...]) &rarr; array
singleSelection.has_fields([selector1, selector2...]) &rarr; boolean
object.has_fields([selector1, selector2...]) &rarr; boolean
{% endapibody %}

Test if an object has all of the specified fields. An object has a field if it has the
specified key and that key maps to a non-null value. For instance, the object
`{'a':1,'b':2,'c':null}` has the fields `a` and `b`.

__Example:__ Which heroes are married?

```rb
r.table('marvel').has_fields(:spouse).run(conn)
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

Remove an element from an array at a given index. Returns the modified array.

__Example:__ Hulk decides to leave the avengers.

```rb
r.expr(["Iron Man", "Hulk", "Spider-Man"]).delete_at(1).run(conn)
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


{% endapisection %}


{% apisection String manipulation%}
These commands provide string operators.

## [match](match/) ##

{% apibody %}
string.match(regexp) &rarr; array
{% endapibody %}

Match against a regular expression. Returns a match object containing the matched string,
that string's start/end position, and the capture groups. Accepts RE2 syntax
([https://code.google.com/p/re2/wiki/Syntax](https://code.google.com/p/re2/wiki/Syntax)).
You can enable case-insensitive matching by prefixing the regular expression with
`(?i)`. (See linked RE2 documentation for more flags.)

__Example:__ Get all users whose name starts with A.

```rb
r.table('users').filter{|row| row[:name].match("^A")}.run(conn)
```

[Read more about this command &rarr;](match/)

{% endapisection %}


{% apisection Math and logic%}

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

## [&](and/) ##

{% apibody %}
bool & bool &rarr; bool
{% endapibody %}

Compute the logical and of two values.

__Example:__ True and false anded is false?

```rb
(r.expr(True) & False).run(conn)
```


## [|](or/) ##

{% apibody %}
bool | bool &rarr; bool
{% endapibody %}

Compute the logical or of two values.

__Example:__ True or false ored is true?

```rb
(r.expr(True) | False).run(conn)
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


## [>](gt/) ##

{% apibody %}
value > value &rarr; bool
{% endapibody %}

Test if the first value is greater than other.

__Example:__ Is 2 greater than 2?

```rb
(r.expr(2) > 2).run(conn)
```

## [>=](ge/) ##

{% apibody %}
value >= value &rarr; bool
{% endapibody %}

Test if the first value is greater than or equal to other.

__Example:__ Is 2 greater than or equal to 2?

```rb
(r.expr(2) >= 2).run(conn)
```

## [<](lt/) ##

{% apibody %}
value < value &rarr; bool
{% endapibody %}

Test if the first value is less than other.

__Example:__ Is 2 less than 2?

```rb
(r.expr(2) < 2).run(conn)
```

## [<=](le/) ##

{% apibody %}
value <= value &rarr; bool
{% endapibody %}

Test if the first value is less than or equal to other.

__Example:__ Is 2 less than or equal to 2?

```rb
(r.expr(2) <= 2).run(conn)
```


## [not](not/) ##

{% apibody %}
bool.not() &rarr; bool
{% endapibody %}
Compute the logical inverse (not).

__Example:__ Not true is false.

```rb
r(true).not.run(conn)
```


{% endapisection %}


{% apisection Dates and times%}

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
- `seconds` is a float. Its value will be rounded to three decimal places
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

Create a time object based on seconds since epoch. The first argument is a float and
will be rounded to three decimal places (millisecond-precision).

__Example:__ Update the birthdate of the user "John" to November 3rd, 1986.

```rb
r.table("user").get("John").update(:birthdate => r.epoch_time(531360000)).run(conn)
```


## [iso8601](iso8601/) ##

{% apibody %}
r.ISO8601(iso8601Date[, {default_timezone:''}]) &rarr; time
{% endapibody %}

Create a time object based on an iso8601 date-time string (e.g.
'2013-01-01T01:01:01+00:00'). We support all valid ISO 8601 formats except for week
dates. If you pass an ISO 8601 date-time without a time zone, you must specify the time
zone with the optarg `default_timezone`. Read more about the ISO 8601 format on the
Wikipedia page.

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
time.to_iso8601() &rarr; number
{% endapibody %}

Convert a time object to its iso 8601 format.

__Example:__ Return the current time in an ISO8601 format.

```rb
r.now().to_iso8601()
```


## [to\_epoch\_time](to_epoch_time/) ##

{% apibody %}
time.to_epoch_time() &rarr; number
{% endapibody %}

Convert a time object to its epoch time.

__Example:__ Return the current time in an ISO8601 format.

```rb
r.now().to_epoch_time()
```



{% endapisection %}


{% apisection Control structures%}

## [do](do/) ##

{% apibody %}
any.do(arg [, args]*, expr) &rarr; any
{% endapibody %}

Evaluate the expr in the context of one or more value bindings.

The type of the result is the type of the value returned from expr.

__Example:__ The object(s) passed to do() can be bound to name(s). The last argument is the expression to evaluate in the context of the bindings.

```rb
r.do(r.table('marvel').get('IronMan')) { |ironman| ironman[:name] }.run(conn)
```


## [branch](branch/) ##

{% apibody %}
r.branch(test, true_branch, false_branch) &rarr; any
{% endapibody %}

Evaluate one of two control paths based on the value of an expression. branch is effectively an if renamed due to language constraints.

The type of the result is determined by the type of the branch that gets executed.

__Example:__ Return the manlier of two heroes:

```rb
r.table('marvel').map { |row|  r.branch(row[:victories] > 100,
    row[:name] + ' is a superhero',
    row[:name] + ' is a hero')
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
argument returns null, returns its second argument. (Alternatively, the second argument
may be a function which will be called with either the text of the non-existence error
or null.)

__Example:__ Stark Industries made the mistake of trusting an intern with data entry,
and now a bunch of fields are missing from some of their documents. Iron Man takes a
break from fighting Mandarin to write some safe analytics queries.

```rb
r.table('projects').map {|p|
    p[:staff].default(0) + p[:management].default(0)
}.run(conn)
```


## [expr](expr/) ##

{% apibody %}
r.expr(value) &rarr; value
{% endapibody %}

Construct a RQL JSON object from a native object.

__Example:__ Objects wrapped with expr can then be manipulated by RQL API functions.

```rb
r.expr({:a => 'b'}).merge({:b => [1,2,3]}).run(conn)
```

[Read more about this command &rarr;](expr/)

## [js](js/) ##

{% apibody %}
r.js(jsString) &rarr; value
{% endapibody %}

Create a javascript expression.

__Example:__ Concatenate two strings using Javascript'

```rb
r.js("'str1' + 'str2'").run(conn)
```

[Read more about this command &rarr;](js/)

## [coerce_to](coerce_to/) ##

{% apibody %}
sequence.coerce_to(type_name) &rarr; array
value.coerce_to(type_name) &rarr; string
array.coerce_to(type_name) &rarr; object
object.coerce_to(type_name) &rarr; array
{% endapibody %}

Converts a value of one type into another. 

You can convert: a selection, sequence, or object into an ARRAY, an array of pairs into an OBJECT, and any DATUM into a STRING.

__Example:__ Convert a table to an array.

```rb
r.table('marvel').coerce_to('array').run(conn)
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

Get information about a RQL value.

__Example:__ Get information about a table such as primary key, or cache size.

```rb
r.table('marvel').info().run(conn)
```

## [json](json/) ##

{% apibody %}
r.json(json_string) &rarr; value
{% endapibody %}

Parse a JSON string on the server.

__Example:__ Send an array to the server'

```rb
r.json("[1,2,3]").run(conn)
```


{% endapisection %}

