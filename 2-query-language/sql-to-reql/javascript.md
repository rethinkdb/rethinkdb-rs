---
layout: documentation
title: SQL to ReQL cheat sheet
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
r.table("users").insert({
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
r.table("users")
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT user_id, name FROM users
</pre>
        </td><td>
<pre>
r.table("users")
 .pluck("userId", "name")
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT * FROM users
WHERE name = "Peter"
</pre>
        </td><td>
<pre>
r.table("users").filter({
    name: "Peter"
})
</pre>

<p>If you have a secondary index built on the field <code>name</code>, you can run a
more efficient query:</p>
<pre>
r.table("users")
    .getAll("Peter", {index: "name"})
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
r.table("users").filter({
    name: "Peter",
    age: 30
})
</pre>

        </td></tr>


        <tr><td>

<pre>
SELECT * FROM users
WHERE name LIKE "P%"
</pre>

        </td><td>
<pre>
r.table("users").filter(
    r.row("name").match("^P")}
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
r.table("users").orderBy("name")
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT * FROM users
ORDER BY name DESC
</pre>

        </td><td>

<pre>
r.table("users").orderBy(
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
r.table("users").filter({
    name: "Peter"
}).orderBy(
    r.desc("name")
).pluck("userId")
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT * FROM users LIMIT 5 SKIP 10
</pre>

        </td><td>

<pre>
r.table("users").skip(10).limit(5)
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT * FROM users
WHERE name IN ('Peter', 'John')
</pre>

        </td><td>

<pre>
r.table("users").filter(
  function (doc) {
    return r.expr(["Peter","John"])
            .contains(doc("name"));
  }
)
</pre>

<p>If you have a secondary index built on the field <code>name</code>, you can run a
more efficient query:</p>
<pre>
r.table("users")
    .getAll("Peter", "John",
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
r.table("users").filter(
  function (doc) {
    return r.expr(["Peter","John"])
            .contains(doc("name"))
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
r.table("users").count()
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT COUNT(name) FROM users
WHERE age &gt; 18
</pre>

        </td><td>

<pre>
r.table("users").filter(
    r.row.hasFields("name")
    .and(r.row("age").gt(18))
).count()
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT AVG("age")
    FROM users
</pre>

        </td><td>

<pre>
r.table("users").avg("age")
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT MAX("age")
    FROM users
</pre>

        </td><td>

<pre>
r.table("users")("age").max()
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT DISTINCT(name) FROM users
</pre>

        </td><td>

<pre>
r.table("users").pluck("name")
 .distinct()
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
r.table("users").filter(
    r.row("age").ge(18)
     .and(r.row("age").le(65))
).count()
</pre>

If you have a secondary index built on the field <code>age</code>, you can run a
more efficient query:
<pre>
r.table("users")
 .between(18, 65, {index: "age"})
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
r.table("users").map({
    name: r.row("name"),
    is_adult: r.branch(
        r.row("age").gt(18),
        "yes",
        "no"
    )
})
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
r.table("posts")
  .filter(function (post) {
    return r.table("users")
      .filter(function (user) {
        return user("id").eq(post("authorId"))
      }).count().gt(0)
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
r.table("users").filter(
    r.row("age").lt(18)
).update({age: 18})
</pre>


        </td></tr>

        <tr><td>

<pre>
UPDATE users
    SET age = age+1
</pre>

        </td><td>

<pre>
r.table("users").update(
    {age: r.row("age").add(1)}
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
r.table("users").delete()
</pre>

        </td></tr>

        <tr><td>
<pre>
DELETE FROM users
WHERE age &lt; 18
</pre>

        </td><td>

<pre>
r.table("users")
    .filter(r.row("age").lt(18))
    .delete()
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
r.table("posts").innerJoin(
  r.table("users"),
  function (post, user) {
    return post("userId").eq(user("id"));
}).zip()
</pre>

<p><em>Note:</em> <code>zip()</code> will merge the user in the post, overwriting fields in case of conflict.</p>

<p>If you have an index (primary key or secondary index) built on the field of the right table, you can perform a more efficient join with <a href="/api/javascript/eq_join/">eqJoin</a>.</p>

<pre>
r.table("posts").eqJoin(
    "id",
    r.table("users"),
    {index: "id"}
).zip()
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

SELECT posts.id AS post_id,
       user.name,
       users.id AS user_id
    FROM posts
    INNER JOIN users
        ON posts.user_id = users.id
</pre>

        </td><td>

<pre>
r.table("posts").innerJoin(
  r.table("users"),
  function (post, user) {
    return post("userId").eq(user("id"));
}).map({
  postId: r.row("left")("id"),
  userId: r.row("right")("id"),
  name: r.row("right")("name")
})
</pre>



        </td></tr>

        <tr><td>
<pre>
SELECT *
    FROM posts
    RIGHT JOIN users
        ON posts.user_id = users.id

SELECT *
    FROM posts
    RIGHT OUTER JOIN users
        ON posts.user_id = users.id
</pre>


        </td><td>

<pre>
r.table("posts").outerJoin(
  r.table("users"),
  function (post, user) {
    return post("userId").eq(user("id"));
}).zip()
</pre>

<p><em>Note</em>: You can perform more efficient <code>OUTER JOIN</code> operations with the <a href="/api/javascript/concat_map/">concatMap</a> command.</p>

<pre>
r.table("posts").concatMap(
  function (post) {
    return r.table("users")
    .getAll(post("id"), {index: id})
    .do(
      function (result) {
        return r.branch(
          result.count().eq(0),
          [{left: post}],
          result.map(function (user) {
            return {
              left: post, right: user
            };
          })
        );
      }
    );
  }
).zip();
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
r.table("users").outerJoin(
  r.table("posts"),
  function (user, post) {
    return post("userId").eq(user("id"));
  }
).zip()
</pre>

<pre>
r.table("users").concatMap(
  function (user) {
    return r.table("posts").getAll(user("id"), {index: "id"}).do(
      function (results) {
        return r.branch(
          results.count().eq(0),
          [{left: user}],
          results.map(function (post) {
            return {left: user, right: post};
          })
        );
      }
    );
  }
).zip()
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
r.table("posts").map(
    r.row("category")
).distinct()
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
r.table('posts')
 .group('category')
 .sum('num_comments')
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
r.table("posts")
 .group('category', 'status')
 .sum('num_comments')
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
r.table("posts")
 .filter(r.row('num_comments').gt(7))
 .group('category')
 .sum('num_comments')
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
r.table("posts")
 .group('category')
 .sum('num_comments')
 .ungroup()
 .filter(r.row("reduction").gt(7))
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
r.dbCreate('my_database')
</pre>

        </td></tr>

        <tr><td>

<pre>
DROP DATABASE my_database;
</pre>

        </td><td>

<pre>
r.dbDrop('my_database')
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
r.tableCreate('users', {primaryKey: "id"})
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
r.table("users").delete()
</pre>



        </td></tr>

        <tr><td>

<pre>
DROP TABLE users;
</pre>

        </td><td>

<pre>
r.tableDrop("users")
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
