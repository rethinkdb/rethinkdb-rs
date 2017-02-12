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
value.default(default_value | function) &rarr; any
sequence.default(default_value | function) &rarr; any
{% endapibody %}

# Description #

Provide a default value in case of non-existence errors. The `default` command evaluates its first argument (the value it's chained to). If that argument returns `null` or a non-existence error is thrown in evaluation, then `default` returns its second argument. The second argument is usually a default value, but it can be a function that returns a value.

__Example:__ Retrieve the titles and authors of the table `posts`.
In the case where the author field is missing or `null`, we want to retrieve the string
`Anonymous`.

```js
r.table("posts").map(function (post) {
    return {
        title: post("title"),
        author: post("author").default("Anonymous")
    }
}).run(conn, callback);
```

<!-- stop -->

We can rewrite the previous query with `r.branch` too.

```js
r.table("posts").map(function (post) {
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
}).run(conn, callback);
```

__Example:__ The `default` command can also be used to filter documents. Retrieve all our users who are not grown-ups or whose age is unknown
(i.e., the field `age` is missing or equals `null`).

```js
r.table("users").filter(function (user) {
    return user("age").lt(18).default(true)
}).run(conn, callback);
```

One more way to write the previous query is to set the age to be `-1` when the
field is missing.

```js
r.table("users").filter(function (user) {
    return user("age").default(-1).lt(18)
}).run(conn, callback);
```

This can be accomplished with [hasFields](/api/javascript/has_fields/) rather than `default`.

```js
r.table("users").filter(function (user) {
    return user.hasFields("age").not().or(user("age").lt(18))
}).run(conn, callback);
```

The body of every [filter](/api/javascript/filter/) is wrapped in an implicit `.default(false)`. You can overwrite the value `false` with the `default` option.

```js
r.table("users").filter(function (user) {
    return user("age").lt(18)
}, {default: true} ).run(conn, callback);
```

__Example:__ The function form of `default` receives the error message as its argument.

```js
r.table("posts").map(function (post) {
    return {
        title: post("title"),
        author: post("author").default(function (err) {
            return err;
        })
    }
}).run(conn, callback);
```

This particular example simply returns the error message, so it isn't very useful. But it would be possible to change the default value based on the specific error message thrown.
