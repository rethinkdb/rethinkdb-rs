---
layout: api-command
language: JavaScript
permalink: api/javascript/default/
command: default
io:
    -   - value
        - any
    -   - sequence
        - any
---

# Command syntax #

{% apibody %}
value.default(default_value) &rarr; any
sequence.default(default_value) &rarr; any
{% endapibody %}

# Description #

Handle non-existence errors. Tries to evaluate and return its first argument. If an
error related to the absence of a value is thrown in the process, or if its first
argument returns `null`, returns its second argument. (Alternatively, the second argument
may be a function which will be called with either the text of the non-existence error
or `null`.)


__Exmple:__ Suppose we want to retrieve the titles and authors of the table `posts`.
In the case where the author field is missing or `null`, we want to retrieve the string
`Anonymous`.

```js
r.table("posts").map( function(post) {
    return {
        title: post("title"),
        author: post("author").default("Anonymous")
    }
}).run(conn, callback)
```

We can rewrite the previous query with `r.branch` too.

```js
r.table("posts").map( function(post) {
    return r.branch(
        post.hasFields("author"),
        {
            title: post("title"),
            author: post("author")
        },
        {
            title: post("title"),
            author: "Anonymous" 
        }
    )
}).run(conn, callback)
```


__Example:__ The `default` command can be useful to filter documents too. Suppose
we want to retrieve all our users who are not grown-ups or whose age is unknown
(i.e the field `age` is missing or equals `null`). We can do it with this query:

```js
r.table("users").filter( function(user) {
    return user("age").lt(18).default(true)
}).run(conn, callback)
```

One more way to write the previous query is to set the age to be `-1` when the
field is missing.

```js
r.table("users").filter( function(user) {
    return user("age").default(-1).lt(18)
}).run(conn, callback)
```

Another way to do the same query is to use `hasFields`.

```js
r.table("users").filter( function(user) {
    return user.hasFields("age").not().or(user("age").lt(18))
}).run(conn, callback)
```

The body of every `filter` is wrapped in an implicit `.default(false)`. You can overwrite
the value `false` by passing an option in filter, so the previous query can also be
written like this.

```js
r.table("users").filter( function(user) {
    return user("age").lt(18)
}, {default: true} ).run(conn, callback)
```

