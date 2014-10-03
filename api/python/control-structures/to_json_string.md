---
layout: api-command
language: Python
permalink: api/python/to_json_string/
command: 'to_json_string, to_json'
related_commands:
    json: json/
---
# Command syntax #

{% apibody %}
value.to_json_string() &rarr; string
value.to_json() &rarr; string
{% endapibody %}

# Description #

Convert a ReQL value or object to a JSON string. You may use either `to_json_string` or `to_json`.

__Example:__ Get a ReQL document as a JSON string.

```py
> r.table('hero').get(1).to_json()

'{"id": 1, "name": "Batman", "city": "Gotham", "powers": ["martial arts", "cinematic entrances"]}'
```
