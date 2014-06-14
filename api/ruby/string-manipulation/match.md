---
layout: api-command
language: Ruby
permalink: api/ruby/match/
command: match
related_commands:
    upcase: upcase/
    downcase: downcase/
    split: split/
---

# Command syntax #

{% apibody %}
string.match(regexp) &rarr; nil/object
{% endapibody %}

<img src="/assets/images/docs/api_illustrations/match.png" class="api_command_illustration" />

# Description #


Matches against a regular expression. If there is a match, returns an object with the fields:

- `str`: The matched string
- `start`: The matched string's start
- `end`: The matched string's end
- `groups`: The capture groups defined with parentheses

If no match is found, returns `nil`.

Accepts RE2 syntax
([https://code.google.com/p/re2/wiki/Syntax](https://code.google.com/p/re2/wiki/Syntax)).
You can enable case-insensitive matching by prefixing the regular expression with
`(?i)`. See the linked RE2 documentation for more flags.

The `match` command does not support backreferences.

__Example:__ Get all users whose name starts with "A". Because `nil` evaluates to `false` in
`filter`, you can just use the result of `match` for the predicate.

```rb
r.table('users').filter{ |doc|
    doc['name'].match("^A")
}.run(conn)
```

__Example:__ Get all users whose name ends with "n".

```rb
r.table('users').filter{ |doc|
    doc['name'].match("n$")
}.run(conn)
```

__Example:__ Get all users whose name has "li" in it

```rb
r.table('users').filter{ |doc|
    doc['name'].match("li")
}.run(conn)
```

__Example:__ Get all users whose name is "John" with a case-insensitive search.

```rb
r.table('users').filter{ |doc|
    doc['name'].match("(?i)^john$")
}.run(conn)
```

__Example:__ Get all users whose name is composed of only characters between "a" and "z".

```rb
r.table('users').filter{ |doc|
    doc['name'].match("(?i)^[a-z]+$")
}.run(conn)
```

__Example:__ Get all users where the zipcode is a string of 5 digits.

```rb
r.table('users').filter{ |doc|
    doc['zipcode'].match("\\d{5}")
|.run(conn)
```


__Example:__ Retrieve the domain of a basic email

```rb
r.expr("name@domain.com").match(".*@(.*)").run(conn)
```

Result:

```rb
{
    :start => 0,
    :end => 20,
    :str => "name@domain.com",
    :groups => [
        {
            :end => 17,
            :start => 7,
            :str => "domain.com"
        }
    ]
}
```

You can then retrieve only the domain with the [\[\]](/api/ruby/get_field) selector.

```rb
r.expr("name@domain.com").match(".*@(.*)")["groups"][0]["str"].run(conn)
```

Returns `'domain.com'`


__Example:__ Fail to parse out the domain and returns `nil`.

```rb
r.expr("name[at]domain.com").match(".*@(.*)").run(conn)
```
