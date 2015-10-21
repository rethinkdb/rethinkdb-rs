---
layout: api-command
language: Java
permalink: api/java/literal/
command: literal
related_commands:
    merge: merge/
    filter: filter/
---
# Command syntax #

{% apibody %}
r.literal(object) &rarr; special
{% endapibody %}

# Description #

Replace an object in a field instead of merging it with an existing object in a `merge` or `update` operation. Using `literal` with no arguments in a `merge` or `update` operation will remove the corresponding field.

__Example:__ Replace one nested document with another rather than merging the fields.

Assume your users table has this structure:

```json
[
    {
        "id": 1,
        "name": "Alice",
        "data": {
            "age": 18,
            "city": "Dallas"
        }
    }       
    ...
]
```

Using `update` to modify the `data` field will normally merge the nested documents:

```java
r.table("users").get(1)
 .update(r.hashMap(data, r.hashMap(age, 19).with(job, "Engineer")))
 .run(conn);

// Result:
{
    "id": 1,
    "name": "Alice",
    "data": {
        "age": 19,
        "city": "Dallas",
        "job": "Engineer"
    }
}       
```

That will preserve `city` and other existing fields. But to replace the entire `data` document with a new object, use `literal`:

```java
r.table("users").get(1)
 .update(r.hashMap(data, r.literal(r.hashMap(age, 19).with(job, "Engineer"))))
 .run(conn);

// Result:
{
    "id": 1,
    "name": "Alice",
    "data": {
        "age": 19,
        "job": "Engineer"
    }
}       
```

__Example:__ Use `literal` to remove a field from a document.

```java
r.table("users").get(1).merge(r.hashMap(data, r.literal())).run(conn);

// Result:
{
    "id": 1,
    "name": "Alice"
}
```
