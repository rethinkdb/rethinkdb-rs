---
layout: api-command
language: Java
permalink: api/java/match/
command: match
io:
    -   - string
        - object
---

# Command syntax #

{% apibody %}
string.match(regexp) &rarr; null/object
{% endapibody %}

<img src="/assets/images/docs/api_illustrations/match.png" class="api_command_illustration" />

# Description #

Match a string against a regular expression. If there is a match, returns an object with the fields:

- `str`: The matched string
- `start`: The matched string's start
- `end`: The matched string's end
- `groups`: The capture groups defined with parentheses

If no match is found, returns `null`.

<!-- break -->

Accepts [RE2 syntax][re2]. You can enable case-insensitive matching by prefixing the regular expression with `(?i)`. See the linked RE2 documentation for more flags.

[re2]: https://github.com/google/re2/wiki/Syntax

The `match` command does not support backreferences.

__Example:__ Get all users whose name starts with "A". Because `null` evaluates to `false` in
[filter](/api/java/filter/), you can use the result of `match` for the predicate.


```java
r.table("users").filter(doc -> doc.g("name").match("^A")).run(conn);
```

__Example:__ Get all users whose name ends with "n."

```java
r.table("users").filter(doc -> doc.g("name").match("n$")).run(conn);
```
__Example:__ Get all users whose name contains "li."

```java
r.table("users").filter(doc -> doc.g("name").match("li")).run(conn);
```

__Example:__ Get all users whose name is "John," performing a case-insensitive search.

```java
r.table("users").filter(doc -> doc.g("name").match("(?i)^john$")).run(conn);
```

__Example:__ Retrieve the domain of a basic email.

```java
r.expr("name@domain.com").match(".*@(.*)").run(conn);
```

Result:

```json
{
    "start": 0,
    "end": 20,
    "str": "name@domain.com",
    "groups": [
        {
            "end": 17,
            "start": 7,
            "str": "domain.com"
        }
    ]
}
```

You can then retrieve only the domain with [g()](/api/java/get_field) and [nth](/api/java/nth).

```java
r.expr("name@domain.com").match(".*@(.*)").g("groups").nth(0)
 .g("str").run(conn);
```

Returns `domain.com`.


__Example:__ A failure to parse out the domain name will return `null`.

```java
r.expr("name[at]domain.com").match(".*@(.*)").run(conn);
```
