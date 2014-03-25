---
layout: api-command
language: Python
permalink: api/python/match/
command: match
---

# Command syntax #

{% apibody %}
string.match(regexp) &rarr; None/object
{% endapibody %}

# Description #

Match against a regular expression. If there is a match, returns an object with the fields:

- `str`: The matched string
- `start`: The matched string's start
- `end`: The matched string's end
- `groups`: The capture groups defined with parentheses

If no match is found, returns `None`.

Accepts RE2 syntax
([https://code.google.com/p/re2/wiki/Syntax](https://code.google.com/p/re2/wiki/Syntax)).
You can enable case-insensitive matching by prefixing the regular expression with
`(?i)`. See the linked RE2 documentation for more flags.

The `match` command does not support backreferences.

__Example:__ Get all users whose name starts with "A". Because `None` evaluates to `false` in
`filter`, you can just use the result of `match` for the predicate.


```py
r.table('users').filter(lambda doc:
    doc['name'].match("^A")
).run(conn)
```

__Example:__ Get all users whose name ends with "n".

```py
r.table('users').filter(lambda doc:
    doc['name'].match("n$")
).run(conn)
```

__Example:__ Get all users whose name has "li" in it

```py
r.table('users').filter(lambda doc:
    doc['name'].match("li")
).run(conn)
```

__Example:__ Get all users whose name is "John" with a case-insensitive search.

```py
r.table('users').filter(lambda doc:
    doc['name'].match("(?i)^john$")
).run(conn)
```

__Example:__ Get all users whose name is composed of only characters between "a" and "z".

```py
r.table('users').filter(lambda doc:
    doc['name'].match("(?i)^[a-z]+$")
).run(conn)
```

__Example:__ Get all users where the zipcode is a string of 5 digits.

```py
r.table('users').filter(lambda doc:
    doc['zipcode'].match("\d{5}")
).run(conn)
```


__Example:__ Retrieve the domain of a basic email

```py
r.expr("name@domain.com").match(".*@(.*)").run(conn)
```

Result:

```py
{
    "start": 0,
    "end": 20,
    "str": "name@domain.com",
    "groups":[
        {
            "end": 17,
            "start": 7,
            "str": "domain.com"
        }
    ]
}
```

You can then retrieve only the domain with the [\(\)](/api/javascript/get_field) selector and [nth](/api/javascript/nth).

```py
r.expr("name@domain.com").match(".*@(.*)")["groups"][0]["str"].run(conn)
```

Returns `'domain.com'`


__Example:__ Fail to parse out the domain and returns `None`.

```py
r.expr("name[at]domain.com").match(".*@(.*)").run(conn)
```
