---
layout: api-command
language: Python
permalink: api/python/default/
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
argument returns `None`, returns its second argument. (Alternatively, the second argument
may be a function which will be called with either the text of the non-existence error
or `None`.)


__Exmple:__ Suppose we want to retrieve the titles and authors of the table `posts`.
In the case where the author field is missing or `None`, we want to retrieve the string
`Anonymous`.

```py
r.table("posts").map(lambda post:
    {
        "title": post["title"],
        "author": post["author"].default("Anonymous")
    }
).run(conn)
```

We can rewrite the previous query with `r.branch` too.

```py
r.table("posts").map(lambda post:
    r.branch(
        post.has_fields("author"),
        {
            "title": post["title"],
            "author": post["author"]
        },
        {
            "title": post["title"],
            "author": "Anonymous" 
        }
    )
).run(conn)
```


__Example:__ The `default` command can be useful to filter documents too. Suppose
we want to retrieve all our users who are not grown-ups or whose age is unknown
(i.e the field `age` is missing or equals `None`). We can do it with this query:

```py
r.table("users").filter(lambda user:
    (user["age"] < 18).default(True)
).run(conn)
```

One more way to write the previous query is to set the age to be `-1` when the
field is missing.

```py
r.table("users").filter(lambda user:
    user["age"].default(-1) < 18
).run(conn)
```

One last way to do the same query is to use `has_fields`.

```py
r.table("users").filter(lambda user:
    user.has_fields("age").not_() | (user["age"] < 18)
).run(conn)
```

The body of every `filter` is wrapped in an implicit `.default(False)`. You can overwrite
the value `False` by passing an option in filter, so the previous query can also be
written like this.

```py
r.table("users").filter(
    lambda user: (user["age"] < 18).default(True),
    default=True
).run(conn)
```

