---
layout: api-command
language: Java
permalink: api/java/bracket/
command: bracket
related_commands:
    row: row/
    nth: nth/
    getField: get_field/
---

# Command syntax #

{% apibody %}
sequence.bracket(attr) &rarr; sequence
singleSelection.bracket(attr) &rarr; value
object.bracket(attr) &rarr; value
array.bracket(index) &rarr; value
{% endapibody %}

# Description #

Get a single field from an object. If called on a sequence, gets that field from every object in the sequence, skipping objects that lack it.

{% infobox %}
Under most circumstances, you'll want to use [getField](/api/java/get_field) (or its shorthand `g`) or [nth](/api/java/nth) rather than `bracket`. The `bracket` term may be useful in situations where you are unsure of the data type returned by the term you are calling `bracket` on.
{% endinfobox %}

__Example:__ What was Iron Man's first appearance in a comic?

```java
r.table("marvel").get("IronMan").bracket("firstAppearance").run(conn);
// more idiomatically:
r.table("marvel").get("IronMan").g("firstAppearance").run(conn);
```

The `()` command also accepts integer arguments as array offsets, like the [nth](/api/java/nth) command.

__Example:__ Get the fourth element in a sequence. (The first element is position `0`, so the fourth element is position `3`.)

```java
r.expr(r.array(10, 20, 30, 40, 50)).bracket(3).run(conn);
// more idiomatically:
r.expr(r.array(10, 20, 30, 40, 50)).nth(3).run(conn);

40
```
