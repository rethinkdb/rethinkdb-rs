---
layout: api-command
language: Java
permalink: api/java/contains/
command: contains
related_commands:
    map: map/
    concat_map: concat_map/
    group: group/
---

# Command syntax #

{% apibody %}
sequence.contains([value | predicate_function, ...]) &rarr; bool
{% endapibody %}

# Description #

When called with values, returns `true` if a sequence contains all the
specified values.  When called with predicate functions, returns `true`
if for each predicate there exists at least one element of the stream
where that predicate returns `true`.

Values and predicates may be mixed freely in the argument list.

__Example:__ Has Iron Man ever fought Superman?

```java
r.table("marvel").get("ironman")("opponents").contains("superman").run(conn);
```


__Example:__ Has Iron Man ever defeated Superman in battle?

```java
r.table("marvel").get("ironman").g("battles").contains(
    battle -> battle.g("winner").eq("ironman").and(
              battle.g("loser").eq("superman"))
).run(conn);
```

__Example:__ Use `contains` with a predicate function to simulate an `or`. Return the Marvel superheroes who live in Detroit, Chicago or Hoboken.

```java
r.table("marvel").filter(
    hero -> r.expr(r.array("Detroit", "Chicago", "Hoboken"))
             .contains(hero.g("city"))

).run(conn);
```
