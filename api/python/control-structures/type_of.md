---
layout: api-command
language: Python
permalink: api/python/type_of/
command: type_of
---

# Command syntax #

{% apibody %}
any.type_of() &rarr; string
{% endapibody %}

# Description #

Gets the type of a ReQL query's return value.

The type will be returned as a string:

* `ARRAY`
* `BOOL`
* `DB`
* `FUNCTION`
* `GROUPED_DATA`
* `GROUPED_STREAM`
* `MAXVAL`
* `MINVAL`
* `NULL`
* `NUMBER`
* `OBJECT`
* `PTYPE<BINARY>`
* `PTYPE<GEOMETRY>`
* `PTYPE<TIME>`
* `SELECTION<ARRAY>`
* `SELECTION<OBJECT>`
* `SELECTION<STREAM>`
* `STREAM`
* `STRING`
* `TABLE_SLICE`
* `TABLE`

Read the article on [ReQL data types](/docs/data-types/) for a more detailed discussion. Note that some possible return values from `type_of` are internal values, such as `MAXVAL`, and unlikely to be returned from queries in standard practice.

__Example:__ Get the type of a string.

```py
> r.expr("foo").type_of().run(conn)
"STRING"
```


