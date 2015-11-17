---
layout: api-command
language: Java
permalink: api/java/to_json_string/
command: 'toJsonString, toJson'
related_commands:
    json: json/
---
# Command syntax #

{% apibody %}
value.toJsonString() &rarr; string
value.toJson() &rarr; string
{% endapibody %}

# Description #

Convert a ReQL value or object to a JSON string. You may use either `toJsonString` or `toJson`.

__Example:__ Get a ReQL document as a JSON string.

```java
r.table("hero").get(1).toJson().run(conn)
```

Returned data:

```json
'{"id": 1, "name": "Batman", "city": "Gotham", "powers": ["martial arts", "cinematic entrances"]}'
```
