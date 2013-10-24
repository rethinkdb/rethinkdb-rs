---
layout: api-command 
language: Ruby
permalink: api/ruby/update/
command: update 
related_commands:
    insert: insert/
    replace: replace/
    delete: delete/
---


# Command syntax #

{% apibody %}
table.update(json | expr[, durability => 'soft', return_vals => true])
    &rarr; object
selection.update(json | expr[, durability => 'soft', return_vals => true])
    &rarr; object
singleSelection.update(json | expr[, durability => 'soft', return_vals => true])
    &rarr; object
{% endapibody %}

# Description #

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

__Example:__ Increment every superhero's age. If age doesn't exist, throws an error. Specify soft durability.

```rb
r.table('marvel').update(:durability => 'soft') {|hero| {:age => hero[:age] + 1}}.run(conn)
```


__Example:__ Allow the server to run non-atomic operations.


```rb
r.table('marvel').update( { :non_atomic => true }) {|hero|
    {:age => hero[:age] + r.js('1')}
}.run(conn)
```


__Example:__ You can get back a copy of the original row and the update row using the return_vals flag.

```rb
r.table('marvel').get('superman').update( {:age => 30}, :return_vals => true).run(conn)
```
