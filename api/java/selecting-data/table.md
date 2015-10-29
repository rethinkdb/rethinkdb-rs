---
layout: api-command
language: Java
permalink: api/java/table/
command: table
related_commands:
    filter: filter/
    get: get/
io:
    -   - db
        - table
---

# Command syntax #

{% apibody %}
db.table(name) &rarr; table
{% endapibody %}

# Description #

Return all documents in a table. Other commands may be chained after `table` to return a subset of documents (such as [get](/api/java/get/) and [filter](/api/java/filter/)) or perform further processing.

__Example:__ Return all documents in the table 'marvel' of the default database.

```java
r.table("marvel").run(conn);
```

__Example:__ Return all documents in the table 'marvel' of the database 'heroes'.

```java
r.db("heroes").table("marvel").run(conn);
```

There are two [optArgs](/api/java/optarg) that may be specified.

* `read_mode`: One of three possible values affecting the consistency guarantee for the table read:
    * `single` returns values that are in memory (but not necessarily written to disk) on the primary replica. This is the default.
    * `majority` will only return values that are safely committed on disk on a majority of replicas. This requires sending a message to every replica on each read, so it is the slowest but most consistent.
    * `outdated` will return values that are in memory on an arbitrarily-selected replica. This is the fastest but least consistent.
* `identifier_format`: possible values are `name` and `uuid`, with a default of `name`. If set to `uuid`, then [system tables](/docs/system-tables/) will refer to servers, databases and tables by UUID rather than name. (This only has an effect when used with system tables.)

__Example:__ Allow potentially out-of-date data in exchange for faster reads.

```java
r.db("heroes").table("marvel").optArg("read_mode", "outdated").run(conn);
```
