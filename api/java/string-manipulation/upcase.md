---
layout: api-command
language: Java
permalink: api/java/upcase/
command: upcase
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

```java
r.expr("Sentence about LaTeX.").upcase().run(conn);
```

Result:

```
"SENTENCE ABOUT LATEX."
```

__Note:__ `upcase` and `downcase` only affect ASCII characters.
