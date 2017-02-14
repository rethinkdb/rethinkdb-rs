---
layout: api-command
language: JavaScript
permalink: api/javascript/upcase/
command: upcase
io:
    -   - string
        - string
related_commands:
    downcase: downcase/
    match: match/
    split: split/
---

# Command syntax #

{% apibody %}
string.upcase() &rarr; string
{% endapibody %}

# Description #

Uppercases a string.

__Example:__

```javascript
r.expr("Sentence about LaTeX.").upcase().run(conn, callback)
```

Result:

```javascript
"SENTENCE ABOUT LATEX."
```

__Note:__ `upcase` and `downcase` only affect ASCII characters.
