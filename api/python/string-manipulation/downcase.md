---
layout: api-command
language: Python
permalink: api/python/downcase/
command: downcase
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

```py
> r.expr("Sentence about LaTeX.").downcase().run(conn)
"sentence about latex."
```
