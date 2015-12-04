---
layout: documentation
title: Storing binary objects
docs_active: storing-binary
permalink: docs/storing-binary/java/
alias: docs/storing-binary/
switcher: true
language: Java
---

{% infobox alert %}
**This document has not been fully updated for Java.** The [API documentation](/api/java) for Java is complete, but many ReQL articles still have examples in other languages. We'll be updating each article after the Java driver is officially released.
{% endinfobox %}

RethinkDB supports a native binary object type, letting you use ReQL to store binary objects directly in the database. The ReQL driver will transparently translate between the ReQL type and [bytes](https://docs.oracle.com/javase/tutorial/java/nutsandbolts/datatypes.html).

For these examples, we'll assume that the RethinkDB connection is available in class scope as `conn`.

# Storing uploaded files in the database

It's a common task for web applications to accept file uploads from users; with RethinkDB you can store these files directly in the database.

```java
import java.nio.file.*;

/**
 * Store a file in the database.
 *
 * @param filePath  the path to the file to save
 * @param saveName  the filename to store with the file in the database
 * @param userId    user ID value
 */
public void saveFile(String[] filePath, String[] saveName, long userId) {
    byte[] fileData = Files.readAllBytes(filePath);
    r.table("files").insert(
        r.hashMap("userId", userId)
         .with("filename", saveName)
         .with("file", r.binary(fileData))
    ).run(conn);
}
```

In `saveFile`, we pass a path to the uploaded file (which may be in a temporary storage directory, even with a temporary name depending on the uploading library we've used), the name to save the file with, and the id of the user who's uploaded the file. The [binary](/api/java/binary) ReQL command is used to store the file's contents as a binary object in the `file` field.

```py
def get_user_file_ids(user_id):
    """
    Retrieve the IDs of previously-saved files for a user as a list of
    dicts: [{'id': x, 'filename': y}, ...]
    """
    return r.table('files').filter({'user_id': user_id}).pluck(
        'id', 'filename').run(conn)

def get_file(file_id):
    """
    Retrieve a file by its ID. Returns a dict with 'filename' and 'file'
    keys.
    """
    return r.table('files').get(file_id).pluck('file', 'filename').run(conn)
```

Then, there are two functions for retrieving files: one to retrieve a directory of a user's uploaded files (`get_user_file_ids`) and one to retrieve the actual file itself (`get_file`). We don't have to use `binary` again here; the ReQL driver will return the proper data type for the `file` field in our object.

# Storing user avatars

Here's another, more fun example: adding [Gravatar](https://en.gravatar.com/site/implement/images/) avatars to user accounts. We can use [http](/api/python/http) to retrieve them.

```py
import hashlib

def add_gravatar(user_id):
    """
    Add a gravatar field with the binary avatar icon to user accounts if they
    have an avatar associated with their email address.
    """
    email = r.table('users').get(user_id)['email'].run(conn)
    hash = hashlib.md5(email).hexdigest()
    gravatar_url = 'http://www.gravatar.com/avatar/' + hash + '?d=retro'
    r.table('users').get(user_id).update({
        'gravatar': r.http(gravatar_url, result_format='binary')}).run(conn)
```

Where's `r.binary`? You don't need it in this case, because `r.http` will return a binary object with the `result_format='binary'` option. (If the MIME type on the sending server is set correctly, you can even leave that off, and `r.http` will figure out the correct type.)
