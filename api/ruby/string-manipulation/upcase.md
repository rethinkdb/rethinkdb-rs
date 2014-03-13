---
layout: api-command
language: Ruby
permalink: api/ruby/upcase/
command: upcase
---

# Command syntax #

{% apibody %}
string.upcase() &rarr; string
{% endapibody %}

# Description #

Upcases a string.

__Example:__

```rb
> r("Sentence about LaTeX.").upcase().run(conn)
"SENTENCE ABOUT LATEX."
```
