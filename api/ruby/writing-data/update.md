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
table.update(json | expr[, :durability => "hard", :return_changes => false, :non_atomic => false])
    &rarr; object
selection.update(json | expr[, :durability => "hard", :return_changes => false, :non_atomic => false])
    &rarr; object
singleSelection.update(json | expr[, :durability => "hard", :return_changes => false, :non_atomic => false])
    &rarr; object
{% endapibody %}


# Description #

Update JSON documents in a table. Accepts a JSON document, a ReQL expression, or a
combination of the two.

The optional arguments are:

- `durability`: possible values are `hard` and `soft`. This option will override the
table or query's durability setting (set in [run](/api/ruby/run/)).  
In soft durability mode RethinkDB will acknowledge the write immediately after
receiving it, but before the write has been committed to disk.
- `return_changes`: if set to `true`, return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made.
- `non_atomic`: set to `true` if you want to perform non-atomic updates (updates that
require fetching data from another document).


Update returns an object that contains the following attributes:

- `replaced`: the number of documents that were updated.
- `unchanged`: the number of documents that would have been modified except the new
value was the same as the old value.
- `skipped`: the number of documents that were skipped because the document didn't exist.
- `errors`: the number of errors encountered while performing the update.
- `first_error`: If errors were encountered, contains the text of the first error.
- `deleted` and `inserted`: 0 for an update operation.
- `changes`: if `return_changes` is set to `true`, this will be an array of objects, one for each objected affected by the `delete` operation. Each object will have two keys: `{:new_val => <new value>, :old_val => <old value>}`.


__Example:__ Update the status of the post with `id` of `1` to `published`.

```rb
r.table("posts").get(1).update({:status => "published"}).run(conn)
```

__Example:__ Update the status of all posts to `published`.

```rb
r.table("posts").update({:status => "published"}).run(conn)
```

__Example:__ Update the status of all the post written by William.

```rb
r.table("posts").filter({:author => "William"}).update({:status => "published"}).run(conn)
```


__Example:__ Increment the field `view` with `id` of `1`.
This query will throw an error if the field `views` doesn't exist.

```rb
r.table("posts").get(1).update{ |post|
    {:views => post["views"]+1}
}.run(conn)
```

__Example:__ Increment the field `view` of the post with `id` of `1`.
If the field `views` does not exist, it will be set to `0`.

```rb
r.table("posts").update{ |post|
    {:views => (post["views"]+1).default(0)}
}.run(conn)
```

__Example:__ Perform a conditional update.  
If the post has more than 100 views, set the `type` of a post to `hot`, else set it to `normal`.

```rb
r.table("posts").get(1).update{ |post|
    r.branch(
        post["views"] > 100,
        {:type => "hot"},
        {:type => "normal"}
    )
}.run(conn)
```

__Example:__ Update the field `num_comments` with the result of a sub-query. Because
this update is not atomic, you must pass the `non_atomic` flag.

```rb
r.table("posts").get(1).update({
    :num_comments => r.table("comments").filter({:id_post => 1}).count()
}, :non_atomic => true ).run(conn)
```

If you forget to specify the `non_atomic` flag, you will get a `RqlRuntimeError`.

```
RqlRuntimeError: Could not prove function deterministic.  Maybe you want to use the non_atomic flag? 
```

__Example:__ Update the field `num_comments` with a random value between 0 and 100.  
This update cannot be proven deterministic because of `r.js` (and in fact is not), so you
must pass the `non_atomic` flag.

```rb
r.table("posts").get(1).update({
    :num_comments => r.js("Math.floor(Math.random()*100)")
}, :non_atomic => true).run(conn)
```

__Example:__ Update the status of the post with `id` of `1` using soft durability.

```rb
r.table("posts").get(1).update({:status => "published"}, :durability => "soft").run(conn)
```

__Example:__ Increment the field `views` and return the values of the document before
and after the update operation.

```rb
r.table("posts").get(1).update(:return_changes => true) { |post|
    :views => post["views"]+1
}.run(conn)
```

The result will have two fields `old_val` and `new_val`.

```rb
{
    :deleted => 1,
    :errors => 0,
    :inserted => 0,
    :changes => [
        {
            :new_val => {
                :id => 1,
                :author => "Julius_Caesar",
                :title => "Commentarii de Bello Gallico",
                :content => "Aleas jacta est",
                :views => 207
            },
            :old_val => {
                :id => 1,
                :author => "Julius_Caesar",
                :title => "Commentarii de Bello Gallico",
                :content => "Aleas jacta est",
                :views => 206
            }
        }
    ],
    :replaced => 0,
    :skipped => 0,
    :unchanged => 0
}
```

