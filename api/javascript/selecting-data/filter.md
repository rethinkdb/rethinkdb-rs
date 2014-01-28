---
layout: api-command
language: JavaScript
permalink: api/javascript/filter/
command: filter
io:
    -   - sequence
        - selection
    -   - stream
        - stream
    -   - array
        - array
related_commands:
    get: get/
    getAll: get_all/

---

# Command syntax #

{% apibody %}
sequence.filter(predicate[, {default: false}]) &rarr; selection
stream.filter(predicate[, {default: false}]) &rarr; stream
array.filter(predicate[, {default: false}]) &rarr; array
{% endapibody %}

# Description #

Get all the documents for which the given predicate is true.

`filter` can be called on a sequence, selection, or a field containing an array of
elements. The return type is the same as the type on which the function was called on.

The body of every filter is wrapped in an implicit `.default(false)`, which means that
if a non-existence errors is thrown (when you try to access a field that does not exist
in a document), RethinkDB will just ignore the document.
The `default` value can be changed by passing an object with a `default` field.
Setting this optional argument to `r.error()` will cause any non-existence errors to
return a `RqlRuntimeError`.


__Example:__ Get all the users that are 30 years old.

```js
r.table('users').filter({age: 30}).run(conn, callback)
```

A more general way to write the previous query is to use `r.row`.

```js
r.table('users').filter(r.row("age").eq(30)).run(conn, callback)
```

Here the predicate is `r.row("age").eq(30)`.

- `r.row` refers to the current document
- `r.row("age")` refers to the field `age` of the current document
- `r.row("age").eq(30)` returns `true` if the field `age` is 30


An even more general way to write the same query is to use an anonymous function.
Read the documentation about [r.row](../row/) to know more about the differences
between `r.row` and anonymous functions in ReQL.

```js
r.table('users').filter(function(user) {
    return user("age").eq(30)
}).run(conn, callback)
```


__Example:__ Get all the users that are more than 18 years old.

```js
r.table("users").filter(r.row("age").gt(18)).run(conn, callback)
```


__Example:__ Get all the users that are less than 18 years old and more than 13 years old.

```js
r.table("users").filter(r.row("age").lt(18).and(r.row("age").gt(13))).run(conn, callback)
```


__Example:__ Get all the users that are more than 18 years old or have their parental consent.

```js
r.table("users").filter(r.row("age").lt(18).or(r.row("hasParentalConsent"))).run(conn, callback)
```


__Example:__ Get all the users that are less than 18 years old or whose age is unknown
(field `age` missing).

```js
r.table("users").filter(r.row("age").lt(18), {default: true}).run(conn, callback)
```

__Example:__ Get all the users that are more than 18 years old. Throw an error if a
document is missing the field `age`.

```js
r.table("users").filter(r.row("age").gt(18), {default: r.error()}).run(conn, callback)
```



__Example:__ Select all users who have given their phone number (all the documents
whose field `phoneNumber` is defined and not `null`).

```js
r.table('users').filter(function(user) {
    return user.hasFields('phoneNumber')
}).run(conn, callback)
```

__Example:__ Retrieve all the users who subscribed between January 1st, 2012
(included) and January 1st, 2013 (excluded).


```js
r.table("users").filter(function(user) {
    return user("subscriptionDate").during( r.time(2012, 1, 1, 'Z'), r.time(2013, 1, 1, 'Z') )
}).run( conn, callback)
```

__Example:__ Retrieve all the users who have a gmail account (whose field `email` ends
with `@gmail.com`).


```js
r.table("users").filter(function(user) {
    return user("email").match("@gmail.com$")
}).run( conn, callback)
```

__Example:__ Filter based on the presence of a value in an array.

Suppose the table `users` has the following schema

```js
{
    name: String
    placesVisited: [String]
}
```

Retrieve all the users whose field `placesVisited` contains `France`.

```js
r.table("users").filter(function(user) {
    return user("placesVisited").contains("France")
}).run( conn, callback)
```

__Example:__ Filter based on nested fields.

Suppose we have a table `users` containing documents with the following schema.

```js
{
    id: String
    name: {
        first: String,
        middle: String,
        last: String
    }
}
```

Retrieve all users named "William Adama" (first name "William", last name
"Adama"), with any middle name.


```js
r.table("users").filter({
    name:{
        first: "William",
        last: "Adama"
    }
}).run(conn, callback)
```

If you want an exact match for a field that is an object, you will have to use `r.literal`.

Retrieve all users named "William Adama" (first name "William", last name
"Adama"), and who do not have a middle name.

```js
r.table("users").filter(r.literal({
    name:{
        first: "William",
        last: "Adama"
    }
})).run(conn, callback)
```


The equivalent queries with an anonymous function.

```js
r.table("users").filter(function(user) {
    return user("name")("first").eq("William")
        .and( user("name")("last").eq("Adama") )
}).run(conn, callback)
```

```js
r.table("users").filter(function(user) {
    return user("name").eq({
        first: "William",
        last: "Adama"
    })
}).run(conn, callback)
```
