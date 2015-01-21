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
selection.filter(predicate[, {default: false}]) &rarr; selection
stream.filter(predicate[, {default: false}]) &rarr; stream
array.filter(predicate[, {default: false}]) &rarr; array
{% endapibody %}

# Description #

Return all the elements in a sequence for which the given predicate is true. The return value of `filter` will be the same as the input (sequence, stream, or array). Documents can be filtered in a variety of ways&mdash;ranges, nested values, boolean conditions, and the results of anonymous functions.

The `filter` command wraps predicates in an implicit [.default(false)](/api/javascript/default), so if the predicate tries to access a field that doesn't exist in a given document, that document is not returned. The `default` optional argument sets the value returned for missing fields, rather than `false`. Setting it to `r.error()` will throw an `RqlRuntimeError` when a non-existent field is accessed.

## Basic predicates ##

__Example:__ Get all users that are 30 years old.


```js
r.table('users').filter({age: 30}).run(conn, callback);
```

The predicate `{age: 30}` selects documents in the `users` table with an `age` field whose value is `30`. Documents with an `age` field set to any other value *or* with no `age` field present are skipped.

While the `{field: value}` style of predicate is useful for exact matches, a more general way to write a predicate is to use the [row](/api/javascript/row) command with a comparison operator such as [eq](/api/javascript/eq) or [gt](/api/javascript/gt), or to use an anonymous function that returns `true` or `false`.

```js
r.table('users').filter(r.row("age").eq(30)).run(conn, callback);
```

In this case, the predicate `r.row("age").eq(30)` returns `true` if the field `age` is equal to 30. You can write this predicate as an anonymous function instead:

```js
r.table('users').filter(function (user) {
    return user("age").eq(30);
}).run(conn, callback);
```

Predicates to `filter` are evaluated on the server, and must use ReQL expressions. You cannot use standard JavaScript comparison operators such as `==`, `<`/`>` and `||`/`&&`.

Also, predicates must evaluate document fields. They cannot evaluate [secondary indexes](/docs/secondary-indexes/).

__Example:__ Get all users that are more than 18 years old.

```js
r.table("users").filter(r.row("age").gt(18)).run(conn, callback)
```


__Example:__ Get all users that are less than 18 years old and more than 13 years old.

```js
r.table("users").filter(
    r.row("age").lt(18).and(r.row("age").gt(13))
).run(conn, callback);
```


__Example:__ Get all the users that are more than 18 years old or have their parental consent.

```js
r.table("users").filter(
    r.row("age").lt(18).or(r.row("hasParentalConsent"))
).run(conn, callback);
```

## More complex predicates ##

__Example:__ Retrieve all the users who subscribed between January 1st, 2012
(included) and January 1st, 2013 (excluded).

```js
r.table("users").filter(function (user) {
    return user("subscriptionDate").during(
        r.time(2012, 1, 1, 'Z'), r.time(2013, 1, 1, 'Z'));
}).run(conn, callback);
```

__Example:__ Retrieve all the users who have a gmail account (whose field `email` ends with `@gmail.com`).

```js
r.table("users").filter(function (user) {
    return user("email").match("@gmail.com$");
}).run(conn, callback);
```

__Example:__ Filter based on the presence of a value in an array.

Given this schema for the `users` table:

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

Given this schema for the `users` table:

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
    name: {
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
    name: {
        first: "William",
        last: "Adama"
    }
})).run(conn, callback)
```

You may rewrite these with anonymous functions.

```js
r.table("users").filter(function(user) {
    return user("name")("first").eq("William")
        .and(user("name")("last").eq("Adama"));
}).run(conn, callback);

r.table("users").filter(function(user) {
    return user("name").eq({
        first: "William",
        last: "Adama"
    });
}).run(conn, callback);
```

## Handling missing fields ##

By default, documents missing fields tested by the `filter` predicate are skipped. In the previous examples, users without an `age` field are not returned. By passing the optional `default` argument to `filter`, you can change this behavior.

__Example:__ Get all users less than 18 years old or whose `age` field is missing.

```js
r.table("users").filter(
    r.row("age").lt(18), {default: true}
).run(conn, callback);
```

__Example:__ Get all users more than 18 years old. Throw an error if a
document is missing the field `age`.

```js
r.table("users").filter(
    r.row("age").gt(18), {default: r.error()}
).run(conn, callback);
```

__Example:__ Get all users who have given their phone number (all the documents whose field `phoneNumber` exists and not `null`).

```js
r.table('users').filter(function (user) {
    return user.hasFields('phoneNumber');
}).run(conn, callback);
```

__Example:__ Get all users with an "editor" role or an "admin" privilege.

```js
r.table('users').filter(function (user) {
    return (user('role').eq('editor').default(false).
        or(user('privilege').eq('admin').default(false)));
}).run(conn, callback);
```

Instead of using the `default` optional argument to `filter`, we have to use default values on the fields within the `or` clause. Why? If the field on the left side of the `or` clause is missing from a document&mdash;in this case, if the user doesn't have a `role` field&mdash;the predicate will generate an error, and will return `false` (or the value the `default` argument is set to) without evaluating the right side of the `or`. By using `.default(false)` on the fields, each side of the `or` will evaluate to either the field's value or `false` if the field doesn't exist.
