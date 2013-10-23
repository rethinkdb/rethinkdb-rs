---
layout: api-command 
language: Python
permalink: api/python/update/
command: update
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/writing-data/update.md
related_commands:
    insert: insert/
    replace: replace/
    delete: delete/
---

{% apibody %}
table.update(json | expr[, durability='soft', return_vals=true]) → object
selection.update(json | expr[, durability='soft', return_vals=true]) → object
singleSelection.update(json | expr[, durability='soft', return_vals=true]) → object
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

```py
r.table('marvel').get('superman').update({ 'age': 30 }).run(conn)
```


__Example:__ Increment every superhero's age. If age doesn't exist, throws an error. Specify soft durability.

```py
r.table('marvel').update(lambda x: {'age': x['age'] + 1}, durability='soft').run(conn)
```


__Example:__ Allow the server to run non-atomic operations.

```py
r.table('marvel').update(
    lambda x: {'age': x['age'] + r.js('1')}, non_atomic=True
).run(conn)
```


__Example:__ You can get back a copy of the original row and the update row using the return_vals flag.

```py
r.table('marvel').get('superman').update({ 'age': 30 }, return_vals=True).run(conn)
```

