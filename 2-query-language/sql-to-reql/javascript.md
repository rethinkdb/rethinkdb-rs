---
layout: documentation
title: SQL to ReQL cheat sheet
active: docs
docs_active: sql-to-reql
permalink: docs/sql-to-reql/javascript/
alias: docs/sql-to-reql/
switcher: true
language: JavaScript
---

<img alt="Data Modeling Illustration" class="api_command_illustration"
    src="/assets/images/docs/api_illustrations/SQL-to-ReQL-cheat-sheet.png" />

&nbsp;

# Terminology #

SQL and RethinkDB share very similar terminology. Below is a table
of terms and concepts in the two systems.

<table class="table-2-columns">
    <thead>
        <tr>
            <th>SQL</th>
            <th>RethinkDB</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>database</td>
            <td>database</td>
        </tr>
        <tr>
            <td>table</td>
            <td>table</td>
        </tr>
        <tr>
            <td>row</td>
            <td>document</td>
        </tr>
        <tr>
            <td>column</td>
            <td>field</td>
        </tr>
        <tr>
            <td>table joins</td>
            <td>table joins</td>
        </tr>
        <tr>
            <td>primary key</td>
            <td>primary key (by default <code>id</code>)</td>
        </tr>
        <tr>
            <td>index</td>
            <td>index</td>
        </tr>
    </tbody>
</table>

# INSERT #

This is a list of queries for inserting data into a database.

<table class="table-2-columns">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>
        <tr>
            <td>
<pre>
INSERT INTO users(user_id,
                  age,
                  name)
VALUES ("f62255a8259f",
        30,
        Peter)
</pre>
            </td>
            <td>
<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/insert/">insert</a>({
   userId: "f62255a8259f",
   age: 30,
   name: "Peter"
})
</pre>
            </td>
        </tr>
    </tbody>
</table>

# SELECT #

This is a list of queries for selecting data out of a database.

<table class="table-2-columns">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>
        <tr><td>

<pre>
SELECT * FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users")
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT user_id, name FROM users
</pre>
        </td><td>
<pre>
r.<a href="/api/javascript/table/">table</a>("users")
 .<a href="/api/javascript/pluck/">pluck</a>("userId", "name")
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT * FROM users
WHERE name = "Peter"
</pre>
        </td><td>
<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/filter/">filter</a>({
    name: "Peter"
})
</pre>

<p>An alternative is to use the implicit variable <code>r.row</code> (the currently visited document):</p>
<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/filter/">filter</a>(
    r.<a href="/api/javascript/row/">row</a>("name").<a href="/api/javascript/eq/">eq</a>("Peter")
)
</pre>

<p>Another alternative is to use an anonymous function:</p>
<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/filter/">filter</a>(function(doc) {
    return doc("name").<a href="/api/javascript/eq/">eq</a>("Peter");
})
</pre>

<p>If you have a secondary index built on the field <code>name</code>, you can run a
more efficient query:
<pre>
r.<a href="/api/javascript/table/">table</a>("users")
    .<a href="/api/javascript/get_all/">getAll</a>("Peter", {index: "name"})
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT user_id, name FROM users
WHERE name = "Peter"
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/filter/">filter</a>({
    name: "Peter"
}).<a href="/api/javascript/pluck/">pluck</a>("userId", "name")
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT * FROM users
WHERE name = "Peter"
AND age = 30
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/filter/">filter</a>({
    name: "Peter",
    age: 30
})
</pre>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/filter/">filter</a>(
    (r.<a href="/api/javascript/row/">row</a>("name").<a href="/api/javascript/eq/">eq</a> "Peter")
     <a href="/api/javascript/and/">&</a> (r.<a href="/api/javascript/row/">row</a>("age").<a href="/api/javascript/eq/">eq</a>(30))
)
</pre>

        </td></tr>


        <tr><td>

<pre>
SELECT * FROM users
WHERE age &gt; 30
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/filter/">filter</a>(
    r.<a href="/api/javascript/row/">row</a>("age").<a href="/api/javascript/gt/">gt</a>(30)
)
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT * FROM users
WHERE name LIKE "P%"
</pre>

        </td><td>
