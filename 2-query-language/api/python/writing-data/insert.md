---
layout: api-command 
language: Python
permalink: api/python/insert/
command: insert
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/writing-data/insert.md
related_commands:
    update: update/ 
    replace: replace/
    delete: delete/
---


{% apibody %}
table.insert(json | [json][, durability='soft', return_vals=True, upsert=True]) &rarr; object
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

```py
r.table('marvel').insert(
    { 'superhero': 'Iron Man', 'superpower':'Arc Reactor' }).run(conn)
```


__Example:__ Insert multiple rows into a table named 'marvel'. Also, specify that only
soft durability is required.

```py
r.table('marvel').insert([
    { 'superhero': 'Wolverine', 'superpower': 'Adamantium' },
    { 'superhero': 'Spiderman', 'superpower': 'spidy sense' }
    ], durability='soft'
).run(conn)
```


__Example:__ Insert a row into a table named 'marvel', overwriting if the document already exists.

```py
r.table('marvel').insert(
    { 'superhero': 'Iron Man', 'superpower': 'Arc Reactor' },
    upsert=True
).run(conn)


__Example:__ Get back a copy of the new row, this is useful if you've done an upsert or generated an ID.

```py
r.table('marvel').insert(
    { 'superhero': 'Iron Man', 'superpower': 'Arc Reactor' },
    upsert=True, return_vals=True
).run(conn)
```
