---
layout: api-command
language: Ruby
permalink: api/ruby/literal/
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

Replace an object in a field instead of merging it with an existing object in a `merge` or `update` operation.

__Example:__ Replace one nested document with another rather than merging the fields.

Assume your users table has this structure:

```js
[
    {
        "id": 1,
        "name": "Alice",
        data: {
            "age": 18,
            "city": "Dallas"
        }
    }       
    ...
]
```

Using `update` to modify the `data` field will normally merge the nested documents:

```rb
r.table('users').get(1).update({ :data => { :age => 19, :job => 'Engineer' } }).run(conn)
```

Result:

```rb
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

```rb
r.table('users').get(1).update({ :data => r.literal({ :age => 19, :job => 'Engineer' }) }).run(conn)
```

Result:

```rb
{
    "id": 1,
    "name": "Alice",
    "data": {
        "age": 19,
        "job": "Engineer"
    }
}       
```
