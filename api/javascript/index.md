---
layout: api
title: "ReQL command reference"
active: api
no_footer: true
permalink: api/javascript/
language: JavaScript
---


{% apisection Accessing RQL%}
All RQL queries begin from the top level module.

## [r](r/) ##

{% apibody %}
r &rarr; r
{% endapibody %}

The top-level RQL namespace.

__Example:__ Setup your top level namespace.

```js
var r = require('rethinkdb');
```


## [connect](connect/) ##

{% apibody %}
r.connect(opts, callback)
{% endapibody %}

Create a new connection to the database server.

If the connection cannot be established, a `RqlDriverError` exception will be thrown

__Example:__ Opens a new connection to the database.

```js
r.connect({host:'localhost', port:28015, db:'marvel', authKey:'hunter2'},
   function(err, conn) { ... })
```


## [close](close/) ##

{% apibody %}
conn.close()
{% endapibody %}

Close an open connection. Closing a connection cancels all outstanding requests and frees
the memory associated with the open requests.

__Example:__ Close an open connection.

```js
conn.close()
```


## [reconnect](reconnect/) ##

{% apibody %}
conn.reconnect()
{% endapibody %}

Close and attempt to reopen a connection. Has the effect of canceling any outstanding
request while keeping the connection open.

__Example:__ Cancel outstanding requests/queries that are no longer needed.

```js
conn.reconnect(function(errror, connection) { ... })
```


## [use](use/) ##

{% apibody %}
conn.use(dbName)
{% endapibody %}

Change the default database on this connection.

__Example:__ Change the default database so that we don't need to specify the database
when referencing a table.

```js
conn.use('heroes')
```


## [run](run/) ##

{% apibody %}
query.run(connection, callback) 
query.run(options[, callback])
{% endapibody %}

Run a query on a connection.

__Example:__ Call run on the connection with a query to execute the query. The callback
will get a cursor from which results may be retrieved.

```js
r.table('marvel').run(conn, function(err, cur) { cur.each(console.log); })
```

[Read more about this command &rarr;](run/)

## [next](next/) ##

{% apibody %}
cursor.next(callback)
{% endapibody %}

Get the next element in the cursor.

__Example:__ Let's grab the next element!

```js
cur.next(function(err, row) {
    return processRow(row);
});
```

## [hasNext](hasNext/) ##

{% apibody %}
cursor.hasNext() &rarr; bool
{% endapibody %}

Check if there are more elements in the cursor.

__Example:__ Are there more elements in the cursor?

var hasMore = cur.hasNext();


## [each](each/) ##

{% apibody %}
cursor.each(callback[, onFinishedCallback])
{% endapibody %}

Lazily iterate over the result set one element at a time.

__Example:__ Let's process all the elements!

```js
cur.each(function(err, row) {
    processRow(row);
});
```

[Read more about this command &rarr;](each/)

## [toArray](toArray/) ##

{% apibody %}
cursor.toArray(callback)
{% endapibody %}

Retrieve all results and pass them as an array to the given callback.

__Example:__ For small result sets it may be more convenient to process them at once as
an array.

```js
cur.toArray(function(err, results) {
    for(var i in results) {
        processRow(results[i]);
    }
});
```


## [close (cursor)](close-cursor/) ##

{% apibody %}
cursor.close()
{% endapibody %}


Close a cursor. Closing a cursor cancels the corresponding query and frees the memory
associated with the open request.

__Example:__ Close a cursor.

```js
cursor.close()
```


## [addListener](add_listener/) ##

{% apibody %}
connection.addListener(event, listener)
{% endapibody %}

The connection object also supports the event emitter interface so you can listen for
changes in connection state.

__Example:__ Monitor connection state with events 'connect', 'close', and 'error'.


```js
r.connect({}, function(err, conn) {
    if (err) throw err;

    conn.addListener('error', function(e) {
        processNetworkError(e);
    });

    conn.addListener('close', function() {
        cleanup();
    });

    runQueries(conn);
});

```

[Read more about this command &rarr;](add_listener/)


{% endapisection %}

{% apisection Manipulating databases%}
## [dbCreate](db_create/) ##

{% apibody %}
r.dbCreate(dbName) &rarr; object
{% endapibody %}

Create a database. A RethinkDB database is a collection of tables, similar to
relational databases.

If successful, the operation returns an object: `{created: 1}`. If a database with the
same name already exists the operation throws RqlRuntimeError.
Note: that you can only use alphanumeric characters and underscores for the database name.

__Example:__ Create a database named 'superheroes'.

```js
r.dbCreate('superheroes').run(conn, callback)
```


## [dbDrop](db_drop/) ##

{% apibody %}
r.dbDrop(dbName) &rarr; object
{% endapibody %}

Drop a database. The database, all its tables, and corresponding data will be deleted.

If successful, the operation returns the object `{dropped: 1}`. If the specified database
doesn't exist a RqlRuntimeError is thrown.

__Example:__ Drop a database named 'superheroes'.

```js
r.dbDrop('superheroes').run(conn, callback)
```


## [dbList](db_list/) ##

{% apibody %}
r.dbList() &rarr; array
{% endapibody %}

List all database names in the system. The result is a list of strings.

__Example:__ List all databases.

```js
r.dbList().run(conn, callback)
```

{% endapisection %}




{% apisection Manipulating tables%}
## [tableCreate](table_create/) ##

{% apibody %}
db.tableCreate(tableName[, options]) &rarr; object
{% endapibody %}

Create a table. A RethinkDB table is a collection of JSON documents. 

If successful, the operation returns an object: `{created: 1}`. If a table with the same
name already exists, the operation throws `RqlRuntimeError`.
Note: that you can only use alphanumeric characters and underscores for the table name.

