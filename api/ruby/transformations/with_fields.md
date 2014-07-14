---
layout: api-command
language: Ruby
permalink: api/ruby/with_fields/
command: with_fields
related_commands:
    has_fields: has_fields/
    pluck: pluck/
    without: without/
---

# Command syntax #

{% apibody %}
sequence.with_fields([selector1, selector2...]) &rarr; stream
array.with_fields([selector1, selector2...]) &rarr; array
{% endapibody %}

# Description #

Plucks one or more attributes from a sequence of objects, filtering out any objects in the sequence that do not have the specified fields. Functionally, this is identical to `has_fields` followed by `pluck` on a sequence.

__Example:__ Get a list of users and their posts, excluding any users who have not made any posts.

Existing table structure:

```rb
[
    { :id => 1, :user => 'bob', :email => 'bob@foo.com', :posts => [ 1, 4, 5 ] },
    { :id => 2, :user => 'george', :email => 'george@foo.com' },
    { :id => 3, :user => 'jane', :email => 'jane@foo.com', :posts => [ 2, 3, 6 ] }
]
```

Command and output:

```rb
r.table('users').with_fields('id', 'user', 'posts').run(conn)

[
    { :id => 1, :user => 'bob', :posts => [ 1, 4, 5 ] },
    { :id => 3, :user => 'jane', :posts => [ 2, 3, 6 ] }
]
```

__Example:__ Use the [nested field syntax](/docs/nested-fields/) to get a list of users with cell phone numbers in their contacts.

```rb
r.table('users').with_fields('id', 'user', {:contact => {:phone => 'work'}).run(conn)
```
