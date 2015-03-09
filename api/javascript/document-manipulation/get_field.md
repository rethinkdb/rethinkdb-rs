---
layout: api-command
language: JavaScript
permalink: api/javascript/get_field/
command: getField
io:
    -   - sequence
        - sequence
    -   - singleSelection
        - value
    -   - object
        - value
related_commands:
    '() (bracket)': bracket/
    nth: nth/
---

# Command syntax #

{% apibody %}
sequence.getField(attr) &rarr; sequence
singleSelection.getField(attr) &rarr; value
object.getField(attr) &rarr; value
{% endapibody %}

# Description #

Get a single field from an object. If called on a sequence, gets that field from every
object in the sequence, skipping objects that lack it.

__Example:__ What was Iron Man's first appearance in a comic?

```js
r.table('marvel').get('IronMan').getField('firstAppearance').run(conn, callback)
```
