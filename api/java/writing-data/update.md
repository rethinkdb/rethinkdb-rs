---
layout: api-command
language: Java
permalink: api/java/update/
command: update
related_commands:
    insert: insert/
    replace: replace/
    delete: delete/
---


# Command syntax #

{% apibody %}
table.update(object | function) &rarr; object
selection.update(object | function) &rarr; object
singleSelection.update(object | function) &rarr; object
{% endapibody %}

# Description #

Update JSON documents in a table. Accepts a JSON document, a ReQL expression, or a combination of the two.

You can pass the following options using [optArg](/api/java/optarg/):

- `durability`: possible values are `hard` and `soft`. This option will override the table or query's durability setting (set in [run](/api/java/run/)). In soft durability mode RethinkDB will acknowledge the write immediately after receiving it, but before the write has been committed to disk.
- `return_changes`:
    - `true`: return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made, only including the documents actually updated.
    - `false`: do not return a `changes` array (the default).
    - `"always"`: behave as `true`, but include all documents the command tried to update whether or not the update was successful. (This was the behavior of `true` pre-2.0.)
- `nonAtomic`: if set to `true`, executes the update and distributes the result to replicas in a non-atomic fashion. This flag is required to perform non-deterministic updates, such as those that require reading data from another table.

Update returns an object that contains the following attributes:

- `replaced`: the number of documents that were updated.
- `unchanged`: the number of documents that would have been modified except the new value was the same as the old value.
- `skipped`: the number of documents that were skipped because the document didn't exist.
- `errors`: the number of errors encountered while performing the update.
- `first_error`: If errors were encountered, contains the text of the first error.
- `deleted` and `inserted`: 0 for an update operation.
- `changes`: if `returnChanges` is set to `true`, this will be an array of objects, one for each objected affected by the `update` operation. Each object will have two keys: `{new_val: <new value>, old_val: <old value>}`.


__Example:__ Update the status of the post with `id` of `1` to `published`.

```java
r.table("posts").get(1).update(r.hashMap("status", "published")).run(conn);
```

__Example:__ Update the status of all posts to `published`.

```java
r.table("posts").update(r.hashMap("status", "published")).run(conn);
```

__Example:__ Update the status of all the posts written by William.

```java
r.table("posts").filter(
    r.hashMap("author", "William")).update(r.hashMap("status", "published")
).run(conn);
```

__Example:__ Increment the field `view` of the post with `id` of `1`.
This query will throw an error if the field `views` doesn't exist.

```java
r.table("posts").get(1).update(
    post -> r.hashMap("views", post.g("views").add(1))
).run(conn);
```

__Example:__ Increment the field `view` of the post with `id` of `1`.
If the field `views` does not exist, it will be set to `0`.

```java
r.table("posts").get(1).update(
    post -> r.hashMap("views", post.g("views").add(1).default(0))
).run(conn);
```

__Example:__ Perform a conditional update.  
If the post has more than 100 views, set the `type` of a post to `hot`, else set it to `normal`.

```java
r.table("posts").get(1).update(
    post -> r.branch(post.g("views").gt(100),
                     r.hashMap("type", "hot"),
                     r.hashMap("type", "normal")
    )
).run(conn);
```

__Example:__ Update the field `numComments` with the result of a sub-query. Because this update is not atomic, you must pass the `non_atomic` flag.

```java
r.table("posts").get(1).update(
    r.hashMap("numComments", r.table("comments")
     .filter(r.hashMap("id_post", 1)).count())
).optArg("non_atomic", true).run(conn);
```

If you forget to specify the `non_atomic` flag, you will get a `ReqlRuntimeError`:

```
ReqlRuntimeError: Could not prove function deterministic.  Maybe you want to use the non_atomic flag? 
```

__Example:__ Update the field `numComments` with a random value between 0 and 100. This update cannot be proven deterministic because of `r.js` (and in fact is not), so you must pass the `nonAtomic` flag.

```java
r.table("posts").get(1).update(
    r.hashMap("numComments", r.js("Math.floor(Math.random()*100)"))
).optArg("non_atomic", true).run(conn);
```

__Example:__ Update the status of the post with `id` of `1` using soft durability.

```java
r.table("posts").get(1).update(
    r.hashMap(status, "published")
).optArg("durability", "soft").run(conn);
```

__Example:__ Increment the field `views` and return the values of the document before and after the update operation.

```java
r.table("posts").get(1).update(
    post -> r.hashMap("views", post.g("views").add(1))
).optArg("return_changes", true).run(conn);
```

The result will now include a `changes` field:

```java
{
    "deleted": 1,
    "errors": 0,
    "inserted": 0,
    "changes": [
        {
            "new_val": {
                "id": 1,
                "author": "Julius_Caesar",
                "title": "Commentarii de Bello Gallico",
                "content": "Aleas jacta est",
                "views": 207
            },
            "old_val": {
                "id": 1,
                "author": "Julius_Caesar",
                "title": "Commentarii de Bello Gallico",
                "content": "Aleas jacta est",
                "views": 206
            }
        }
    ],
    "replaced": 0,
    "skipped": 0,
    "unchanged": 0
}
```


## Updating nested fields ##

The `update` command supports RethinkDB's [nested field][nf] syntax to update subdocuments. Consider a user table with contact information in this format:

[nf]: /docs/nested-fields/javascript

```json
{
	"id": 10001,
	"name": "Bob Smith",
	"contact": {
		"phone": {
			"work": "408-555-1212",
			"home": "408-555-1213",
			"cell": "408-555-1214"
		},
		"email": {
			"work": "bob@smith.com",
			"home": "bobsmith@example.com",
			"other": "bobbys@moosecall.net"
		},
		"im": {
			"skype": "Bob Smith",
			"aim": "bobmoose",
			"icq": "nobodyremembersicqnumbers"
		}
	},
	"notes": [
		{
			"date": r.time(2014,1,1,'Z'),
			"from": "John Doe",
			"subject": "My name is even more boring than Bob's"
		},
		{
			"date": r.time(2014,2,2,'Z'),
			"from": "Bob Smith Sr",
			"subject": "Happy Second of February"
		}
	]
}
```

__Example:__ Update Bob Smith's cell phone number.

```java
r.table("users").get(10001).update(
    r.hashMap("contact",
        r.hashMap("phone",
            r.hashMap("cell", "408-555-4242")))
).run(conn);
```

__Example:__ Add another note to Bob Smith's record.

```java
import com.rethinkdb.model.MapObject;

MapObject icqNote = new MapObject()
    .with("date", r.now())
    .with("from", "Inigo Montoya")
    .with("subject", "You killed my father");
r.table("users").get(10001).update(
    row -> r.hashMap("notes", row.g("notes").append(icqNote))
).run(conn);
```

__Example:__ Send a note to every user with an ICQ number.

```java
import com.rethinkdb.model.MapObject;

MapObject icqNote = new MapObject()
    .with("date", r.now())
    .with("from", "Admin")
    .with("subject", "Welcome to the future");
r.table("users").filter(
    row -> row.hasFields(r.hashMap("contact", r.hashMap("im", "icq")))
).update(r.hashMap("notes", row.g("notes").append(icqNote))).run(conn);
```

__Example:__ Replace all of Bob's IM records. Normally, `update` will merge nested documents together; to replace the entire `"im"` document, use the [literal][] command.

[literal]: /api/java/literal/

```java
r.table("users").get(10001).update(
    r.hashMap("contact",
        r.hashMap("im",
            r.literal(r.hashMap("aim", "themoosemeister"))))
).run(conn);
```
