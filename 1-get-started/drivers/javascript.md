---
layout: documentation
title: Installing the JavaScript driver
title_image: /assets/images/docs/driver-languages/javascript.png
active: docs
docs_active: install-drivers
permalink: docs/install-drivers/javascript/
---
{% include install-driver-docs-header.md %}

# Installation #

_Prerequisites:_ The JavaScript driver requires Node.js >= 0.10.0.

Install the driver with npm:

```bash
$ npm install rethinkdb
```

The RethinkDB JavaScript driver has installed successfully if
the output ends with the following lines:

```
npm WARN optional dep failed, continuing node-protobuf@1.0.5
rethinkdb@1.7.0-2 ../../node_modules/rethinkdb
```

# Usage #

You can use the drivers from Node.js like this:

```javascript
$ node
r = require('rethinkdb')
r.connect({ host: 'localhost', port: 28015 }, function(err, conn) {
  if(err) throw err;
  r.db('test').tableCreate('tv_shows').run(conn, function(err, res) {
    if(err) throw err;
    console.log(res);
    r.table('tv_shows').insert({ name: 'Star Trek TNG' }).run(conn, function(err, res)
    {
      if(err) throw err;
      console.log(res);
    });
  });
});
```

# Optional: optimized backend #

During the installation, you can ignore errors similar to this one:

```
  CXX(target) Release/obj.target/protobuf/protobuf.o
../protobuf.cpp:7:10: fatal error: 'google/protobuf/dynamic_message.h' file not found
#include <google/protobuf/dynamic_message.h>
         ^
1 error generated.
make: *** [Release/obj.target/protobuf/protobuf.o] Error 1
gyp ERR! build error
gyp ERR! stack Error: `make` failed with exit code: 2
gyp ERR! stack     at ChildProcess.onExit (/usr/local/Cellar/node/0.10.10/lib/node_modules/npm/node_modules/node-gyp/lib/build.js:267:23)
gyp ERR! stack     at ChildProcess.EventEmitter.emit (events.js:98:17)
gyp ERR! stack     at Process.ChildProcess._handle.onexit (child_process.js:789:12)
gyp ERR! System Darwin 12.4.0
gyp ERR! command "node" "/usr/local/Cellar/node/0.10.10/lib/node_modules/npm/node_modules/node-gyp/bin/node-gyp.js" "rebuild"
gyp ERR! cwd /Users/atnnn/node_modules/rethinkdb/node_modules/node-protobuf
gyp ERR! node -v v0.10.10
gyp ERR! node-gyp -v v0.9.6
gyp ERR! not ok
```

The protobuf errors above indicate that the optimized C++ protobuf backend was
not installed. The driver will use the slower JavaScript backend.

For faster JavaScript driver performance, read about [using an
optimized C++ protobuf backend](/docs/driver-performance/).

# Next steps #

{% include install-driver-docs-footer.md %}

