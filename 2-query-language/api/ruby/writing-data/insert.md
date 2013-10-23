---
layout: api-command 
language: Ruby
permalink: api/ruby/insert/
command: insert
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/writing-data/insert.md
related_commands:
    update: update/
    replace: replace/
    delete: delete/
---

# Command syntax #

{% apibody %}
table.insert(json | [json][, { :durability => 'soft', :return_vals => true :upsert => true}])
    &rarr; object
{% endapibody %}

# Description #

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

__Example:__ Insert multiple rows into a table named 'marvel'. Also, specify that only soft durability is required.

```rb
r.table('marvel').insert([
    {:superhero => 'Wolverine', :superpower => 'Adamantium'},
    {:superhero => 'Spiderman', :superpower => 'spidy sense'}
    ], :durability => 'soft'
).run(conn)
```


__Example:__ Insert a row into a table named 'marvel', overwriting if the document
already exists.

```rb
r.table('marvel').insert(
    {:superhero => 'Iron Man', :superpower => 'Arc Reactor' },
    {:upsert => true }
).run(conn)
```

__Example:__ Get back a copy of the new row, this is useful if you've done an upsert or
generated an ID.

```rb
r.table('marvel').insert(
    {:superhero => 'Iron Man', :superpower => 'Arc Reactor' },
    {:upsert => true , :return_vals => true}
).run(conn)
```
