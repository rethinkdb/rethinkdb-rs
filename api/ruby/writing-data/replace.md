---
layout: api-command
language: Ruby
permalink: api/ruby/replace/
command: replace
related_commands:
    insert: insert/
    update: update/
    delete: delete/
---


# Command syntax #

{% apibody %}
table.replace(json | expr[, :durability => "hard", :return_vals => false, :non_atomic => false])
    &rarr; object
selection.replace(json | expr[, :durability => "hard", :return_vals => false, :non_atomic => false])
    &rarr; object
singleSelection.replace(json | expr[, :durability => "hard", :return_vals => false, :non_atomic => false])
    &rarr; object
{% endapibody %}

<img src="/assets/images/docs/api_illustrations/replace.png" class="api_command_illustration" />

# Description #

Replace documents in a table. Accepts a JSON document or a ReQL expression, and replaces
the original document with the new one. The new document must have the same primary key
as the original document.

The optional arguments are:

- `durability`: possible values are `hard` and `soft`. This option will override the
table or query's durability setting (set in [run](/api/ruby/run/)).  
In soft durability mode RethinkDB will acknowledge the write immediately after
receiving it, but before the write has been committed to disk.
- `return_vals`: if set to `true` and in case of a single replace, the replaced document
will be returned.
- `non_atomic`: set to `true` if you want to perform non-atomic replaces (replaces that
require fetching data from another document).


Replace returns an object that contains the following attributes:

- `replaced`: the number of documents that were replaced
- `unchanged`: the number of documents that would have been modified, except that the
new value was the same as the old value
- `inserted`: the number of new documents added. You can have new documents inserted if
you do a point-replace on a key that isn't in the table or you do a replace on a
selection and one of the documents you are replacing has been deleted
- `deleted`: the number of deleted documents when doing a replace with `nil`
- `errors`: the number of errors encountered while performing the replace.
- `first_error`: If errors were encountered, contains the text of the first error.
- `skipped`: 0 for a replace operation
- `old_val`: if `return_vals` is set to `true`, contains the old document.
- `new_val`: if `return_vals` is set to `true`, contains the new document.

__Example:__ Replace the document with the primary key `1`.

```rb
r.table("posts").get(1).replace({
    :id => 1,
    :title => "Lorem ipsum",
    :content => "Aleas jacta est",
    :status => "draft"
}).run(conn)
```

__Example:__ Remove the field `status` from all posts.

```rb
r.table("posts").replace{ |post|
    post.without("status")
}.run(conn)
```

__Example:__ Remove all the fields that are not `id`, `title` or `content`.

```rb
r.table("posts").replace{ |post|
    post.pluck("id", "title", "content")
}.run(conn)
```

__Example:__ Replace the document with the primary key `1` using soft durability.

```rb
r.table("posts").get(1).replace({
    :id => 1,
    :title => "Lorem ipsum",
    :content => "Aleas jacta est",
    :status => "draft"
}, :durability => "soft").run(conn)
```

__Example:__ Replace the document with the primary key `1` and return the values of the document before
and after the replace operation.

```rb
r.table("posts").get(1).replace({
    :id => 1,
    :title => "Lorem ipsum",
    :content => "Aleas jacta est",
    :status => "published"
}, :return_vals => true).run(conn)
```

The result will have two fields `old_val` and `new_val`.

```rb
{
    :deleted => 0,
    :errors => 0,
    :inserted => 0,
    :new_val => {
        :id => 1,
        :title => "Lorem ipsum"
        :content => "Aleas jacta est",
        :status => "published",
    },
    :old_val => {
        :id => 1,
        :title => "Lorem ipsum"
        :content => "TODO",
        :status => "draft",
        :author => "William",
    },
    :replaced => 1,
    :skipped => 0,
    :unchanged => 0
}
```
