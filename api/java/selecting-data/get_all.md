---
layout: api-command
language: Java
permalink: api/java/get_all/
command: getAll
related_commands:
    get: get/
    between: between/
---

# Command syntax #

{% apibody %}
table.getAll(key[, key2...]) &rarr; selection
{% endapibody %}

<img src="/assets/images/docs/api_illustrations/get-all.png" class="api_command_illustration" />

# Description #

Get all documents where the given value matches the value of the requested index.

__Example:__ Secondary index keys are not guaranteed to be unique so we cannot query via [get](/api/java/get/) when using a secondary index.

```java
r.table("marvel").getAll("man_of_steel").optArg("index", "code_name").run(conn);
```

__Example:__ Without an index argument, we default to the primary index. While `get` will either return the document or `null` when no document with such a primary key value exists, this will return either a one or zero length stream.

```java
r.table("dc").getAll("superman").run(conn);
```

__Example:__ You can get multiple documents in a single call to `get_all`.

```java
r.table("dc").getAll("superman", "ant man").run(conn);
```

__Example:__ You can use [args](/api/java/args/) with `getAll` to retrieve multiple documents whose keys are in a list. This uses `getAll` to get a list of female superheroes, coerces that to an array, and then gets a list of villains who have those superheroes as enemies.

```java
r.do(
    r.table("heroes").getAll("f").optArg("index", "gender")
        .g("id").coerceTo("array"),
    heroines -> r.table("villains").getAll(r.args(heroines))
).run(conn);
```

Secondary indexes can be used in extremely powerful ways with `getAll` and other commands; read the full article on [secondary indexes](/docs/secondary-indexes) for examples using boolean operations, `contains` and more.
