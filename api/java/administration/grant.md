---
layout: api-command
language: Java
permalink: api/java/grant/
command: grant
---

# Command syntax #

{% apibody %}
r.grant("username", r.hashMap("permission", bool[, ...])) &rarr; object
db.grant("username", r.hashMap("permission", bool[, ...])) &rarr; object
table.grant("username", r.hashMap("permission", bool[, ...])) &rarr; object
{% endapibody %}

# Description #

Grant or deny access permissions for a user account, globally or on a per-database or per-table basis.

There are four different permissions that can be granted to an account:

* `read` allows reading the data in tables.
* `write` allows modifying data, including inserting, replacing/updating, and deleting.
* `connect` allows a user to open HTTP connections via the [http][] command. This permission can only be granted in global scope.
* `config` allows users to create/drop [secondary indexes][si] on a table and changing the cluster configuration; to create and drop tables, if granted on a database; and to create and drop databases, if granted globally.

[si]: /docs/secondary-indexes/
[http]: /api/java/http

Permissions may be granted on a global scope, or granted for a specific table or database. The scope is defined by calling `grant` on its own (e.g., `r.grant()`, on a table (`r.table().grant()`), or on a database (`r.db().grant()`).

The `grant` command returns an object of the following form:

```json
{
    "granted": 1,
    "permissions_changes": [
        {
            "new_val": { new permissions },
            "old_val": { original permissions }
        }
    ]
```

The `granted` field will always be `1`, and the `permissions_changes` list will have one object, describing the new permissions values and the old values they were changed from (which may be `null`).

Permissions that are not defined on a local scope will be inherited from the next largest scope. For example, a write operation on a table will first check if `write` permissions are explicitly set to `true` or `false` for that table and account combination; if they are not, the `write` permissions for the database will be used if those are explicitly set; and if neither table nor database permissions are set for that account, the global `write` permissions for that account will be used.

__Note:__ For all accounts other than the special, system-defined `admin` account, permissions that are not explicitly set in any scope will effectively be `false`. When you create a new user account by inserting a record into the [system table][st], that account will have _no_ permissions until they are explicitly granted.

[st]: /docs/system-tables/#users

For a full description of permissions, read [Permissions and user accounts][pa].

[pa]: /docs/permissions-and-accounts/

__Example:__ Grant the `chatapp` user account read and write permissions on the `users` database.

```java
r.db("users").grant("chatapp", r.hashMap("read", true).with("write", true)).run(conn);
```

Return:

```json
{
    "granted": 1,
    "permissions_changes": [
        {
            "new_val": { "read": true, "write": true },
            "old_val": { null }
        }
    ]
```

__Example:__ Deny write permissions from the `chatapp` account for the `admin` table.

```java
r.db("users").table("admin").grant("chatapp", r.hashMap("write", false)).run(conn);
```

This will override the `write: true` permissions granted in the first example, but for this table only. Other tables in the `users` database will inherit from the database permissions.

__Example:__ Delete a table-level permission for the `chatapp` account.

```java
r.db("users").table("admin").grant("chatapp", r.hashMap("write", null)).run(conn);
```

By specifying `None`, the table scope `write` permission is removed, and will again inherit from the next highest scope (database or global).

__Example:__ Grant `chatapp` the ability to use HTTP connections.

```java
r.grant("chatapp", r.hashMap("connect", true)).run(conn);
```

This grant can only be given on a global level.


__Example:__ Grant a `monitor` account read-only access to all databases.

```java
r.grant("monitor", r.hashMap("read", true)).run(conn);
```
