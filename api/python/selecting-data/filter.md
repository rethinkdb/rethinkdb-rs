---
layout: api-command
language: Python
permalink: api/python/filter/
command: filter
related_commands:
    get: get/
    get_all: get_all/
    between: between/
---

# Command syntax #

{% apibody %}
selection.filter(predicate[, default=False]) &rarr; selection
stream.filter(predicate[, default=False]) &rarr; stream
array.filter(predicate[, default=False]) &rarr; array
{% endapibody %}

# Description #

Get all the documents for which the given predicate is true.

`filter` can be called on a sequence, selection, or a field containing an array of
elements. The return type is the same as the type on which the function was called on.

The body of every filter is wrapped in an implicit `.default(False)`, which means that
if a non-existence errors is thrown (when you try to access a field that does not exist
in a document), RethinkDB will just ignore the document.
The `default` value can be changed by passing the named argument `default`.
Setting this optional argument to `r.error()` will cause any non-existence errors to
return a `RqlRuntimeError`.


__Example:__ Get all the users that are 30 years old.

```py
r.table('users').filter({"age": 30}).run(conn)
```

A more general way to write the previous query is to use `r.row`.

```py
r.table('users').filter(r.row["age"] == 30).run(conn)
```

Here the predicate is `r.row["age"] == 30`.

- `r.row` refers to the current document
- `r.row["age"]` refers to the field `age` of the current document
- `r.row["age"] == 30` returns `True` if the field `age` is 30


An even more general way to write the same query is to use a lambda function.
Read the documentation about [r.row](../row/) to know more about the differences
between `r.row` and lambda functions in ReQL.

```py
r.table('users').filter(lambda user:
    user["age"] == 30
).run(conn)
```


__Example:__ Get all the users that are more than 18 years old.

```py
r.table("users").filter(r.row["age"] > 18).run(conn)
```


__Example:__ Get all the users that are less than 18 years old and more than 13 years old.

```py
r.table("users").filter((r.row["age"] < 18) & (r.row["age"] > 13)).run(conn)
```


__Example:__ Get all the users that are more than 18 years old or have their parental consent.

```py
r.table("users").filter((r.row["age"].lt(18)) | (r.row["hasParentalConsent"])).run(conn)
```



__Example:__ Get all the users that are less than 18 years old or whose age is unknown
(field `age` missing).

```py
r.table("users").filter(r.row["age"] < 18, default=True).run(conn)
```

__Example:__ Get all the users that are more than 18 years old. Throw an error if a
document is missing the field `age`.

```py
r.table("users").filter(r.row["age"] > 18, default=r.error()).run(conn)
```



__Example:__ Select all users who have given their phone number (all the documents
whose field `phone_number` is defined and not `None`).

```py
r.table('users').filter(lambda user:
    user.has_fields('phone_number')
).run(conn)
```

__Example:__ Retrieve all the users who subscribed between January 1st, 2012
(included) and January 1st, 2013 (excluded).

```py
r.table("users").filter(lambda user:
    user["subscription_date"].during( r.time(2012, 1, 1, 'Z'), r.time(2013, 1, 1, 'Z') )
).run(conn)
```


__Example:__ Retrieve all the users who have a gmail account (whose field `email` ends
with `@gmail.com`).


```py
r.table("users").filter(lambda user:
    user["email"].match("@gmail.com$")
).run(conn)
```

__Example:__ Filter based on the presence of a value in an array.

Suppose the table `users` has the following schema

```py
{
    "name": <type 'str'>
    "places_visited": [<type 'str'>]
}
```

Retrieve all the users whose field `places_visited` contains `France`.

```py
r.table("users").filter(lambda user:
    user["places_visited"].contains("France")
).run(conn)
```

__Example:__ Filter based on nested fields.

Suppose we have a table `users` containing documents with the following schema.

```py
{
    "id": <type 'str'>
    "name": {
        "first": <type 'str'>,
        "middle": <type 'str'>,
        "last": <type 'str'>
    }
}
```

Retrieve all users named "William Adama" (first name "William", last name
"Adama"), with any middle name.


```py
r.table("users").filter({
    "name":{
        "first": "William",
        "last": "Adama"
    }
}).run(conn)
```

If you want an exact match for a field that is an object, you will have to use `r.literal`.

Retrieve all users named "William Adama" (first name "William", last name
"Adama"), and who do not have a middle name.

```py
r.table("users").filter(r.literal({
    "name":{
        "first": "William",
        "last": "Adama"
    }
})).run(conn)
```


The equivalent queries with a lambda function.

```py
r.table("users").filter(lambda user:
    (user["name"]["first"] == "William")
        & (user["name"]["last"] == "Adama")
).run(conn)
```

```py
r.table("users").filter(lambda user:
    user["name"] == {
        "first": "William",
        "last": "Adama"
    }
).run(conn)
```
