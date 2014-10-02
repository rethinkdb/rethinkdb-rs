---
layout: api-command
language: JavaScript
permalink: api/javascript/to_json/
command: to_json
io:
    -   - any
        - string
related_commands:
    json: json/
---
# Command syntax #

{% apibody %}
any.to_json(value) &rarr; string
{% endapibody %}

# Description #

Convert a ReQL value or object to a JSON string.

__Example:__ Get a ReQL document as a JSON string.

```js
r.table('hero').get(1).to_json()
// result returned to callback
'{"id": 1, "name": "Batman", "city": "Gotham", "powers": ["martial arts", "cinematic entrances"]}'
```
