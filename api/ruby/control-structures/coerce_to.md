---
layout: api-command
language: Ruby
permalink: api/ruby/coerce_to/
command: coerce_to
related_commands:
    object: object/
---

# Command syntax #

{% apibody %}
sequence.coerce_to('array') &rarr; array
value.coerce_to('string') &rarr; string
string.coerce_to('number') &rarr; number
array.coerce_to('object') &rarr; object
object.coerce_to('array') &rarr; array
{% endapibody %}

# Description #

Convert a value of one type into another.

* a sequence, selection or object can be coerced to an array
* an array of key-value pairs can be coerced to an object
* a string can be coerced to a number
* any datum (single value) can be coerced to a string

__Example:__ Coerce a stream to an array to store its output in a field. (A stream cannot be stored in a field directly.)

```rb
r.table('posts').map { |post|
    { :comments => r.table('comments').get_all(post['id'], {:index => 'post_id'}).coerce_to('array') }
}.run(conn)
```

__Example:__ Coerce an array of pairs into an object.

```rb
r.expr([['name', 'Ironman'], ['victories', 2000]]).coerce_to('object').run(conn)
```

__Note:__ To coerce a list of key-value pairs like `['name', 'Ironman', 'victories', 2000]` to an object, use the [object](/api/ruby/object) command.

__Example:__ Coerce a number to a string.

```rb
r.expr(1).coerce_to('string').run(conn)
```

