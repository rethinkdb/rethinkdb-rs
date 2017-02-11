---
layout: api-command
language: Java
permalink: api/java/pluck/
command: pluck
related_commands:
    without: without/
    map: map/
---

# Command syntax #

{% apibody %}
sequence.pluck([selector1, selector2...]) &rarr; stream
array.pluck([selector1, selector2...]) &rarr; array
object.pluck([selector1, selector2...]) &rarr; object
singleSelection.pluck([selector1, selector2...]) &rarr; object
{% endapibody %}

<img src="/assets/images/docs/api_illustrations/pluck.png" class="api_command_illustration" />

# Description #

Plucks out one or more attributes from either an object or a sequence of objects
(projection).

__Example:__ We just need information about IronMan's reactor and not the rest of the
document.

```java
r.table("marvel").get("IronMan").pluck("reactorState", "reactorPower").run(conn);
```


__Example:__ For the hero beauty contest we only care about certain qualities.

```java
r.table("marvel").pluck("beauty", "muscleTone", "charm").run(conn);
```


__Example:__ Pluck can also be used on nested objects.

```java
// JSON equivalent:
//   { "abilities": { "damage": true, "mana_cost": true }, "weapons": true }
r.table("marvel").pluck(
    r.hashMap("abilities",
        r.hashMap("damage", true).with("mana_cost", true))
    .with("weapons", true)
).run(conn);
```


__Example:__ The nested syntax can quickly become overly verbose, so there's a shorthand for it.

```java
// JSON equivalent:
//   { "abilities": [ "damage", "mana cost" ] }, "weapons"
r.table("marvel")
 .pluck(r.hashMap("abilities", r.array("damage", "mana_cost")), "weapons")
 .run(conn);
```

For more information read the [nested field documentation](/docs/nested-fields/).
