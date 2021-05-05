# mobc-reql

ReQL connection pool implementation

```
use mobc_reql::{GetSession, Pool, SessionManager};

// Create the session manager
let mut manager = SessionManager::new(Default::default());

// Pull the rest of your nodes from your cluster. The connection pool
// connects to the node with the lowest latency.
// It is optional but highly recommended. This way, your app will
// continue working even when nodes go up and down.
manager.discover_hosts().await?;

// Create the pool
let pool = Pool::builder().max_open(20).build(manager);

// Get a session from the pool
let session = pool.session().await?;

// You can pass a reference of the session to run.
// This allows you to use the same underlying connection for multiple
// queries as long as none of them is a change-feed.
// You can even use just one connection for your entire app, even running
// the queries concurrently.
//
// Change feeds on the other hand, require dedicated connections,
// so for each changefeed you need to grab a new session from the pool.
r.expr("Hello world!").run(&session);
```
