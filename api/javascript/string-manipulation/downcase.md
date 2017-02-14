---
layout: api-command
language: JavaScript
permalink: api/javascript/downcase/
command: downcase
io:
    -   - string
        - string
related_commands:
    upcase: upcase/
    match: match/
    split: split/
---

# Command syntax #

{% apibody %}
string.downcase() &rarr; string
{% endapibody %}

# Description #

Lowercases a string.

__Example:__

```javascript
r.expr("Sentence about LaTeX.").downcase().run(conn, callback)
```

Result:

```javascript
"sentence about latex."
```

__Note:__ `upcase` and `downcase` only affect ASCII characters.
