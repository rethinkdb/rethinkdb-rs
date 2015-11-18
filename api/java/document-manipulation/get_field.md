---
layout: api-command
language: Java
permalink: api/java/get_field/
command: getField, g
related_commands:
    bracket: bracket/
    nth: nth/
---

# Command syntax #

{% apibody %}
sequence.g(attr) &rarr; sequence
singleSelection.g(attr) &rarr; value
object.g(attr) &rarr; value
{% endapibody %}

# Description #

Get a single field from an object. If called on a sequence, gets that field from every
object in the sequence, skipping objects that lack it.

You may use either `getField` or its shorthand, `g`.

__Example:__ What was Iron Man's first appearance in a comic?

```java
r.table("marvel").get("IronMan").g("firstAppearance").run(conn);
```
