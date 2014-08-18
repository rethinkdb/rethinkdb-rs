---
layout: api-command
language: Ruby
permalink: api/ruby/downcase/
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

Lowercases a string.

__Example:__

```rb
> r.expr("Sentence about LaTeX.").downcase().run(conn)
"sentence about latex."
```
