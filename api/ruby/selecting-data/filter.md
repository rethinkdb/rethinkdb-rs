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
sequence.filter(predicate) &rarr; selection
stream.filter(predicate) &rarr; stream
array.filter(predicate) &rarr; array
{% endapibody %}

# Description #

Get all the documents for which the given predicate is true.

`filter` can be called on a sequence, selection, or a field containing an array of
elements. The return type is the same as the type on which the function was called on.

The body of every filter is wrapped in an implicit `.default(false)`, which means that
if a non-existence errors is thrown (when you try to access a field that does not exist
in a document), RethinkDB will just ignore the document.
The `default` value can be changed by passing the symbol `default`.
Setting this optional argument to `r.error()` will cause any non-existence errors to
return a `RqlRuntimeError`.


__Example:__ Get all the users that are 30 years old.

```rb
r.table('users').filter({:age => 30}).run(conn)
```

A more general way to write the previous query is to use Ruby's block.

```rb
r.table('users').filter{ |user|
    user["age"].eq(30)
}.run(conn)
```

Here the predicate is `user["age"].eq(30)`.

- `user` refers to the current document
- `user["age"]` refers to the field `age` of the current document
- `user["age"].eq(30)` returns `true` if the field `age` is 30



__Example:__ Get all the users that are more than 18 years old.

```rb
r.table("users").filter{ |user|
    user["age"] > 18
}.run(conn)
```

__Example:__ Get all the users that are less than 18 years old or whose age is unknown
(field `age` missing).

```rb
r.table("users").filter(
    lambda { |user| user["age"] < 18 },
    :default => true
).run(conn)
```

__Example:__ Get all the users that are more than 18 years old. Throw an error if a
document is missing the field `age`.

```rb
r.table("users").filter(
    lambda { |user| user["age"] > 18 },
    :default => r.error()
).run(conn)
```


__Example:__ Select all users who have given their phone number (all the documents
whose field `phone_number` is defined and not `None`).

```rb
r.table('users').filter{ |user|
    user.has_fields('phone_number')
}.run(conn)
```

__Example:__ Retrieve all the users who subscribed between January 1st, 2012
(included) and January 1st, 2013 (excluded).

```rb
r.table("users").filter{ |user|
    user["subscription_date"].during( r.time(2012, 1, 1, 'Z'), r.time(2013, 1, 1, 'Z') )
}.run(conn)
```


__Exmaple:__ Retrieve all the users who have a gmail account (whose field `email` ends
with `@gmail.com`).


```rb
r.table("users").filter{ |user|
    user["email"].match("@gmail.com$")
}.run(conn)
```

__Exmaple:__ Filter based on the presence of a value in an array.

Suppose the table `users` has the following schema

```rb
{
    :name => String
    :places_visited => [String]
}
```

Retrieve all the users whose field `places_visited` contains `France`.

```rb
r.table("users").filter{ |user|
    user["places_visited"].contains("France")
}.run(conn)
```

__Example:__ Filter based on nested fields.

Suppose we have a table `users` containing documents with the following schema.

```rb
{
    "id": String
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
    :name =>{
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


The equivalent queries with a lambda function.

```rb
r.table("users").filter{ |user|
    (user["name"]["first"].eq("William")) &
    (user["name"]["last"].eq("Adama"))
}.run(conn)
```

```rb
r.table("users").filter{ |user|
    user["name"].eq(r.literal({
        :first => "William",
        :last => "Adama"
    }))
}.run(conn)
```
