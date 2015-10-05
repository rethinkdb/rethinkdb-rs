---
layout: api-command
language: Java
permalink: api/java/to_json_string/
command: 'toJsonString, toJSON'
related_commands:
    json: json/
---
# Command syntax #

{% apibody %}
value.toJsonString() &rarr; string
value.toJSON() &rarr; string
{% endapibody %}

# Description #

Convert a ReQL value or object to a JSON string. You may use either `toJsonString` or `toJSON`.

__Example:__ Get a ReQL document as a JSON string.

```java
> r.table('hero').get(1).toJSON()
// result returned to callback
'{"id": 1, "name": "Batman", "city": "Gotham", "powers": ["martial arts", "cinematic entrances"]}'
```
