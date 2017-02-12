---
layout: documentation
title: Storing binary objects
docs_active: storing-binary
permalink: docs/storing-binary/javascript/
alias: docs/storing-binary/
switcher: true
language: JavaScript
---

RethinkDB supports a native binary object type, letting you use ReQL to store binary objects directly in the database. The ReQL driver will transparently translate between the ReQL type and Node.js [Buffers](http://nodejs.org/api/buffer.html).

{% infobox %}
__Note:__ The binary object type is meant for data that cannot be reliably stored as UTF-8 strings, such as uploaded files. If you're working with data that *can* be stored as strings, it's usually easier to stick to the string data type.
{% endinfobox %}

For these examples, we'll assume that the RethinkDB connection is available as `conn`.

# Storing uploaded files in the database

It's a common task for web applications to accept file uploads from users; with RethinkDB you can store these files directly in the database.

```js
var fs = require('fs');

function saveFile(filePath, saveName, userId, callback) {
  fs.readFile(filePath, function(err, contents) {
    if (err) return callback(err);
    r.table('files').insert({
      userId: userId,
      filename: saveName,
      file: contents // contents is a buffer, so we do not need to wrap it in `r.binary`
    }).run(conn, callback)
  }
}
```

In `saveFile`, we pass a path to the uploaded file (which may be in a temporary storage directory, even with a temporary name depending on the uploading library we've used), the name to save the file with, and the id of the user who's uploaded the file. The [binary](/api/javascript/binary) ReQL command is used to store the file's contents as a binary object in the `file` field.

```js
function getUserFileIDs(userId, callback) {
  r.table('files').filter({userId: userId}).pluck('id', 'filename').run(conn, callback)
}

function getFile(fileId, callback) {
  r.table('files').get(fileId).pluck('file'​, 'filename').run(conn, callback)
}
```

Then, there are two functions for retrieving files: one to retrieve a directory of a user's uploaded files (`getUserFileIDs`) and one to retrieve the actual file itself (`getFile`). We don't have to use `binary` again here; the ReQL driver will return the proper data type for the `file` field in our object.

# Storing user avatars

Here's another, more fun example: adding [Gravatar](https://en.gravatar.com/site/implement/images/) avatars to user accounts. We can use [http](/api/javascript/http) to retrieve them.

```js
// https://www.npmjs.org/package/MD5
var md5 = require('MD5');

function addGravatar(userId, callback) {
  r.table('users').get(userId)('email').​run(conn, function (err, email) {
    if (err) return callback(err);
    hash = md5(email);
    gravatarUrl = 'http://gravatar.com/avatar/' + hash + '?d=retro';
    r.table('users').get(userId).update({
      gravatar: r.http(gravatarUrl, {resultFormat: 'binary'})
    }).run(conn, callback)
  }
}
```

Where's `r.binary`? You don't need it in this case, because `r.http` will return a binary object with the `{resultFormat: 'binary'}` option. (If the MIME type on the sending server is set correctly, you can even leave that off, and `r.http` will figure out the correct type.)
