---
layout: api-command
language: Ruby
permalink: api/ruby/downcase/
command: downcase
---

# Command syntax #

{% apibody %}
string.downcase() &rarr; string
{% endapibody %}

# Description #

Downcases a string.

__Example:__

```rb
> r("Sentence about LaTeX.").downcase().run(conn)
"sentence about latex."
```