<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/filter/">filter</a>(
    r.<a href="/api/javascript/row/">row</a>("name").<a href="/api/javascript/match/">match</a>("^P")}
)
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT * FROM users
WHERE name LIKE "%er"
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/filter/">filter</a>(
    r.<a href="/api/javascript/row/">row</a>("name").<a href="/api/javascript/match/">match</a>("er$")}
)
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT * FROM users
ORDER BY name ASC
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/order_by/">orderBy</a>("name")
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT * FROM users
ORDER BY name DESC
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/order_by/">orderBy</a>(
    r.desc("name")
)
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT user_id FROM users
WHERE name = "Peter"
ORDER BY name DESC
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/filter/">filter</a>({
    name: "Peter"
}).<a href="/api/javascript/order_by/">orderBy</a>(
    r.desc("name")
).<a href="/api/javascript/pluck/">pluck</a>("userId")
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT * FROM users LIMIT 5
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/limit/">limit</a>(5)
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT * FROM users LIMIT 5 SKIP 10
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/skip/">skip</a>(10).<a href="/api/javascript/limit/">limit</a>(5)
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT * FROM users
WHERE name IN ('Peter', 'John')
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/filter/">filter</a>(
  function (doc) {
    return r.<a href="/api/javascript/expr/"</a>expr</a>(["Peter","John"])
            .<a href="/api/javascript/contains">contains</a>(doc("name"));
  }
)
</pre>

<p>If you have a secondary index built on the field <code>name</code>, you can run a
more efficient query:
<pre>
r.<a href="/api/javascript/table/">table</a>("users")
    .<a href="/api/javascript/get_all/">getAll</a>("Peter", "John",
        {index: "name"})
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT * FROM users
WHERE name NOT IN ('Peter', 'John')
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/filter/">filter</a>(
  function (doc) {
    return r.<a href="/api/javascript/expr/"</a>expr</a>(["Peter","John"])
            .<a href="/api/javascript/contains">contains</a>(doc("name"))
            .not();
  }
)
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT COUNT(*) FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/count/">count</a>()
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT COUNT(name) FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/filter/">filter</a>(function (doc) {
  return doc.<a href="/api/javascript/has_fields/">hasFields</a>("name")
}).<a href="/api/javascript/count/">count</a>()
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT COUNT(name) FROM users
WHERE age &gt; 18
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/filter/">filter</a>(
    r.<a href="/api/javascript/row/">row</a>.<a href="/api/javascript/hasFields/">hasFields</a>("name")
    .<a href="/api/javascript/and/">and</a>(r.<a href="/api/javascript/row/">row</a>("age").<a href="/api/javascript/gt/">gt</a>(18))
).<a href="/api/javascript/count/">count</a>()
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT AVG("age")
    FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/avg/">avg</a>("age")
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT MAX("age")
    FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users")("age").<a href="/api/javascript/max/">max</a>()
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT MIN("age")
    FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users")("age").<a href="/api/javascript/min/">min</a>()
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT SUM("num_posts")
    FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/sum/">sum</a>("num_posts")
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT DISTINCT(name) FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/pluck/">pluck</a>("name")
 .<a href="/api/javascript/distinct/">distinct</a>()
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT *
    FROM users
    WHERE age BETWEEN 18 AND 65;
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/filter/">filter</a>(
    r.<a href="/api/javascript/row/">row</a>("age").<a href="/api/javascript/ge/">ge</a>(18)
     .<a href="/api/javascript/and/">and</a>(r.<a href="/api/javascript/row/">row</a>("age").<a href="/api/javascript/le/">le</a>(65))
).<a href="/api/javascript/count/">count</a>()
</pre>

If you have a secondary index built on the field <code>age</code>, you can run a
more efficient query:
<pre>
r.<a href="/api/javascript/table/">table</a>("users")
 .<a href="/api/javascript/between/">between</a>(18, 65, {index: "age"})
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT name, 'is_adult' = CASE
    WHEN age>18 THEN 'yes'
    ELSE 'no'
    END
FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/map/">map</a>({
    name: r.<a href="/api/javascript/row/">row</a>("name"),
    is_adult: r.<a href="/api/javascript/branch/">branch</a>(
        r.<a href="/api/javascript/row/">row</a>("age").<a href="/api/javascript/gt/">gt</a>(18),
        "yes",
        "no"
    )
})
</pre>
<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/map/">map</a>(
  function (user) {
    return {
      name: user("name"),
      is_adult: r.<a href="/api/javascript/branch/">branch</a>(
        user("age").<a href="/api/javascript/gt/">gt</a>(18),
        "yes",
        "no"
      )
    };
  }
)
</pre>

        </td></tr>


        <tr><td>

<pre>
SELECT *
  FROM posts
  WHERE EXISTS
    (SELECT * FROM users
     WHERE posts.author_id
         = users.id)
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("posts")
  .<a href="/api/javascript/filter/">filter</a>(function (post) {
    return r.<a href="/api/javascript/table/">table</a>("users")
      .<a href="/api/javascript/filter/">filter</a>(function (user) {
        return user("id").<a href="/api/javascript/eq/">eq</a>(post("authorId"))
      }).<a href="/api/javascript/count/">count</a>().<a href="/api/javascript/gt/">gt</a>(0)
    })
</pre>

        </td></tr>





    </tbody>
</table>

# UPDATE #

This is a list of commands for updating data in the database.

<table class="table-2-columns">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>

        <tr><td>

<pre>
UPDATE users
    SET age = 18
    WHERE age &lt; 18
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/filter/">filter</a>(
    r.<a href="/api/javascript/row/">row</a>("age").<a href="/api/javascript/lt">lt</a>(18)
).<a href="/api/javascript/update/">update</a>({age: 18})
</pre>


<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/filter/">filter</a>(
  function (doc) {
    return doc("age").<a href="/api/javascript/lt">lt</a>(18);
  }).<a href="/api/javascript/update/">update</a>({age: 18})
</pre>

        </td></tr>

        <tr><td>

<pre>
UPDATE users
    SET age = age+1
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/update/">update</a>(
    {age: r.<a href="/api/javascript/row/">row</a>("age").add(1)}
)
</pre>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/update/">update</a>(
    function (doc) {
        return {age: doc("age").add(1)};
    }
)
</pre>

        </td></tr>

    </tbody>
</table>

# DELETE #

This is a list of queries for deleting data from the database.

<table class="table-2-columns">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>

        <tr><td>
<pre>
DELETE FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/delete/">delete</a>()
</pre>

        </td></tr>

        <tr><td>
<pre>
DELETE FROM users
WHERE age &lt; 18
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users")
    .<a href="/api/javascript/filter/">filter</a>(r.<a href="/api/javascript/row/">row</a>("age").<a href="/api/javascript/lt/">lt</a>(18))
    .<a href="/api/javascript/delete/">delete</a>()
</pre>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/filter/">filter</a>(
  function (doc) {
    return doc("age").<a href="/api/javascript/lt">(18)</a>;
  }
).<a href="/api/javascript/delete/">delete</a>()
</pre>

        </td></tr>

    </tbody>
</table>



# JOINS #

This is a list of queries for performing joins between multiple
tables.

<table class="table-2-columns">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>

        <tr><td>
<pre>
SELECT *
FROM posts
JOIN users
ON posts.user_id = users.id
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("posts").<a href="/api/javascript/inner_join/">innerJoin</a>(
  r.<a href="/api/javascript/table/">table</a>("users"),
  function (post, user) {
    return post("userId").<a href="/api/javascript/eq/">eq</a>(user("id"));
}).<a href="/api/javascript/zip/">zip</a>()
</pre>

<p><em>Note:</em> <code>zip()</code> will merge the user in the post, overwriting fields in case of conflict.</p>

