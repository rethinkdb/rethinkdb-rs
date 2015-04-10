---
layout: api-command
language: Ruby
permalink: api/ruby/filter/
command: filter
related_commands:
    get: get/
    get_all: get_all/
    between: between/
---


# Command syntax #

{% apibody %}
selection.filter(predicate[, :default => false]) &rarr; selection
stream.filter(predicate[, :default => false]) &rarr; stream
array.filter(predicate[, :default => false]) &rarr; array

{% endapibody %}

# Description #

Return all the elements in a sequence for which the given predicate is true. The return value of `filter` will be the same as the input (sequence, stream, or array). Documents can be filtered in a variety of ways&mdash;ranges, nested values, boolean conditions, and the results of anonymous functions.

By default, `filter` will silently skip documents with missing fields: if the predicate tries to access a field that doesn't exist (for instance, the predicate `{:age => 30}` applied to a document with no `age` field), that document will not be returned in the result set, and no error will be generated. This behavior can be changed with the `default` optional argument.

* If `default` is set to `true`, documents with missing fields will be returned rather than skipped.
* If `default` is set to `r.error()`, an `RqlRuntimeError` will be thrown when a document with a missing field is tested.
* If `default` is set to `false` (the default), documents with missing fields will be skipped.

## Basic predicates ##

__Example:__ Get all users who are 30 years old.


```rb
r.table('users').filter({:age => 30}).run(conn)
```

The predicate `{'age': 30}` selects documents in the `users` table with an `age` field whose value is `30`. Documents with an `age` field set to any other value *or* with no `age` field present are skipped.

While the `{'field': value}` style of predicate is useful for exact matches, a more general way to write a predicate is to use a Ruby block that returns `true` or `false`.

```rb
r.table('users').filter{ |user|
    user["age"].eq(30)
}.run(conn)
```

In this case, the function returns `true` if the field `age` is equal to 30.

Predicates to `filter` are evaluated on the server, and must use ReQL expressions. Some Ruby comparison operators are overloaded by the RethinkDB driver and will be translated to ReQL, such as `==`, `<`/`>` and `|`/`&` (note the single character form, rather than `||`/`&&`).

Also, predicates must evaluate document fields. They cannot evaluate [secondary indexes](/docs/secondary-indexes/).

__Example:__ Get all users who are more than 18 years old.

```rb
r.table("users").filter{ |user|
    user["age"] > 18
}.run(conn)
```

__Example:__ Get all users who are less than 18 years old and more than 13 years old.

```rb
r.table("users").filter{ |user|
    (user["age"] < 18) & (user["age"] > 13)
}.run(conn)
```

__Example:__ Get all users who are more than 18 years old or have their parental consent.

```rb
r.table("users").filter{ |user|
    (user["age"] >= 18) | (user["hasParentalConsent"])
}.run(conn)
```

## More complex predicates ##

__Example:__ Retrieve all users who subscribed between January 1st, 2012
(included) and January 1st, 2013 (excluded).

```rb
r.table("users").filter{ |user|
    user["subscription_date"].during(r.time(2012, 1, 1, 'Z'),
        r.time(2013, 1, 1, 'Z'))
}.run(conn)
```

__Example:__ Retrieve all users who have a gmail account (whose field `email` ends with `@gmail.com`).

```rb
r.table("users").filter{ |user|
    user["email"].match("@gmail.com$")
}.run(conn)
```

__Example:__ Filter based on the presence of a value in an array.

Given this schema for the `users` table:

```rb
{
    :name => String
    :places_visited => [String]
}
```

Retrieve all users whose field `places_visited` contains `France`.

```rb
r.table("users").filter{|user|
    user["places_visited"].contains("France")
}.run(conn)
```

__Example:__ Filter based on nested fields.

Given this schema for the `users` table:

```rb
{
    :id => String
    :name => {
        :first => String,
        :middle => String,
        :last => String
    }
}
```

Retrieve all users named "William Adama" (first name "William", last name
"Adama"), with any middle name.


```rb
r.table("users").filter({
    :name => {
        :first => "William",
        :last => "Adama"
    }
}).run(conn)
```

If you want an exact match for a field that is an object, you will have to use `r.literal`.

Retrieve all users named "William Adama" (first name "William", last name
"Adama"), and who do not have a middle name.

```rb
r.table("users").filter(r.literal({
    :name => {
        :first => "William",
        :last=> "Adama"
    }
})).run(conn)
```

You may rewrite these with blocks.

```rb
r.table("users").filter{ |user|
    (user["name"]["first"].eq("William")) &
    (user["name"]["last"].eq("Adama"))
}.run(conn)
```

```rb
r.table("users").filter{ |user|
    user["name"].eq({
        :first => "William",
        :last => "Adama"
    })
}.run(conn)
```

## Handling missing fields ##

By default, documents missing fields tested by the `filter` predicate are skipped. In the previous examples, users without an `age` field are not returned. By passing the optional `default` argument to `filter`, you can change this behavior.

__Example:__ Get all users less than 18 years old or whose `age` field is missing.

```rb
r.table("users").filter(:default => true){ |user| user["age"] < 18 }.run(conn)
```

__Example:__ Get all users more than 18 years old. Throw an error if a
document is missing the field `age`.

```rb
r.table("users").filter(:default => r.error()){
    |user| user["age"] > 18
}.run(conn)
```

__Example:__ Get all users who have given their phone number (all the documents whose field `phone_number` exists and is not `nil`).

```rb
r.table('users').filter{ |user|
    user.has_fields('phone_number')
}.run(conn)
```

__Example:__ Get all users with an "editor" role or an "admin" privilege.

```rb
r.table('users').filter{ |user|
    user['role'].eq('editor').default(false) |
        user['privilege'].eq('admin').default(false)
}.run(conn)
```

Instead of using the `default` optional argument to `filter`, we have to use default values on the fields within the `or` clause. Why? If the field on the left side of the `or` clause is missing from a document&mdash;in this case, if the user doesn't have a `role` field&mdash;the predicate will generate an error, and will return `false` (or the value the `default` argument is set to) without evaluating the right side of the `or`. By using `.default(false)` on the fields, each side of the `or` will evaluate to either the field's value or `false` if the field doesn't exist.
