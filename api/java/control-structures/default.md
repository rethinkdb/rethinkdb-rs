---
layout: api-command
language: Java
permalink: api/java/default/
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

Provide a default value in case of non-existence errors. The `default` command evaluates its first argument (the value it's chained to). If that argument returns `null` or a non-existence error is thrown in evaluation, then `default` returns its second argument. The second argument is usually a default value, but it can be a function that returns a value.

__Example:__ Suppose we want to retrieve the titles and authors of the table `posts`.
In the case where the author field is missing or `null`, we want to retrieve the string
`Anonymous`.

```java
r.table("posts").map(post ->
    r.hashMap("title", post.g("title"))
     .with("author", post.g("author").default_("Anonymous"))
).run(conn);
```

We can rewrite the previous query with `r.branch` too.

```java
r.table("posts").map(post ->
    r.branch(
        post.hasFields("author"),
        r.hashMap("title", post.g("title"))
         .with("author", post.g("author")),
        r.hashMap("title", post.g("title"))
         .with("author", "Anonymous")
    )
).run(conn);
```


__Example:__ The `default` command can also be used to filter documents. Suppose we want to retrieve all our users who are not grown-ups or whose age is unknown (i.e., the field `age` is missing or equals `null`). We can do it with this query:

```java
r.table("users").filter(
    user -> user.g("age").lt(18).default_(true)
).run(conn);
```

One more way to write the previous query is to set the age to be `-1` when the
field is missing.

```java
r.table("users").filter(
    user -> user.g("age").default_(-1).lt(18)
).run(conn);
```

Another way to do the same query is to use [hasFields](/api/java/has_fields/).

```java
r.table("users").filter(
    user -> user.hasFields("age").not().or(user.g("age").lt(18))
).run(conn);
```

The body of every [filter](/api/java/filter/) is wrapped in an implicit `.default_(false)`. You can overwrite
the value `false` with the `default` [optArg](/api/java/optarg) to `filter`, so the previous query can also be
written like this.

```java
r.table("users").filter(
    user -> user.g("age").lt(18).default_(true)
).optArg("default", true).run(conn);

```

