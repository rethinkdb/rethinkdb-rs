---
layout: api-command 
language: JavaScript
permalink: api/javascript/with_fields/
command: withFields
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/transformations/withFields.md
io:
    -   - sequence
        - stream
    -   - array
        - array
related_commands:
    map: map/
    concatMap: concat_map/
---

# Command syntax #

{% apibody %}
sequence.withFields([selector1, selector2...]) &rarr; stream
array.withFields([selector1, selector2...]) &rarr; array
{% endapibody %}

# Description #

Takes a sequence of objects and a list of fields. If any objects in the sequence don't
have all of the specified fields, they're dropped from the sequence. The remaining
objects have the specified fields plucked out. (This is identical to `has_fields`
followed by `pluck` on a sequence.)

__Example:__ Get a list of heroes and their nemeses, excluding any heroes that lack one.

```js
r.table('marvel').withFields('id', 'nemesis')
```

__Example:__ Get a list of heroes and their nemeses, excluding any heroes whose nemesis isn't in an evil organization.

```js
r.table('marvel').withFields('id', {'nemesis' : {'evil_organization' : true}})
```

__Example:__ The nested syntax can quickly become overly verbose so there's a shorthand.

```js
r.table('marvel').withFields('id', {'nemesis' : 'evil_organization'})
```

