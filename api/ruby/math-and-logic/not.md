---
layout: api-command
language: Ruby
permalink: api/ruby/not/
command: not
related_commands:
    'eq': eq/
    'ne': ne/
---

# Command syntax #

{% apibody %}
bool.not() &rarr; bool
not(bool) &rarr; bool
{% endapibody %}

# Description #
Compute the logical inverse (not) of an expression.

`not` can be called either postfix-style, immediately after an expression that evaluates as a boolean value, or infix-style, passing the expression as a parameter to `not`.

__Example:__ Not true is false.

```ruby
r.expr(true).not().run(conn)
r.not(true).run(conn)
```

These evaluate to `false`.

__Example:__ Return all the users that do not have a "flag" field.

```ruby
r.table('users').filter{ |user|
    user.has_fields('flag').not()
}.run(conn)
```

__Example:__ As above, but infix style.

```ruby
r.table('users').filter{ |user|
    r.not(user.has_fields('flag'))
}.run(conn)
```
