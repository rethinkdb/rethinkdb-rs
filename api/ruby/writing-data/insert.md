---
layout: api-command
language: Ruby
permalink: api/ruby/insert/
command: insert
related_commands:
    update: update/
    replace: replace/
    delete: delete/
---

# Command syntax #

{% apibody %}
table.insert(json | [json][, :durability => "hard", :return_vals => false :conflict => "error"])
    &rarr; object
{% endapibody %}

# Description #

<img src="/assets/images/docs/api_illustrations/insert_ruby.png" class="api_command_illustration" />

Insert documents into a table. Accepts a single document or an array of
documents.

The optional arguments are:

- `durability`: possible values are `hard` and `soft`. This option will override the table or query's durability setting (set in [run](/api/ruby/run/)). In soft durability mode Rethink_dB will acknowledge the write immediately after receiving and caching it, but before the write has been committed to disk.
- `return_vals`: if set to `true` when a single document is given, return `old_val` and `new_val` keys with the old value of the document (or `nil` on an insert) and the new value.
- `conflict`: Determine handling of inserting documents with the same primary key as existing entries. Possible values are `"error"`, `"replace"` or `"update"`.
    - `"error"`: Do not insert the new document and record the conflict as an error. This is the default.
    - `"replace"`: [Replace](/api/ruby/replace/) the old document in its entirety with the new one.
    - `"update"`: [Update](/api/ruby/update/) fields of the old document with fields from the new one.

Insert returns an object that contains the following attributes:

- `inserted`: the number of documents successfully inserted.
- `replaced`: the number of documents updated when `conflict` is set to `"replace"` or `"update"`.
- `unchanged`: the number of documents whose fields are identical to existing documents with the same primary key when `conflict` is set to `"replace"` or `"update"`.
- `errors`: the number of errors encountered while performing the insert.
- `first_error`: If errors were encountered, contains the text of the first error.
- `deleted` and `skipped`: 0 for an insert operation.
- `generated_keys`: a list of generated primary keys for inserted documents whose primary keys were not specified (capped to 100,000).
- `warnings`: if the field `generated_keys` is truncated, you will get the warning _"Too many generated keys (&lt;X&gt;), array truncated to 100000."_.
- `old_val`: if `return_vals` is set to `true`, contains `nil`.
- `new_val`: if `return_vals` is set to `true`, contains the inserted/updated document.

__Example:__ Insert a document into the table `posts`.

```rb
r.table("posts").insert({
    :id => 1,
    :title => "Lorem ipsum",
    :content => "Dolor sit amet"
}).run(conn)
```

The result will be:

```rb
{
    :deleted => 0,
    :errors => 0,
    :inserted => 1,
    :replaced => 0,
    :skipped => 0,
    :unchanged => 0
}
```


__Example:__ Insert a document without a defined primary key into the table `posts` where the
primary key is `id`.

```rb
r.table("posts").insert({
    :title => "Lorem ipsum",
    :content => "Dolor sit amet"
}).run(conn)
```

RethinkDB will generate a primary key and return it in `generated_keys`.

```rb
{
    :deleted => 0,
    :errors => 0,
    :generated_keys => [
        "dd782b64-70a7-43e4-b65e-dd14ae61d947"
    ],
    :inserted => 1,
    :replaced => 0,
    :skipped => 0,
    :unchanged => 0
}
```

Retrieve the document you just inserted with:

```rb
r.table("posts").get("dd782b64-70a7-43e4-b65e-dd14ae61d947").run(conn)
```

And you will get back:

```rb
{
    :id => "dd782b64-70a7-43e4-b65e-dd14ae61d947",
    :title => "Lorem ipsum",
    :content => "Dolor sit amet"
}
```


__Example:__ Insert multiple documents into the table `users`.

```rb
r.table("users").insert([
    {:id => "william", :email =>"william@rethinkdb.com"},
    {:id => "lara", :email => "lara@rethinkdb.com"}
]).run(conn)
```


__Example:__ Insert a document into the table `users`, replacing the document if the document
already exists.  
_Note_: If the document exists, the `insert` command will behave like [replace](/api/ruby/replace/), not like [update](/api/ruby/update/) 

```rb
r.table("users").insert(
    {:id => "william", :email => "william@rethinkdb.com"},
    :conflict => "error"
).run(conn)
```


__Example:__ Copy the documents from `posts` to `posts_backup`.

```rb
r.table("posts_backup").insert( r.table("posts") ).run(conn)
```


__Example:__ Get back a copy of the inserted document (with its generated primary key).

```rb
r.table("posts").insert(
    {:title => "Lorem ipsum", :content => "Dolor sit amet"},
    :return_vals => true
).run(conn)
```

The result will be

```rb
{
    :deleted => 0,
    :errors => 0,
    :generated_keys => [
        "dd782b64-70a7-43e4-b65e-dd14ae61d947"
    ],
    :inserted => 1,
    :replaced => 0,
    :skipped => 0,
    :unchanged => 0,
    :old_val => nil,
    :new_val => {
        :id => "dd782b64-70a7-43e4-b65e-dd14ae61d947",
        :title => "Lorem ipsum",
        :content => "Dolor sit amet"
    }
}
```
