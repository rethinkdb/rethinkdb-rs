---
layout: api-command
language: Ruby
permalink: api/ruby/to_json/
command: to_json
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

```rb
> r.table('hero').get(1).to_json()

'{"id": 1, "name": "Batman", "city": "Gotham", "powers": ["martial arts", "cinematic entrances"]}'
```
