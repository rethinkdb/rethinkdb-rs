{% infobox %}
__Before you start:__ make sure you've [installed RethinkDB][install]&mdash;it
should only take a minute!

[install]: /install
{% endinfobox %}

<img src="/assets/images/docs/api_illustrations/quickstart.png" class="api_command_illustration" />

# Start the server #

First, start the RethinkDB server. Under OS X or Linux, do this from a terminal window.

    $ rethinkdb
    info: Creating directory 'rethinkdb_data'
    info: Listening for intracluster connections on port 29015
    info: Listening for client driver connections on port 28015
    info: Listening for administrative HTTP connections on port 8080
    info: Server ready

From Windows, do this from a command prompt window. Use the `cd` command to go to the directory that you unpacked `rethinkdb.exe` in.

    C:\Users\Slava\>cd RethinkDB
    C:\Users\Slava\RethinkDB\>

Then, start RethinkDB with its default options.

    C:\Users\Slava\RethinkDB\>rethinkdb.exe
    info: Creating directory 'rethinkdb_data'
    info: Listening for intracluster connections on port 29015
    info: Listening for client driver connections on port 28015
    info: Listening for administrative HTTP connections on port 8080
    info: Server ready

Point your browser to `localhost:8080`. You'll see an administrative UI
where you can control the cluster (which so far consists of one server), and
play with the query language.

# Run some queries #

Click on the _Data Explorer_ tab in the browser. You can manipulate
data using JavaScript straight from your browser. By default,
RethinkDB creates a database named `test`. Let's create a table:

```javascript
r.db('test').tableCreate('tv_shows')
```

Use the "Run" button or Shift+Enter to run the query. Now, let's insert some JSON documents into the table:

```javascript
r.table('tv_shows').insert([{ name: 'Star Trek TNG', episodes: 178 },
                            { name: 'Battlestar Galactica', episodes: 75 }])
```

We've just inserted two rows into the `tv_shows` table. Let's verify the
number of rows inserted:

```javascript
r.table('tv_shows').count()
```

Finally, let's do a slightly more sophisticated query. Let's find all
shows with more than 100 episodes.

```javascript
r.table('tv_shows').filter(r.row('episodes').gt(100))
```

As a result, we of course get the best science fiction show in
existence.

# Next steps #

Congrats, you're on your way to database bliss! Now move on to the
[ten-minute guide](/docs/guide/javascript) and learn how to use the
client drivers, get more in-depth information on basic commands, and
start writing real applications with RethinkDB.
