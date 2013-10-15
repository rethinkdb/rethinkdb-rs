---
layout: api-command 
language: JavaScript
permalink: api/javascript/replace/
command: replace
---

{% apibody %}
table.replace(json | expr[, {durability: 'soft', return_vals: true}]) → object
selection.replace(json | expr[, {durability: 'soft', return_vals: true}]) → object
singleSelection.replace(json | expr[, {durability: 'soft', return_vals: true}]) → object
{% endapibody %}

Replace documents in a table. Accepts a JSON document or a RQL expression, and replaces
the original document with the new one. The new document must have the same primary key
as the original document. The optional argument durability with value 'hard' or 'soft'
will override the table or query's default durability setting. The optional argument
return_vals will return the old and new values of the row you're modifying when set to
true (only valid for single-row replacements). The optional argument non_atomic lets you
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


__Example:__ Allow the server to run non-atomic operations.

```js
r.table('marvel').get('superman').replace({ id: 'superman', age: r.js('30') },
    { 'nonAtomic': true }
).run(conn, callback)
```

__Example:__ Mark all Marvel heroes as favorites, specifying soft durability.

```js
r.table('heroes').filter(r.row('universe').eq('marvel'))
    .replace(function(doc) {return doc.merge({is_fav: true})}, { durability: 'soft' })
    .run(conn, callback)
```

__Example:__ You can get a copy of the previous value and the old value back using the return_vals flag.

```js
r.table('heroes').filter(r.row('universe').eq('marvel'))
    .replace(function(doc) {return doc.merge({is_fav: true})}, { return_vals: true })
    .run(conn, callback)
```
