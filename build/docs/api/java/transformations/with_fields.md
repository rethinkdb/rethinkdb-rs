---
layout: api-command
language: Java
permalink: api/java/with_fields/
command: withFields
related_commands:
    pluck: pluck/
    hasFields: has_fields/
---

# Command syntax #

{% apibody %}
sequence.withFields([selector1, selector2...]) &rarr; stream
array.withFields([selector1, selector2...]) &rarr; array
{% endapibody %}

# Description #

Plucks one or more attributes from a sequence of objects, filtering out any objects in the sequence that do not have the specified fields. Functionally, this is identical to [hasFields](/api/java/has_fields/) followed by [pluck](/api/java/pluck/) on a sequence.

__Example:__ Get a list of users and their posts, excluding any users who have not made any posts.

Existing table structure:

```json
[
    { "id": 1, "user": "bob", "email": "bob@foo.com", "posts": [ 1, 4, 5 ] },
    { "id": 2, "user": "george", "email": "george@foo.com" },
    { "id": 3, "user": "jane", "email": "jane@foo.com", "posts": [ 2, 3, 6 ] }
]
```

Command and output:

```java
r.table("users").withFields("id", "user", "posts").run(conn);

// Result passed to callback
[
    { "id": 1, "user": "bob", "posts": [ 1, 4, 5 ] },
    { "id": 3, "user": "jane", "posts": [ 2, 3, 6 ] }
]
```

__Example:__ Use the [nested field syntax](/docs/nested-fields/) to get a list of users with cell phone numbers in their contacts.

```java
r.table("users").withFields("id", "user",
    r.hashMap("contact", r.hashMap("phone", "work"))
).run(conn);
```