<p>If you have an index (primary key or secondary index) built on the field of the right table, you can perform a more efficient join with <a href="/api/javascript/eq_join/">eqJoin</a>.</p>

<pre>
r.<a href="/api/javascript/table/">table</a>("posts").<a href="/api/javascript/eq_join/">eqJoin</a>(
    "id",
    r.<a href="/api/javascript/table/">table</a>("users"),
    {index: "id"}
).<a href="/api/javascript/zip/">zip</a>()
</pre>

        </td></tr>

        <tr><td>
<pre>
SELECT posts.id AS post_id,
       user.name,
       users.id AS user_id
    FROM posts
    JOIN users
        ON posts.user_id = users.id
</pre>

<pre>
SELECT posts.id AS post_id,
       user.name,
       users.id AS user_id
    FROM posts
    INNER JOIN users
        ON posts.user_id = users.id
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("posts").<a href="/api/javascript/inner_join/">innerJoin</a>(
  r.<a href="/api/javascript/table/">table</a>("users"),
  function (post, user) {
    return post("userId").<a href="/api/javascript/eq/">eq</a>(user("id"));
}).<a href="/api/javascript/map/">map</a>({
  postId: r.<a href="/api/javascript/row/">row</a>("left")("id"),
  userId: r.<a href="/api/javascript/row/">row</a>("right")("id"),
  name: r.<a href="/api/javascript/row/">row</a>("right")("name")
})
</pre>



        </td></tr>

        <tr><td>
<pre>
SELECT *
    FROM posts
    RIGHT JOIN users
        ON posts.user_id = users.id
</pre>

<pre>
SELECT *
    FROM posts
    RIGHT OUTER JOIN users
        ON posts.user_id = users.id
</pre>


        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("posts").<a href="/api/javascript/outer_join/">outerJoin</a>(
  r.<a href="/api/javascript/table/">table</a>("users"),
  function (post, user) {
    return post("userId").<a href="/api/javascript/eq/">eq</a>(user("id"));
}).<a href="/api/javascript/zip/">zip</a>()
</pre>

<p><em>Note</em>: You can perform more efficient <code>OUTER JOIN</code> operations with the <a href="/api/javascript/concat_map/">concatMap</a> command.</p>

<pre>
r.<a href="/api/javascript/table/">table</a>("posts").<a href="/api/javascript/concat_map/">concatMap</a>(
  function (post) {
    return r.<a href="/api/javascript/table/">table</a>("users")
    .<a href="/api/javascript/get_all/">getAll</a>(post("id"), {index: id})
    .<a href="/api/javascript/do/">do</a>(
      function (result) {
        return r.<a href="/api/javascript/branch/">branch</a>(
          result.<a href="/api/javascript/count/">count</a>().eq(0),
          [{left: post}],
          result.<a href="/api/javascript/map/">map</a>(function (user) {
            return {
              left: post, right: user
            };
          })
        );
      }
    );
  }
).<a href="/api/javascript/zip/">zip</a>();
</pre>

        </td></tr>

        <tr><td>
<pre>
SELECT *
    FROM posts
    LEFT JOIN users
        ON posts.user_id = users.id
</pre>
<pre>
SELECT *
    FROM posts
    LEFT OUTER JOIN users
        ON posts.user_id = users.id
</pre>


        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/outer_join/">outerJoin</a>(
  r.<a href="/api/javascript/table/">table</a>("posts"),
  function (user, post) {
    return post("userId").<a href="/api/javascript/eq/">eq</a>(user("id"));
  }
).<a href="/api/javascript/zip/">zip</a>()
</pre>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/concat_map/">concatMap</a>(
  function (user) {
    return r.<a href="/api/javascript/table/">table</a>("posts").<a href="/api/javascript/get_all/">getAll</a>(user("id"), {index: "id"}).<a href="/api/javascript/do/">do</a>(
      function (results) {
        return r.<a href="/api/javascript/branch/">branch</a>(
          results.<a href="/api/javascript/count/">count</a>().<a href="/api/javascript/eq/">eq</a>(0),
          [{left: user}],
          results.<a href="/api/javascript/map/">map</a>(function (post) {
            return {left: user, right: post};
          })
        );
      }
    );
  }
).<a href="/api/javascript/zip/">zip</a>()
</pre>

        </td></tr>
    </tbody>
