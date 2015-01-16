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

# Next steps #

{% infobox %}
Move on to the [ten-minute guide](/docs/guide/javascript/) and learn how to use RethinkDB.
{% endinfobox %}
