---
layout: api-command 
language: Python
permalink: api/python/has_fields/
command: has_fields
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/document-manipulation/has_fields.md
---

{% apibody %}
sequence.has_fields([selector1, selector2...]) → stream
array.has_fields([selector1, selector2...]) → array
singleSelection.has_fields([selector1, selector2...]) → boolean
object.has_fields([selector1, selector2...]) → boolean
{% endapibody %}

Test if an object has all of the specified fields. An object has a field if it has the
specified key and that key maps to a non-null value. For instance, the object
`{'a':1,'b':2,'c':null}` has the fields `a` and `b`.

__Example:__ Which heroes are married?

```py
r.table('marvel').has_fields('spouse').run(conn)
```


__Example:__ Test if a single object has a field.

```py
r.table('marvel').get("IronMan").has_fields('spouse').run(conn)
```


__Example:__ You can also test if nested fields exist to get only spouses with powers of their own.

```py
r.table('marvel').has_fields({'spouse' : {'powers' : True}}).run(conn)
```


__Example:__ The nested syntax can quickly get verbose so there's a shorthand.

```py
r.table('marvel').has_fields({'spouse' : 'powers'}).run(conn)
```