</table>


# AGGREGATIONS #

This is a list of queries for performing data aggregation.

<table class="table-2-columns">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>

        <tr><td>

<pre>
SELECT category
    FROM posts
    GROUP BY category
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("posts").<a href="/api/javascript/map/">map</a>(
    r.<a href="/api/javascript/row/">row</a>("category")
).<a href="/api/javascript/distinct/">distinct</a>()
</pre>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/map/">map</a>(
  function (user) {
    return user("category");
  }
).<a href="/api/javascript/distinct/">distinct</a>()
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT category,
       SUM('num_comments')
    FROM posts
    GROUP BY category
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>('posts')
 .<a href="/api/javascript/group/">group</a>('category')
 .<a href="/api/javascript/sum/">sum</a>('num_comments')
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT category,
       status,
       SUM('num_comments')
    FROM posts
    GROUP BY category, status
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("posts")
 .<a href="/api/javascript/group/">group</a>('category', 'status')
 .<a href="/api/javascript/sum/">sum</a>('num_comments')
</pre>
        </td></tr>

        <tr><td>

<pre>
SELECT category,
       SUM(num_comments)
    FROM posts
    WHERE num_comments &gt; 7
    GROUP BY category

</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("posts")
 .<a href="/api/javascript/filter/">filter</a>(r.row('num_comments').<a href="/api/javascript/gt/">gt</a>(7))
 .<a href="/api/javascript/group/">group</a>('category')
 .<a href="/api/javascript/sum/">sum</a>('num_comments')
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT category,
       SUM(num_comments)
    FROM posts
    GROUP BY category
    HAVING num_comments &gt; 7

</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("posts")
 .<a href="/api/javascript/group/">group</a>('category')
 .<a href="/api/javascript/sum/">sum</a>('num_comments')
 .<a href="/api/javascript/ungroup/">ungroup</a>()
 .<a href="/api/javascript/filter/">filter</a>(r.<a href="/api/javascript/row/">row</a>("reduction").<a href="/api/javascript/gt/">gt</a>(7))
</pre>

        </td></tr>

    </tbody>
</table>

# TABLE and DATABASE manipulation #

This is a list of queries for creating and dropping tables and
databases.

<table class="table-2-columns">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>

        <tr><td>

<pre>
CREATE DATABASE my_database;
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/db_create/">dbCreate</a>('my_database')
</pre>

        </td></tr>

        <tr><td>

<pre>
DROP DATABASE my_database;
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/db_drop/">dbDrop</a>('my_database')
</pre>

        </td></tr>



        <tr><td>

<pre>
CREATE TABLE users
    (id INT IDENTITY(1,1) PRIMARY KEY,
    name VARCHAR(50),
    age INT);
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table_create/">tableCreate</a>('users', {primaryKey: "id"})
</pre>
<p><em>Note:</em> RethinkDB is a NoSQL database and does not enforce
schemas.</p>

<p><em>Note:</em> The default primary key is <code>id</code></p>



        </td></tr>


        <tr><td>

<pre>
TRUNCATE TABLE users;
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table/">table</a>("users").<a href="/api/javascript/delete/">delete</a>()
</pre>



        </td></tr>

        <tr><td>

<pre>
DROP TABLE users;
</pre>

        </td><td>

<pre>
r.<a href="/api/javascript/table_drop/">tableDrop</a>("users")
</pre>

        </td></tr>

    </tbody>
</table>



# Read More #

Browse the following resources to learn more about ReQL:

- [Lambda functions in RethinkDB](/blog/lambda-functions/)
- [Introduction to map-reduce](/docs/map-reduce/)
- [Introduction to Joins](/docs/table-joins/)
- [API Reference](/api/)
