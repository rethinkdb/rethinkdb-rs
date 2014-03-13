---
layout: api-command
language: JavaScript
permalink: api/javascript/downcase/
command: downcase
io:
    -   - string
        - array
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

Downcases a string.

__Example:__

```rb
> r.expr("Sentence about LaTeX.").downcase().run(conn, callback)
"sentence about latex."
```
