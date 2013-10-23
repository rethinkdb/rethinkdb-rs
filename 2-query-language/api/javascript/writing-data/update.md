---
layout: api-command 
language: JavaScript
permalink: api/javascript/update/
command: update
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/writing-data/update.md
io:
    -   - table
        - object
    -   - selection
        - object
    -   - singleSelection
        - object
related_commands:
    insert: insert/
    replace: replace/
    delete: delete/
---


{% apibody %}
table.update(json | expr[, {durability: 'soft', return_vals: true]) &rarr; object
selection.update(json | expr[, {durability: 'soft', return_vals: true]) &rarr; object
singleSelection.update(json | expr[, {durability: 'soft', return_vals: true]) &rarr; object
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


__Example:__ Increment every superhero's age. If age doesn't exist, throws an error. Specify soft durability.

```js
r.table('marvel').update(
    { age: r.row('age').add(1) },
    { durability: 'soft' }
).run(conn, callback)
```


__Example:__ Allow the server to run non-atomic operations.

```js
r.table('marvel').update(
    { age: r.row('age').add(r.js('1')) },
    {'nonAtomic':true}
).run(conn, callback)
```

__Example:__ You can get back a copy of the original row and the update row using the return_vals flag.

```js
r.table('marvel').get('superman').update({ age: 30 }, {return_vals: true}).run(conn, callback)
```