When creating a table you can specify the following options:

- `primaryKey`: the name of the primary key. The default primary key is id;
- `durability`: if set to `soft`, this enables _soft durability_ on this table:
writes will be acknowledged by the server immediately and flushed to disk in the
background. Default is `hard` (acknowledgement of writes happens after data has been
written to disk);
- `cacheSize`: set the cache size (in bytes) to be used by the table. The
default is 1073741824 (1024MB);
- `datacenter`: the name of the datacenter this table should be assigned to.


__Example:__ Create a table named 'dc_universe' with the default settings.

```js
r.db('test').tableCreate('dc_universe').run(conn, callback)
```

[Read more about this command &rarr;](table_create/)

## [tableDrop](table_drop/) ##

{% apibody %}
db.tableDrop(tableName) &rarr; object
{% endapibody %}

Drop a table. The table and all its data will be deleted.

If succesful, the operation returns an object: {dropped: 1}. If the specified table
doesn't exist a `RqlRuntimeError` is thrown.

__Example:__ Drop a table named 'dc_universe'.

```js
r.db('test').tableDrop('dc_universe').run(conn, callback)
```


## [tableList](table_list/) ##

{% apibody %}
db.tableList() &rarr; array
{% endapibody %}

List all table names in a database. The result is a list of strings.

__Example:__ List all tables of the 'test' database.

```js
r.db('test').tableList().run(conn, callback)
```


## [indexCreate](index_create/) ##

{% apibody %}
table.indexCreate(indexName[, indexFunction]) &rarr; object
{% endapibody %}

Create a new secondary index on this table.

__Example:__ To efficiently query our heros by code name we have to create a secondary
index.

```js
r.table('dc').indexCreate('code_name').run(conn, callback)
```

[Read more about this command &rarr;](index_create/)


## [indexDrop](index_drop/) ##

{% apibody %}
table.indexDrop(indexName) &rarr; object
{% endapibody %}

Delete a previously created secondary index of this table.

__Example:__ Drop a secondary index named 'code_name'.

```js
r.table('dc').indexDrop('code_name').run(conn, callback)
```


## [indexList](index_list/) ##

{% apibody %}
table.indexList() &rarr; array
{% endapibody %}

List all the secondary indexes of this table.

__Example:__ List the available secondary indexes for this table.

```js
r.table('marvel').indexList().run(conn, callback)
```



{% endapisection %}


{% apisection Writing data%}

## [insert](insert/) ##

{% apibody %}
table.insert(json | [json]
    [, {durability: 'soft', returnVals: true, upsert:true}])
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

```js
r.table('marvel').insert({ superhero: 'Iron Man', superpower: 'Arc Reactor' })
    .run(conn, callback)
```

[Read more about this command &rarr;](insert/)

## [update](update/) ##

