---
layout: api-command
language: Ruby
permalink: api/ruby/has_fields/
command: has_fields
related_commands:
    '[] (get_field)': get_field/
    with_fields: with_fields/
---

# Command syntax #

{% apibody %}
sequence.has_fields([selector1, selector2...]) &rarr; stream
array.has_fields([selector1, selector2...]) &rarr; array
singleSelection.has_fields([selector1, selector2...]) &rarr; boolean
object.has_fields([selector1, selector2...]) &rarr; boolean
{% endapibody %}

# Description #

Test if an object has all of the specified fields. An object has a field if it has the
specified key and that key maps to a non-nil value. For instance, the object
`{:a => 1, :b => 2, :c => nil}` has the fields `a` and `b`.


__Example:__ Which heroes are married?

```rb
r.table('marvel').has_fields(:spouse).run(conn)
```

__Example:__ Test if a single object has a field.

```rb
r.table('marvel').get("IronMan").has_fields(:spouse).run(conn)
```


__Example:__ You can also test if nested fields exist to get only spouses with powers of their own.

```rb
r.table('marvel').has_fields({:spouse => {:powers => true}}).run(conn)
```

__Example:__ The nested syntax can quickly get verbose so there's a shorthand.

```rb
r.table('marvel').has_fields({:spouse => :powers}).run(conn)
```

