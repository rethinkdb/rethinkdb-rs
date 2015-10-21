---
layout: api-command
language: Java
permalink: api/java/merge/
command: merge
related_commands:
    pluck: pluck/
    without: without/
    map: map/
---

# Command syntax #

{% apibody %}
singleSelection.merge([object | function, object | function, ...]) &rarr; object
object.merge([object | function, object | function, ...]) &rarr; object
sequence.merge([object | function, object | function, ...]) &rarr; stream
array.merge([object | function, object | function, ...]) &rarr; array
{% endapibody %}

# Description #

Merge two or more objects together to construct a new object with properties from all. When there is a conflict between field names, preference is given to fields in the rightmost object in the argument list. `merge` also accepts a subquery function that returns an object, which will be used similarly to a [map](/api/java/map/) function.

__Example:__ Equip Thor for battle.

```java
r.table("marvel").get("thor")
 .merge(r.table("equipment").get("hammer"),
        r.table("equipment").get("pimento_sandwich"))
 .run(conn);
```

__Example:__ Equip every hero for battle, using a subquery function to retrieve their weapons.

```java
r.table("marvel").merge(
    hero -> r.hashMap("weapons", r.table("weapons").get(hero.g("weapon_id")))
).run(conn);
```

__Example:__ Use `merge` to join each blog post with its comments.

Note that the sequence being merged&mdash;in this example, the comments&mdash;must be coerced from a selection to an array. Without `coerceTo` the operation will throw an error ("Expected type DATUM but found SELECTION").

```java
r.table("posts").merge(
    post -> r.hashMap("comments", r.table("comments").getAll(post.g("id"))
                      .optArg("index", "post_id").coerceTo("array"))
).run(conn);
```

__Example:__ Merge can be used recursively to modify object within objects.

```java
r.expr(r.hashMap("weapons", r.hashMap("spectacular graviton beam",
    r.hashMap("dmg", 10).with("cooldown", 20))))
 .merge(r.hashMap("weapons", r.hashMap("spectacular graviton beam",
    r.hashMap("dmg", 10))))
 .run(conn);
```


__Example:__ To replace a nested object with another object you can use the [literal](/api/java/literal) term.

```java
r.expr(r.hashMap("weapons", r.hashMap("spectacular graviton beam",
    r.hashMap("dmg", 10).with("cooldown", 20))))
 .merge(r.hashMap("weapons", r.literal(r.hashMap("repulsor rays",
    r.hashMap("dmg", 3).with("cooldown", 0)))))
 .run(conn);
```


__Example:__ `literal` can be used to remove keys from an object as well.

```java
r.expr(r.hashMap("weapons", r.hashMap("spectacular graviton beam",
    r.hashMap("dmg", 10).with("cooldown", 20))))
 .merge(r.hashMap("weapons", r.hashMap("spectacular graviton beam",
    r.literal())))
 .run(conn);
```