{% apibody %}
table.update(json | expr[, {durability: 'soft', return_vals: true])
    &rarr; object
selection.update(json | expr[, {durability: 'soft', return_vals: true])
    &rarr; object
singleSelection.update(json | expr[, {durability: 'soft', return_vals: true])
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

```js
r.table('marvel').get('superman').update({ age: 30 }).run(conn, callback)
```

[Read more about this command &rarr;](update/)


## [replace](replace/) ##

{% apibody %}
table.replace(json | expr[, {durability: 'soft', return_vals: true}])
    &rarr; object
selection.replace(json | expr[, {durability: 'soft', return_vals: true}])
    &rarr; object
singleSelection.replace(json | expr
    [, {durability: 'soft', return_vals: true}])
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

```js
r.table('marvel').get('superman').replace({ id: 'superman', age: 30 })
    .run(conn, callback)
```

[Read more about this command &rarr;](replace/)

## [delete](delete/) ##

{% apibody %}
table.delete([{durability: 'soft', return_vals: true}])
    &rarr; object
selection.delete([{durability: 'soft', return_vals: true}])
    &rarr; object
singleSelection.delete([{durability: 'soft', return_vals: true}])
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

```js
r.table('marvel').get('superman').delete().run(conn, callback)
```

[Read more about this command &rarr;](delete/)

{% endapisection %}


{% apisection Selecting data%}

## [db](db/) ##

{% apibody %}
r.db(dbName) &rarr; db
{% endapibody %}

Reference a database.

__Example:__ Before we can query a table we have to select the correct database.

```js
r.db('heroes').table('marvel').run(conn, callback)
```


## [table](table/) ##

{% apibody %}
db.table(name[, {useOutdated: false}]) &rarr; table
{% endapibody %}

Select all documents in a table. This command can be chained with other commands to do
further processing on the data.

__Example:__ Return all documents in the table 'marvel' of the default database.

```js
r.table('marvel').run(conn, callback)
```

[Read more about this command &rarr;](replace/)

## [get](get/) ##

{% apibody %}
table.get(key) &rarr; singleRowSelection
{% endapibody %}

Get a document by primary key.

__Example:__ Find a document with the primary key 'superman'.

```js
r.table('marvel').get('superman').run(conn, callback)
```


## [getAll](get_all) ##

{% apibody %}
table.getAll(key[, key2...], [, {index:'id'}]) &rarr; selection
{% endapibody %}

Get all documents where the given value matches the value of the requested index.

__Example:__ Secondary index keys are not guaranteed to be unique so we cannot query via
"get" when using a secondary index.

```js
r.table('marvel').getAll('man_of_steel', {index:'code_name'}).run(conn, callback)
```

[Read more about this command &rarr;](get_all/)


## [between](between/) ##

{% apibody %}
table.between(lowerKey, upperKey
    [, {index:'id', left_bound:'closed', right_bound:'open'}])
        &rarr; selection
{% endapibody %}

Get all documents between two keys. Accepts three optional arguments: `index`,
`left_bound`, and `right_bound`. If `index` is set to the name of a secondary index,
`between` will return all documents where that index's value is in the specified range
(it uses the primary key by default). `left_bound` or `right_bound` may be set to `open`
or `closed` to indicate whether or not to include that endpoint of the range (by default,
`left_bound` is closed and `right_bound` is open).

__Example:__ Find all users with primary key >= 10 and < 20 (a normal half-open interval).

```js
r.table('marvel').between(10, 20).run(conn, callback)
```

[Read more about this command &rarr;](between/)

## [filter](filter/) ##

{% apibody %}
sequence.filter(predicate) &rarr; selection
stream.filter(predicate) &rarr; stream
array.filter(predicate) &rarr; array
{% endapibody %}

Get all the documents for which the given predicate is true.

filter can be called on a sequence, selection, or a field containing an array of
elements. The return type is the same as the type on which the function was called on.
The body of every filter is wrapped in an implicit `.default(false)`, and the default
value can be changed by passing the optional argument `default`. Setting this optional
argument to `r.error()` will cause any non-existence errors to abort the filter.

__Example:__ Get all active users aged 30.

```js
r.table('users').filter({active: true, profile: {age: 30}}).run(conn, callback)
```

[Read more about this command &rarr;](between/)


{% endapisection %}


{% apisection Joins%}
These commands allow the combination of multiple sequences into a single sequence

## [innerJoin](inner_join/) ##

{% apibody %}
sequence.innerJoin(otherSequence, predicate) &rarr; stream
array.innerJoin(otherSequence, predicate) &rarr; array
{% endapibody %}

Returns the inner product of two sequences (e.g. a table, a filter result) filtered by
the predicate. The query compares each row of the left sequence with each row of the
right sequence to find all pairs of rows which satisfy the predicate. When the predicate
is satisfied, each matched pair of rows of both sequences are combined into a result row.

__Example:__ Construct a sequence of documents containing all cross-universe matchups where a marvel hero would lose.

```js
r.table('marvel').innerJoin(r.table('dc'), function(marvelRow, dcRow) {
    return marvelRow('strength').lt(dcRow('strength'))
}).run(conn, callback)
```


## [outerJoin](outer_join/) ##

{% apibody %}
sequence.outerJoin(otherSequence, predicate) &rarr; stream
array.outerJoin(otherSequence, predicate) &rarr; array
{% endapibody %}

Computes a left outer join by retaining each row in the left table even if no match was
found in the right table.

__Example:__ Construct a sequence of documents containing all cross-universe matchups
where a marvel hero would lose, but keep marvel heroes who would never lose a matchup in
the sequence.

```js
r.table('marvel').outerJoin(r.table('dc'), function(marvelRow, dcRow) {
    return marvelRow('strength').lt(dcRow('strength'))
}).run(conn, callback)
```


## [eqJoin](eq_join/) ##

{% apibody %}
sequence.eqJoin(leftAttr, otherTable[, {index:'id'}]) &rarr; stream
array.eqJoin(leftAttr, otherTable[, {index:'id'}]) &rarr; array
{% endapibody %}

An efficient join that looks up elements in the right table by primary key.

__Example:__ Let our heroes join forces to battle evil!

```js
r.table('marvel').eqJoin('main_dc_collaborator', r.table('dc')).run(conn, callback)
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
r.table('marvel').eqJoin('main_dc_collaborator', r.table('dc'))
    .zip().run(conn, callback)
```



{% endapisection %}

{% apisection Transformations%}
These commands are used to transform data in a sequence.

## [map](map/) ##

{% apibody %}
sequence.map(mappingFunction) &rarr; stream
array.map(mappingFunction) &rarr; array
{% endapibody %}

Transform each element of the sequence by applying the given mapping function.

__Example:__ Construct a sequence of hero power ratings.

```js
r.table('marvel').map(function(hero) {
    return hero('combatPower').add(hero('compassionPower').mul(2))
}).run(conn, callback)
```


## [withFields](with_fields/) ##

{% apibody %}
sequence.withFields([selector1, selector2...]) &rarr; stream
array.withFields([selector1, selector2...]) &rarr; array
{% endapibody %}

Takes a sequence of objects and a list of fields. If any objects in the sequence don't
have all of the specified fields, they're dropped from the sequence. The remaining
objects have the specified fields plucked out. (This is identical to `has_fields`
followed by `pluck` on a sequence.)

__Example:__ Get a list of heroes and their nemeses, excluding any heroes that lack one.

```js
r.table('marvel').withFields('id', 'nemesis')
```

[Read more about this command &rarr;](with_fields/)

## [concatMap](concat_map/) ##

{% apibody %}
sequence.concatMap(mappingFunction) &rarr; stream
array.concatMap(mappingFunction) &rarr; array
{% endapibody %}

Flattens a sequence of arrays returned by the mappingFunction into a single sequence.

__Example:__ Construct a sequence of all monsters defeated by Marvel heroes. Here the field
'defeatedMonsters' is a list that is concatenated to the sequence.

```js
r.table('marvel').concatMap(function(hero) {
    return hero('defeatedMonsters')
}).run(conn, callback)
```


## [orderBy](order_by/) ##

{% apibody %}
sequence.orderBy(key1, [key2...]) &rarr; stream
array.orderBy(key1, [key2...]) &rarr; array
{% endapibody %}

Sort the sequence by document values of the given key(s). `orderBy` defaults to ascending
ordering. To explicitly specify the ordering, wrap the attribute with either `r.asc` or
`r.desc`.

__Example:__ Order our heroes by a series of performance metrics.

```js
r.table('marvel').orderBy('enemiesVanquished', 'damselsSaved').run(conn, callback)
```

[Read more about this command &rarr;](order_by/)

## [skip](skip/) ##

{% apibody %}
sequence.skip(n) &rarr; stream
array.skip(n) &rarr; array
{% endapibody %}

Skip a number of elements from the head of the sequence.

__Example:__ Here in conjunction with `order_by` we choose to ignore the most successful heroes.

```js
r.table('marvel').orderBy('successMetric').skip(10).run(conn, callback)
```


## [limit](limit/) ##

{% apibody %}
sequence.limit(n) &rarr; stream
array.limit(n) &rarr; array
{% endapibody %}


End the sequence after the given number of elements.

__Example:__ Only so many can fit in our Pantheon of heroes.

```js
r.table('marvel').orderBy('belovedness').limit(10).run(conn, callback)
```

## [slice](slice/) ##

{% apibody %}
sequence.slice(startIndex[, endIndex]) &rarr; stream
array.slice(startIndex[, endIndex]) &rarr; array
{% endapibody %}

Trim the sequence to within the bounds provided.

__Example:__ For this fight, we need heroes with a good mix of strength and agility.

```js
r.table('marvel').orderBy('strength').slice(5, 10).run(conn, callback)
```

## [nth](nth/) ##

{% apibody %}
sequence.nth(index) &rarr; object
{% endapibody %}

Get the nth element of a sequence.

__Example:__ Select the second element in the array.

```js
r.expr([1,2,3]).nth(1).run(conn, callback)
```


## [indexesOf](indexes_of/) ##

{% apibody %}
sequence.indexesOf(datum | predicate) &rarr; array
{% endapibody %}

Get the indexes of an element in a sequence. If the argument is a predicate, get the indexes of all elements matching it.

__Example:__ Find the position of the letter 'c'.

```js
r.expr(['a','b','c']).indexesOf('c').run(conn, callback)
```

[Read more about this command &rarr;](indexes_of/)


## [isEmpty](is_empty/) ##

{% apibody %}
sequence.isEmpty() &rarr; bool
{% endapibody %}

Test if a sequence is empty.

__Example:__ Are there any documents in the marvel table?

```js
r.table('marvel').isEmpty().run(conn, callback)
```

## [union](union/) ##

{% apibody %}
sequence.union(sequence) &rarr; array
{% endapibody %}

Concatenate two sequences.

__Example:__ Construct a stream of all heroes.

```js
r.table('marvel').union(r.table('dc')).run(conn, callback)
```


## [sample](sample/) ##

{% apibody %}
sequence.sample(number) &rarr; selection
stream.sample(number) &rarr; array
array.sample(number) &rarr; array
{% endapibody %}

Select a given number of elements from a sequence with uniform random distribution. Selection is done without replacement.

__Example:__ Select 3 random heroes.

```js
r.table('marvel').sample(3).run(conn, callback)
```


{% endapisection %}


{% apisection Aggregation%}
These commands are used to compute smaller values from large sequences.

## [reduce](reduce/) ##

{% apibody %}
sequence.reduce(reductionFunction[, base]) &rarr; value
{% endapibody %}

Produce a single value from a sequence through repeated application of a reduction
function.

The reduce function gets invoked repeatedly not only for the input values but also for
results of previous reduce invocations. The type and format of the object that is passed
in to reduce must be the same with the one returned from reduce.

__Example:__ How many enemies have our heroes defeated?

```js
r.table('marvel').map(r.row('monstersKilled')).reduce(function(acc, val) {
    return acc.add(val)
}, 0).run(conn, callback)
```


## [count](count/) ##

{% apibody %}
sequence.count([filter]) &rarr; number
{% endapibody %}

Count the number of elements in the sequence. With a single argument, count the number
of elements equal to it. If the argument is a function, it is equivalent to calling
filter before count.

__Example:__ Just how many super heroes are there?

```js
r.table('marvel').count().add(r.table('dc').count()).run(conn, callback)
```

[Read more about this command &rarr;](count/)

## [distinct](distinct/) ##

{% apibody %}
sequence.distinct() &rarr; array
{% endapibody %}

Remove duplicate elements from the sequence.

__Example:__ Which unique villains have been vanquished by marvel heroes?

```js
r.table('marvel').concatMap(function(hero) {return hero('villainList')}).distinct()
    .run(conn, callback)
```


## [groupedMapReduce](grouped_map_reduce/) ##

{% apibody %}
sequence.groupedMapReduce(grouping, mapping, reduction, base)
    &rarr; value
{% endapibody %}

Partition the sequence into groups based on the `grouping` function. The elements of each
group are then mapped using the `mapping` function and reduced using the `reduction`
function.

`grouped_map_reduce` is a generalized form of group by.

__Example:__ It's only fair that heroes be compared against their weight class.

```js
r.table('marvel').groupedMapReduce(
    function(hero) { return hero('weightClass')},  // grouping
    function(hero) { return hero.pluck('name', 'strength')},  // mapping
    function(acc, hero) {  // reduction
        return r.branch(acc('strength').lt(hero('strength')), hero, acc)
    },
    {name:'none', strength:0} // reduction base
).run(conn, callback)
```


## [groupBy](group_by/) ##

{% apibody %}
sequence.groupBy(selector1[, selector2...], reductionObject)
    &rarr; array
{% endapibody %}

Groups elements by the values of the given attributes and then applies the given
reduction. Though similar to `groupedMapReduce`, `groupBy` takes a standardized object
for specifying the reduction. Can be used with a number of predefined common reductions.

__Example:__ Using a predefined reduction we can easily find the average strength of members of each weight class.

```js
r.table('marvel').groupBy('weightClass', r.avg('strength')).run(conn, callback)
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

```js
r.table('marvel').get('ironman')('opponents').contains('superman').run(conn, callback)
```

[Read more about this command &rarr;](contains/)


{% endapisection %}


{% apisection Aggregators%}
These standard aggregator objects are to be used in conjunction with groupBy.

## [count](count-aggregator/) ##

{% apibody %}
r.count
{% endapibody %}

Count the total size of the group.

__Example:__ Just how many heroes do we have at each strength level?

```js
r.table('marvel').groupBy('strength', r.count).run(conn, callback)
```


## [sum](sum/) ##

{% apibody %}
r.sum(attr)
{% endapibody %}

Compute the sum of the given field in the group.

__Example:__ How many enemies have been vanquished by heroes at each strength level?

```js
r.table('marvel').groupBy('strength', r.sum('enemiesVanquished')).run(conn, callback)
```


## [avg](avg/) ##

{% apibody %}
r.avg(attr)
{% endapibody %}

Compute the average value of the given attribute for the group.

__Example:__ What's the average agility of heroes at each strength level?

```js
r.table('marvel').groupBy('strength', r.avg('agility')).run(conn, callback)
```



{% endapisection %}


{% apisection Document manipulation%}

## [row](row/) ##

{% apibody %}
r.row &rarr; value
{% endapibody %}

Returns the currently visited document.

__Example:__ Get all users whose age is greater than 5.

```js
r.table('users').filter(r.row('age').gt(5)).run(conn, callback)
```

[Read more about this command &rarr;](row/)


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

```js
r.table('marvel').get('IronMan').pluck('reactorState', 'reactorPower').run(conn, callback)
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

```js
r.table('marvel').get('IronMan').without('personalVictoriesList').run(conn, callback)
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

```js
r.table('marvel').get('IronMan').merge(
    r.table('loadouts').get('alienInvasionKit')
).run(conn, callback)
```

[Read more about this command &rarr;](merge/)


## [append](append/) ##

{% apibody %}
array.append(value) &rarr; array
{% endapibody %}

Append a value to an array.

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```js
r.table('marvel').get('IronMan')('equipment').append('newBoots').run(conn, callback)
```


## [prepend](prepend/) ##

{% apibody %}
array.prepend(value) &rarr; array
{% endapibody %}

Prepend a value to an array.

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```js
r.table('marvel').get('IronMan')('equipment').prepend('newBoots').run(conn, callback)
```


## [difference](difference/) ##

{% apibody %}
array.difference(array) &rarr; array
{% endapibody %}

Remove the elements of one array from another array.

__Example:__ Retrieve Iron Man's equipment list without boots.

```js
r.table('marvel').get('IronMan')('equipment').difference(['Boots']).run(conn, callback)
```


## [setInsert](set_insert/) ##

{% apibody %}
array.setInsert(value) &rarr; array
{% endapibody %}

Add a value to an array and return it as a set (an array with distinct values).

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.

```js
r.table('marvel').get('IronMan')('equipment').setInsert('newBoots').run(conn, callback)
```


## [setUnion](set_union/) ##

{% apibody %}
array.setUnion(array) &rarr; array
{% endapibody %}

Add a several values to an array and return it as a set (an array with distinct values).

__Example:__ Retrieve Iron Man's equipment list with the addition of some new boots and an arc reactor.

```js
r.table('marvel').get('IronMan')('equipment').setUnion(['newBoots', 'arc_reactor']).run(conn, callback)
```


## [setIntersection](set_intersection/) ##

{% apibody %}
array.setIntersection(array) &rarr; array
{% endapibody %}

Intersect two arrays returning values that occur in both of them as a set (an array with
distinct values).

__Example:__ Check which pieces of equipment Iron Man has from a fixed list.

```js
r.table('marvel').get('IronMan')('equipment').setIntersection(['newBoots', 'arc_reactor']).run(conn, callback)
```


## [setDifference](set_difference/) ##

{% apibody %}
array.setDifference(array) &rarr; array
{% endapibody %}

Remove the elements of one array from another and return them as a set (an array with
distinct values).

__Example:__ Check which pieces of equipment Iron Man has, excluding a fixed list.

```js
r.table('marvel').get('IronMan')('equipment').setDifference(['newBoots', 'arc_reactor']).run(conn, callback)
```


## [()](get_attr) ##

{% apibody %}
sequence(attr) &rarr; sequence
singleSelection(attr) &rarr; value
object(attr) &rarr; value
{% endapibody %}

Get a single field from an object. If called on a sequence, gets that field from every
object in the sequence, skipping objects that lack it.

__Example:__ What was Iron Man's first appearance in a comic?

```js
r.table('marvel').get('IronMan')('firstAppearance').run(conn, callback)
```


## [hasFields](has_fields/) ##

{% apibody %}
sequence.hasFields([selector1, selector2...]) &rarr; stream
array.hasFields([selector1, selector2...]) &rarr; array
singleSelection.hasFields([selector1, selector2...]) &rarr; boolean
object.hasFields([selector1, selector2...]) &rarr; boolean
{% endapibody %}

Test if an object has all of the specified fields. An object has a field if it has the
specified key and that key maps to a non-null value. For instance, the object
`{'a':1,'b':2,'c':null}` has the fields `a` and `b`.

__Example:__ Which heroes are married?

```js
r.table('marvel').hasFields('spouse')
```

[Read more about this command &rarr;](has_fields/)


## [insertAt](insert_at) ##

{% apibody %}
array.insertAt(index, value) &rarr; array
{% endapibody %}

Insert a value in to an array at a given index. Returns the modified array.

__Example:__ Hulk decides to join the avengers.

```js
r.expr(["Iron Man", "Spider-Man"]).insertAt(1, "Hulk").run(conn, callback)
```


## [spliceAt](splice_at) ##

{% apibody %}
array.spliceAt(index, array) &rarr; array
{% endapibody %}

Insert several values in to an array at a given index. Returns the modified array.

__Example:__ Hulk and Thor decide to join the avengers.

```js
r.expr(["Iron Man", "Spider-Man"]).spliceAt(1, ["Hulk", "Thor"]).run(conn, callback)
```


## [deleteAt](delete_at/) ##

{% apibody %}
array.deleteAt(index [,endIndex]) &rarr; array
{% endapibody %}

Remove an element from an array at a given index. Returns the modified array.

__Example:__ Hulk decides to leave the avengers.

```js
r.expr(["Iron Man", "Hulk", "Spider-Man"]).deleteAt(1).run(conn, callback)
```

[Read more about this command &rarr;](delete_at/)

## [changeAt](change_at/) ##

{% apibody %}
array.changeAt(index, value) &rarr; array
{% endapibody %}

Change a value in an array at a given index. Returns the modified array.

__Example:__ Bruce Banner hulks out.

```js
r.expr(["Iron Man", "Bruce", "Spider-Man"]).changeAt(1, "Hulk").run(conn, callback)
```

## [keys](keys) ##

{% apibody %}
singleSelection.keys() &rarr; array
object.keys() &rarr; array
{% endapibody %}

Return an array containing all of the object's keys.

__Example:__ Get all the keys of a row.

```js
r.table('marvel').get('ironman').keys().run(conn, callback)
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

```js
r.table('users').filter(function(row){return row('name').match("^A")}).run(conn, callback)
```

[Read more about this command &rarr;](match/)

{% endapisection %}


{% apisection Math and logic%}

## [add](add/) ##

{% apibody %}
number.add(number) &rarr; number
string.add(string) &rarr; string
array.add(array) &rarr; array
time.add(number) &rarr; time
{% endapibody %}

Sum two numbers, concatenate two strings, or concatenate 2 arrays.

__Example:__ It's as easy as 2 + 2 = 4.

```js
r.expr(2).add(2).run(conn, callback)
```


[Read more about this command &rarr;](add/)

## [sub](sub/) ##

{% apibody %}
number.sub(number) &rarr; number
time.sub(time) &rarr; number
time.sub(number) &rarr; time
{% endapibody %}

Subtract two numbers.

__Example:__ It's as easy as 2 - 2 = 0.

```js
r.expr(2).sub(2).run(conn, callback)
```

[Read more about this command &rarr;](sub/)


## [mul](mul/) ##

{% apibody %}
number.mul(number) &rarr; number
array.mul(number) &rarr; array
{% endapibody %}

Multiply two numbers, or make a periodic array.

__Example:__ It's as easy as 2 * 2 = 4.

```js
r.expr(2).mul(2).run(conn, callback)
```

[Read more about this command &rarr;](mul/)


## [div](div/) ##

{% apibody %}
number.div(number) &rarr; number
{% endapibody %}

Divide two numbers.

__Example:__ It's as easy as 2 / 2 = 1.

```js
r.expr(2).div(2).run(conn, callback)
```



## [mod](mod/) ##

{% apibody %}
number.mod(number) &rarr; number
{% endapibody %}

Find the remainder when dividing two numbers.

__Example:__ It's as easy as 2 % 2 = 0.

```js
r.expr(2).mod(2).run(conn, callback)
```

## [and](and/) ##

{% apibody %}
bool.and(bool) &rarr; bool
{% endapibody %}

Compute the logical and of two values.

__Example:__ True and false anded is false?

```js
r.expr(true).and(false).run(conn, callback)
```


## [or](or/) ##

{% apibody %}
bool.or(bool) &rarr; bool
{% endapibody %}

Compute the logical or of two values.

__Example:__ True or false ored is true?

```js
r.expr(true).or(false).run(conn, callback)
```


## [eq](eq/) ##

{% apibody %}
value.eq(value) &rarr; bool
{% endapibody %}

Test if two values are equal.

__Example:__ Does 2 equal 2?

```js
r.expr(2).eq(2).run(conn, callback)
```


## [ne](ne/) ##

{% apibody %}
value.ne(value) &rarr; bool
{% endapibody %}

Test if two values are not equal.

__Example:__ Does 2 not equal 2?

```js
r.expr(2).ne(2).run(conn, callback)
```


## [gt](gt/) ##

{% apibody %}
value.gt(value) &rarr; bool
{% endapibody %}

Test if the first value is greater than other.

__Example:__ Is 2 greater than 2?

```js
r.expr(2).gt(2).run(conn, callback)
```

## [ge](ge/) ##

{% apibody %}
value.ge(value) &rarr; bool
{% endapibody %}

Test if the first value is greater than or equal to other.

__Example:__ Is 2 greater than or equal to 2?

```js
r.expr(2).ge(2).run(conn, callback)
```

## [lt](lt/) ##

{% apibody %}
value.lt(value) &rarr; bool
{% endapibody %}

Test if the first value is less than other.

__Example:__ Is 2 less than 2?

```js
r.expr(2).lt(2).run(conn, callback)
```

## [le](le/) ##

{% apibody %}
value.le(value) &rarr; bool
{% endapibody %}

Test if the first value is less than or equal to other.

__Example:__ Is 2 less than or equal to 2?

```js
r.expr(2).le(2).run(conn, callback)
```


## [not](not/) ##

{% apibody %}
bool.not() &rarr; bool
{% endapibody %}
Compute the logical inverse (not).

__Example:__ Not true is false.

```js
r.expr(true).not().run(conn, callback)
```


{% endapisection %}


{% apisection Dates and times%}

## [now](now/) ##

{% apibody %}
r.now() &rarr; time
{% endapibody %}

Return a time object representing the current time in UTC. The command now() is computed once when the server receives the query, so multiple instances of r.now() will always return the same time inside a query.

__Example:__ Add a new user with the time at which he subscribed.

```js
r.table("users").insert({
    name: "John",
    subscription_date: r.now()
}).run(conn, callback)
```

## [time](time/) ##

{% apibody %}
r.time(year, month, day[, hour, minute, second], timezone)
    &rarr; time
{% endapibody %}

Create a time object for a specific time.

__Example:__ Update the birthdate of the user "John" to November 3rd, 1986 UTC.

```js
r.table("user").get("John").update({birthdate: r.time(1986, 11, 3, 'Z')}).run(conn, callback)
```



## [epochTime](epoch_time/) ##

{% apibody %}
r.epochTime(epochTime) &rarr; time
{% endapibody %}

Create a time object based on seconds since epoch.

__Example:__ Update the birthdate of the user "John" to November 3rd, 1986.

```js
r.table("user").get("John").update({birthdate: r.epochTime(531360000)}).run(conn, callback)
```


## [ISO8601](iso8601/) ##

{% apibody %}
r.ISO8601(iso8601Date[, {default_timezone:''}]) &rarr; time
{% endapibody %}

Create a time object based on an iso8601 date-time string (e.g.
'2013-01-01T01:01:01+00:00'). We support all valid ISO 8601 formats except for week
dates. If you pass an ISO 8601 date-time without a time zone, you must specify the time
zone with the optarg `default_timezone`. Read more about the ISO 8601 format on the
Wikipedia page.

__Example:__ Update the time of John's birth.

```js
r.table("user").get("John").update({birth: r.ISO8601('1986-11-03T08:30:00-07:00')}).run(conn, callback)
```


## [inTimezone](in_timezone/) ##

{% apibody %}
time.inTimezone(timezone) &rarr; time
{% endapibody %}

Return a new time object with a different timezone. While the time stays the same, the results returned by methods such as hours() will change since they take the timezone into account. The timezone argument has to be of the ISO 8601 format.

__Example:__ Hour of the day in San Francisco (UTC/GMT -8, without daylight saving time).

```js
r.now().inTimezone('-08:00').hours().run(conn, callback)
```



## [timezone](timezone/) ##

{% apibody %}
time.timezone() &rarr; string
{% endapibody %}

Return the timezone of the time object.

__Example:__ Return all the users in the "-07:00" timezone.

```js
r.table("users").filter( function(user) {
    return user("subscriptionDate").timezone().eq("-07:00")
})
```


## [during](during/) ##

{% apibody %}
time.during(startTime, endTime[, options]) &rarr; bool
{% endapibody %}

Return if a time is between two other times (by default, inclusive for the start, exclusive for the end).

__Example:__ Retrieve all the posts that were posted between December 1st, 2013 (inclusive) and December 10th, 2013 (exclusive).

```js
r.table("posts").filter(
    r.row('date').during(r.time(2013, 12, 1), r.time(2013, 12, 10))
).run(conn, callback)
```

[Read more about this command &rarr;](during/)



## [date](date/) ##

{% apibody %}
time.date() &rarr; time
{% endapibody %}

Return a new time object only based on the day, month and year (ie. the same day at 00:00).

__Example:__ Retrieve all the users whose birthday is today

```js
r.table("users").filter(function(user) {
    return user("birthdate").date().eq(r.now().date())
}).run(conn, callback)
```



## [timeOfDay](time_of_day/) ##

{% apibody %}
time.timeOfDay() &rarr; number
{% endapibody %}

Return the number of seconds elapsed since the beginning of the day stored in the time object.

__Example:__ Retrieve posts that were submitted before noon.

```js
r.table("posts").filter(
    r.row("date").timeOfDay().le(12*60*60)
).run(conn, callback)
```


## [year](year/) ##

{% apibody %}
time.year() &rarr; number
{% endapibody %}

Return the year of a time object.

__Example:__ Retrieve all the users born in 1986.

```js
r.table("users").filter(function(user) {
    return user("birthdate").year().eq(1986)
}).run(conn, callback)
```


## [month](month/) ##

{% apibody %}
time.month() &rarr; number
{% endapibody %}

Return the month of a time object as a number between 1 and 12. For your convenience, the terms r.january, r.february etc. are defined and map to the appropriate integer.

__Example:__ Retrieve all the users who were born in November.

```js
r.table("users").filter(
    r.row("birthdate").month().eq(11)
)
```

[Read more about this command &rarr;](month/)


## [day](day/) ##

{% apibody %}
time.day() &rarr; number
{% endapibody %}

Return the day of a time object as a number between 1 and 31.

__Example:__ Return the users born on the 24th of any month.

```js
r.table("users").filter(
    r.row("birthdate").day().eq(24)
).run(conn, callback)
```



## [dayOfWeek](day_of_week/) ##

{% apibody %}
time.dayOfWeek() &rarr; number
{% endapibody %}

Return the day of week of a time object as a number between 1 and 7 (following ISO 8601 standard). For your convenience, the terms r.monday, r.tuesday etc. are defined and map to the appropriate integer.

__Example:__ Return today's day of week.

```js
r.now().dayOfWeek().run(conn, callback)
```

[Read more about this command &rarr;](day_of_week/)



## [dateOfYear](date_of_year/) ##

{% apibody %}
time.day_of_year() &rarr; number
{% endapibody %}

Return the day of the year of a time object as a number between 1 and 366 (following ISO 8601 standard).

__Example:__ Retrieve all the users who were born the first day of a year.

```js
r.table("users").filter(
    r.row("birthdate").dayOfYear().eq(1)
)
```


## [hours](hours/) ##

{% apibody %}
time.hours() &rarr; number
{% endapibody %}

Return the hour in a time object as a number between 0 and 23.

__Example:__ Return all the posts submitted after midnight and before 4am.

```js
r.table("posts").filter(function(post) {
    return post("date").hours().lt(4)
})
```


## [minutes](minutes/) ##

{% apibody %}
time.minutes() &rarr; number
{% endapibody %}

Return the minute in a time object as a number between 0 and 59.

__Example:__ Return all the posts submitted during the first 10 minutes of every hour.

```js
r.table("posts").filter(function(post) {
    return post("date").minutes().lt(10)
})
```



## [seconds](seconds/) ##

{% apibody %}
time.seconds() &rarr; number
{% endapibody %}

Return the seconds in a time object as a number between 0 and 59.999 (double precision).

__Example:__ Return the post submitted during the first 30 seconds of every minute.

```js
r.table("posts").filter(function(post) {
    return post("date").seconds().lt(30)
})
```

## [toISO8601](to_iso8601/) ##

{% apibody %}
time.toISO8601() &rarr; string
{% endapibody %}

Convert a time object to its iso 8601 format.

__Example:__ Return the current time in an ISO8601 format.

```js
r.now().toISO8601()
```


## [toEpochTime](to_epoch_time) ##

{% apibody %}
time.to_epoch_time() &rarr; number
{% endapibody %}

Convert a time object to its epoch time.

__Example:__ Return the current time in an ISO8601 format.

```js
r.now().toEpochTime()
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

```js
r.do(r.table('marvel').get('IronMan'),
    function (ironman) { return ironman('name'); }
).run(conn, callback)
```


## [branch](branch/) ##

{% apibody %}
r.branch(test, true_branch, false_branch) &rarr; any
{% endapibody %}

Evaluate one of two control paths based on the value of an expression. branch is effectively an if renamed due to language constraints.

The type of the result is determined by the type of the branch that gets executed.

__Example:__ Return the manlier of two heroes:

```
r.table('marvel').map(r.branch(r.row('victories').gt(100),
    r.row('name').add(' is a superhero'),
    r.row('name').add(' is a hero'))
).run(conn, callback)
```


## [forEach](for_each/) ##

{% apibody %}
sequence.forEach(write_query) &rarr; object
{% endapibody %}

Loop over a sequence, evaluating the given write query for each element.

__Example:__ Now that our heroes have defeated their villains, we can safely remove them from the villain table.

```js
r.table('marvel').forEach(function(hero) {
    return r.table('villains').get(hero('villainDefeated')).delete()
}).run(conn, callback)
```



## [error](error/) ##

{% apibody %}
r.error(message) &rarr; error
{% endapibody %}

Throw a runtime error. If called with no arguments inside the second argument to `default`, re-throw the current error.

__Example:__ Iron Man can't possibly have lost a battle:

```js
r.table('marvel').get('IronMan').do(function(ironman) {
    return r.branch(ironman('victories').lt(ironman('battles')),
        r.error('impossible code path'),
        ironman)
}).run(conn, callback)
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

```js
r.table('projects').map(function(p) {
    return p('staff').default(0).add(p('management').default(0))
}).run(conn, callback)
```


## [expr](expr/) ##

{% apibody %}
r.expr(value) &rarr; value
{% endapibody %}

Construct a RQL JSON object from a native object.

__Example:__ Objects wrapped with expr can then be manipulated by RQL API functions.

```js
r.expr({a:'b'}).merge({b:[1,2,3]}).run(conn, callback)
```

[Read more about this command &rarr;](expr/)

## [js](js/) ##

{% apibody %}
r.js(jsString) &rarr; value
{% endapibody %}

Create a javascript expression.

__Example:__ Concatenate two strings using Javascript'

```js
r.js("'str1' + 'str2'").run(conn, callback)
```

[Read more about this command &rarr;](js/)

## [coerceTo](coerce_to/) ##

{% apibody %}
sequence.coerceTo(typeName) &rarr; array
value.coerceTo(typeName) &rarr; string
array.coerceTo(typeName) &rarr; object
object.coerceTo(typeName) &rarr; array
{% endapibody %}

Converts a value of one type into another. 

You can convert: a selection, sequence, or object into an ARRAY, an array of pairs into an OBJECT, and any DATUM into a STRING.

__Example:__ Convert a table to an array.

```js
r.table('marvel').coerceTo('array').run(conn, callback)
```

[Read more about this command &rarr;](coerce_to/)

## [typeOf](type_of/) ##

{% apibody %}
any.typeOf() &rarr; string
{% endapibody %}

Gets the type of a value.

__Example:__ Get the type of a string.

```js
r.expr("foo").typeOf().run(conn, callback)
```

## [info](info/) ##

{% apibody %}
any.info() &rarr; object
{% endapibody %}

Get information about a RQL value.

__Example:__ Get information about a table such as primary key, or cache size.

```js
r.table('marvel').info().run(conn, callback)
```

## [json](json/) ##

{% apibody %}
r.json(json_string) &rarr; value
{% endapibody %}

Parse a JSON string on the server.

__Example:__ Send an array to the server'

```js
r.json("[1,2,3]").run(conn, callback)
```


{% endapisection %}






