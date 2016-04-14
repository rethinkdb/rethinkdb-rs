---
layout: documentation
title: Permissions and user accounts
docs_active: permissions-and-accounts
permalink: docs/permissions-and-accounts/
---

{% toctag %}

RethinkDB controls access to clusters through a system based around **users, permissions,** and **scopes.** Together, these allow you to specify fine grained control for reading, writing and administrative access down to a per-table level.

# Users

A *user* in RethinkDB is similar to users in most other database systems; a database administrator may have a user account, and client applications may be given user accounts. These are unrelated to user accounts that may be implemented within the application.

Users are created by [inserting][ins] documents into the `users` [system table][st]. Every user has an account name in the `id` field, and an optional password.

[ins]: /api/javascript/insert
[st]:  /docs/system-tables/

```js
r.db('rethinkdb').table('users').insert({id: 'bob', password: 'secret'})
```

If you read this document back, you'll get this:

```json
{
    "id": "bob",
    "password": true
}
```

The `password` field is simply a boolean indicating whether a password is set or not. There is no way to read a password from the database.

You can [update][up] the password to a new value, or remove it by setting it to `false`.

[up]: /api/javascript/update

```js
r.db('rethinkdb').table('users').get('bob').update({password: false})
```

You cannot change a username once it's been created. You can, however, [delete][dl] users from the table.

[dl]: /api/javascript/delete

## Password hashing iterations

By default, RethinkDB will use 4096 iterations for hashing passwords during the connection handshake between client drivers and the server. There is an option to set iterations on a per-account basis by setting passwords to an object of the form `{password: "password", iterations: 4096}`. If you wished to use only 1024 iterations, you could set a password like so:

```js
r.db('rethinkdb').table('users').insert({id: 'bob', password: {password: 'secret', iterations: 1024}})
```

Note that you will not be able to read the `iterations` value for an account; as it's stored in the password field, it remains read-only.

The value for `iterations` is a tradeoff between performance and security against brute force attacks. If connections are slow, consider lowering the number of iterations. Raising the number of iterations will make it harder to use a brute force attack, but will increase the CPU usage on clients while establishing a connection.

## The admin user

A new RethinkDB cluster always has one user named `admin`; this user always has all permissions at a global scope, and the user cannot be deleted. By default, the `admin` user has no password. You can change this by updating the `admin` user document, or by specifying the `--initial-password` [command line option][cli] on startup.

[cli]: /docs/cli-options/

The web administration UI always connects as if it were the `admin` user, and skips the authentication process (i.e., the password is not used for this connection). While the web UI cannot be password-protected, you can limit the addresses it will accept connections on using the `--bind-http` command line option. For more details on this, review [Secure your cluster][sec].

[sec]: /docs/security/#securing-the-web-interface

If you forget the admin password, it can be changed from the Data Explorer using `update` as described above.

# Permissions

There are four different permissions that can be granted to a user:

* `read` allows reading the data in tables.
* `write` allows modifying data, including inserting, replacing/updating, and deleting.
* `connect` allows a user to open HTTP connections via the [http][] command. (Restricting this offers security against an exploit in your code being used to circumvent firewall restrictions.)
* `config` allows users different abilities, depending on its scope:
    * __table__ scope allows creating and dropping [secondary indexes][si] on a table, as well as changing the table's cluster configuration (commands such as `reconfigure` and `rebalance`).
    * __database__ scope allows the ability to create and drop tables, in addition to the above.
    * __global__ scope allows the ability to create and drop databases, in addition to the above. (However, a user must have `config` permissions for the tables within a database to drop them, which might not be the case if their `config` permissions are overridden at a table level; see [Scopes](#scopes) below.)

[si]: /docs/secondary-indexes/
[http]: /api/javascript/http

Permissions are stored in the `permissions` system table. While you can change permissions by modifying documents within that table, it's far more convenient to use the [grant](#the-grant-command) command; see below.

# Scopes

The `read`, `write` and `config` permissions can be specified on three scopes, from most granular to least:

* table (affecting a table only)
* database (affecting a database and the tables within)
* global (affecting all databases and the tables within)

Permissions specified at a lower level will override permissions set at a higher level: a user could be granted read and write access to the `field_notes` database, but denied the ability to write to the `calendar` table and to either read or write to the `supervisor_only` table.


    User: notesapp
        database "field_notes" { read: true, write: true, config: false }
            table "calendar" { write: false }
            table "supervisor_only" { read: false, write: false }

The `calendar` table inherits `read: true` from the database level, but specifies `write: false` to make the table ready-only for `notesapp`. The `supervisor_only` table overrides both read and write access. The `notesapp` account has read and write access to all other tables in the `field_notes` database, but no ability to create and drop indexes or change any table's cluster configuration.

# The grant command

The ReQL [grant][gr] command is used to grant and revoke permissions for users. The scope is selected by chaining `grant` after `db` (for database scope), `table` (for table scope), or calling it directly (for global scope).

[gr]: /api/javascript/grant

    r.grant("user", {permissions}) → object
    table.grant("user", {permissions}) → object
    db.grant("user", {permissions}) → object

To specify the permissions described above for Bob, you would execute the following ReQL commands:

```js
// set database scope
r.db('field_notes').grant('bob', {read: true, write: true, config: false});

// set table scopes
r.db('field_notes').table('calendar').grant('bob', {write: false});
r.db('field_notes').table('supervisor_only').grant('bob', {read: false, write: false});
```

# For more information

API documentation for `grant`:

* [JavaScript](/api/javascript/grant)
* [Python](/api/python/grant)
* [Ruby](/api/ruby/grant)
* [Java](/api/java/grant)

Also, read about:

* [System tables][st]
* [Securing your cluster](/docs/security)
