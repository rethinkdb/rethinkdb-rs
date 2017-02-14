---
layout: api-command
language: JavaScript
permalink: api/javascript/type_of/
command: typeOf
io:
    -   - any
        - string
---

# Command syntax #

{% apibody %}
any.typeOf() &rarr; string
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

Read the article on [ReQL data types](/docs/data-types/) for a more detailed discussion. Note that some possible return values from `typeOf` are internal values, such as `MAXVAL`, and unlikely to be returned from queries in standard practice.

__Example:__ Get the type of a string.

```javascript
r.expr("foo").typeOf().run(conn, callback);
// Result passed to callback
"STRING"
```
