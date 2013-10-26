---
layout: api-command 
language: Ruby
permalink: api/ruby/default/
command: default 
---

# Command syntax #

{% apibody %}
value.default(default_value) &rarr; any
sequence.default(default_value) &rarr; any
{% endapibody %}

# Description #

Handle non-existence errors. Tries to evaluate and return its first argument. If an
error related to the absence of a value is thrown in the process, or if its first
argument returns null, returns its second argument. (Alternatively, the second argument
may be a function which will be called with either the text of the non-existence error
or null.)

__Example:__ Suppose we want to retrieve the titles and authors of the table `posts`.
In case the author field is missing or `None`, we want to retrieve the string `Anonymous`.

```rb
r.table("posts").map{ |post|
    {
        :title => post["title"],
        :author => post["author"].default("Anonymous")
    }
}.run(conn)
```

We can rewrite the previous query with `r.branch` too.

```rb
r.table("posts").map{ |post|
    r.branch(
        post.has_fields("author") & (post["author"].ne(nil)),
        {
            :title => post["title"],
            :author => post["author"]
        },
        {
            :title => post["title"],
            :author => "Anonymous" 
        }
    )
}.run(conn)
```


__Example:__ The `default` command can be useful to filter documents too. Suppose
we want to retrieve all our users who are not grown-ups or whose age is unknown
(i.e the field `age` is missing or equals to `nil`). We can do it with this query:

```rb
r.table("users").filter{ |user|
    (user["age"] < 18).default(true)
}.run(conn)
```

One more way to write the previous query is to set the age to be `-1` in case the
field is missing.

```rb
r.table("users").filter{ |user|
    user["age"].default(-1) < 18
}.run(conn)
```

One last way to do the same query is to use `has_fields`.

```rb
r.table("users").filter{ |user|
    user.has_fields("age").not() | (user["age"].eq(nil)) | (user["age"] < 18)
}.run(conn)
```

The body of every `filter` is wrapped in an implicit `.default(false)`. You can overwrite
the value `false` by passing an option in filter, so the previous query can also be
written like this.

```rb
r.table("users").filter{ |user|
    (user["age"] < 18)
, :default => true}.run(conn)
```

