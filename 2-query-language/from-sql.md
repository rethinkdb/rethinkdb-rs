---
layout: documentation
title: SQL to ReQL cheat sheet
active: docs
docs_active: sql-to-reql
permalink: docs/sql-to-reql/
---

{% infobox info %}
__Note:__ Examples below are in Python. Head to the [API reference](/api/) to see the commands in other languages.
{% endinfobox %}


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
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:writing_data-insert">insert</a>({
   "user_id": "f62255a8259f",
   "age": 30,
   "name": "Peter"
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
r.<a href="/api/#py:selecting_data-table">table</a>("users")
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT user_id, name FROM users
</pre>
        </td><td>
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users")
 .<a href="/api/#py:document_manipulation-pluck">pluck</a>("user_id", "name")
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT * FROM users
WHERE name = "Peter"
</pre>
        </td><td>
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:selecting_data-filter">filter</a>({
    "name": "Peter"
})
</pre>

<p>An alternative is to use the implicit variable <code>r.row</code> (the currently visited document):</p>
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:selecting_data-filter">filter</a>(
    r.<a href="/api/#py:document_manipulation-row">row</a>["name"] <a href="/api/#py:math_and_logic-eq">==</a> "Peter"
)
</pre>

<p>Another alternative is to use an anonymous function:</p>
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:selecting_data-filter">filter</a>(lambda doc:
    doc["name"] <a href="/api/#py:math_and_logic-eq">==</a> "Peter"
)
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT user_id, name FROM users
WHERE name = "Peter"
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:selecting_data-filter">filter</a>({
    "name": "Peter"
}).<a href="/api/#py:document_manipulation-pluck">pluck</a>("user_id", "name")
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT * FROM users
WHERE name == "Peter"
AND age = 30
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:selecting_data-filter">filter</a>({
    "name": "Peter",
    "age": 30
})
</pre>

<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:selecting_data-filter">filter</a>(
    (r.<a href="/api/#py:document_manipulation-row">row</a>["name"] <a href="/api/#py:math_and_logic-eq">==</a> "Peter")
     <a href="/api/#py:math_and_logic-and">&</a> (r.<a href="/api/#py:document_manipulation-row">row</a>["age"] <a href="/api/#py:math_and_logic-eq">==</a> 30)
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
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:selecting_data-filter">filter</a>(
    r.<a href="/api/#py:document_manipulation-row">row</a>["age"] <a href="/api/#py:math_and_logic-gt">&gt;</a> 30
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
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:selecting_data-filter">filter</a>(
    r.<a href="/api/#py:document_manipulation-row">row</a>['name'].<a href="/api/#py:string_manipulation-match">match</a>("^P")}
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
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:selecting_data-filter">filter</a>(
    r.<a href="/api/#py:document_manipulation-row">row</a>['name'].<a href="/api/#py:string_manipulation-match">match</a>("er$")}
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
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:transformations-order_by">order_by</a>("name")
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT * FROM users
ORDER BY name DESC
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:transformations-order_by">order_by</a>(
    r.desc("name")
)
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT user_id FROM users
WHERE name = Peter
ORDER BY name DESC
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:selecting_data-filter">filter</a>({
    "name": "Peter"
}).<a href="/api/#py:transformations-order_by">order_by</a>(
    r.desc("name")
).<a href="/api/#py:document_manipulation-pluck">pluck</a>("user_id")
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT * FROM users LIMIT 5
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:transformations-limit">limit</a>(5)
</pre>

        </td></tr>
 
        <tr><td>

<pre>
SELECT * FROM users LIMIT 5 SKIP 10
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:transformations-skip">skip</a>(10).<a href="/api/#py:transformations-limit">limit</a>(5)
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT * FROM users
WHERE name IN {'Peter', 'John'}
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:selecting_data-filter">filter</a>(
    r.<a href="/api/#py:control_structures-expr"</a>expr</a>(["Peter", "John"])
        .<a href="/api/#py:aggregation-contains">contains</a>(r.<a href="/api/#py:document_manipulation-row">row</a>["name"])
)
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT * FROM users
WHERE name NOT IN {'Peter', 'John'}
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:selecting_data-filter">filter</a>(
    r.<a href="/api/#py:control_structures-expr"</a>expr</a>(["Peter", "John"])
        .<a href="/api/#py:aggregation-contains">contains</a>(r.<a href="/api/#py:document_manipulation-row">row</a>["name"])
        .not()
)
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT COUNT(*) FROM users
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:aggregation-count">count</a>()
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT COUNT(name) FROM users
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:selecting_data-filter">filter</a>(lambda doc:
    doc.<a href="/api/#py:document_manipulation-has_fields">has_fields</a>("name")
).<a href="/api/#py:aggregation-count">count</a>()
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT COUNT(name) FROM users
WHERE age &gt; 18
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:selecting_data-filter">filter</a>(
    (r.<a href="/api/#py:document_manipulation-row">row</a>.<a href="/api/#py:document_manipulation-has_fields">has_fields</a>("name"))
     <a href="/api/#py:math_and_logic-and">&</a> (r.<a href="/api/#py:document_manipulation-row">row</a>["age"] <a href="/api/#py:math_and_logic-gt">&gt;</a> 18)
).<a href="/api/#py:aggregation-count">count</a>()
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT AVG("age")
FROM users
</pre>

        </td><td>
        
<pre>
(r.<a href="/api/#py:selecting_data-table">table</a>("users")
    .<a href="/api/#py:transformations-map">map</a>(lambda user:
        user["age"]
    ).<a href="api/#py:aggregation-reduce">reduce</a>(lambda left, right:
        left+right
    )/r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:aggregation-count">count</a>())
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT MAX("age")
    FROM users
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users")
    .<a href="/api/#py:transformations-map">map</a>(lambda user:
        user["age"]
    ).<a href="api/#py:aggregation-reduce">reduce</a>(lambda left, right:
        r.<a href="/api/#py:control_structures-branch">branch</a>(
            left&gt;right,
            left,
            right
        )
    )
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT MIN("age")
    FROM users
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users")
    .<a href="/api/#py:transformations-map">map</a>(lambda user:
        user["age"]
    ).<a href="api/#py:aggregation-reduce">reduce</a>(lambda left, right:
        r.<a href="/api/#py:control_structures-branch">branch</a>(
            left>right,
            right,
            left
        )
    )
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT SUM("num_posts")
    FROM users
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users")
    .<a href="/api/#py:transformations-map">map</a>(lambda user:
        user["num_posts"]
    ).<a href="api/#py:aggregation-reduce">reduce</a>(lambda left, right:
        left+right
    )
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT DISTINCT(name) FROM users
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:document_manipulation-pluck">pluck</a>("name").<a href="/api/#py:aggregation-distinct">distinct</a>()
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
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:selecting_data-filter">filter</a>(
    (r.row["age"] <a href="api/#py:math_and_logic-ge">>=</a> 18)
    & (r.row["age"] <a href="api/#py:math_and_logic-le">>=</a> 65)
</pre>

If you have a secondary index built on the field <code>age</code>. you can run a 
more efficient query:
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users")
    .<a href="/api/#py:selecting_data-between">between</a>(18, 65, index="age")
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
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:transformations-map">map</a>({
    "name": r.<a href="/api/#py:document_manipulation-row">row</a>["name"],
    "is_adult": r.<a href="/api/#py:control_structures-branch">branch</a>(
        r.<a href="/api/#py:document_manipulation-row">row</a>["age"] <a href="/api/#py:math_and_logic-gt">&gt;</a> 18,
        "yes",
        "no"
    )
})
</pre>
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:transformations-map">map</a>(lambda user:
    {
        "name": user["name"],
        "is_adult": r.<a href="/api/#py:control_structures-branch">branch</a>(
            user["age"] <a href="/api/#py:math_and_logic-gt">&gt;</a> 18
            "yes",
            "no"
        )
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
r.<a href="/api/#py:selecting_data-table">table</a>("posts")
  .<a href="/api/#py:selecting_data-filter">filter</a>(lambda post:
    r.<a href="/api/#py:selecting_data-table">table</a>("users")
      .<a href="/api/#py:selecting_data-filter">filter</a>( lambda user:
        user.id <a href="/api/#py:math_and_logic-eq">==</a> post.author_id
      ).<a href="/api/#py:aggregation-count">count</a>() <a href="/api/#py:math_and_logic-gt">&gt;</a> 0
    )
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
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:selecting_data-filter">filter</a>(
    r.<a href="/api/#py:document_manipulation-row">row</a>["age"] < 18
).<a href="/api/#py:writing_data-update">update</a>({
    "age": 18
})
</pre>


<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:selecting_data-filter">filter</a>(lambda doc:
    doc["age"] < 18
).<a href="/api/#py:writing_data-update">update</a>({
    "age": 18
})
</pre>

        </td></tr>

        <tr><td>

<pre>
UPDATE users
    SET age = age+1
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:writing_data-update">update</a>(
    { "age": r.<a href="/api/#py:document_manipulation-row">row</a>["age"]+1 }
)
</pre>

<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:writing_data-update">update</a>(lambda doc:
    { "age": doc["age"]+1 }
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
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:writing_data-delete">delete</a>()
</pre>

        </td></tr>

        <tr><td>
<pre>
DELETE FROM users
WHERE age &lt; 18
</pre>

        </td><td>

<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users")
    .<a href="/api/#py:selecting_data-filter">filter</a>( r.<a href="/api/#py:document_manipulation-row">row</a>["age"] < 18)
    .<a href="/api/#py:writing_data-delete">delete</a>()
</pre>

<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users")
    .<a href="/api/#py:selecting_data-filter">filter</a>(lambda doc:
    doc["age"] < 18
    ).<a href="/api/#py:writing_data-delete">delete</a>()
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
r.<a href="/api/#py:selecting_data-table">table</a>("posts").<a href="/api/#py:joins-inner">inner_join</a>(
    r.<a href="/api/#py:selecting_data-table">table</a>("users"),
    lambda post, user:
        post["user_id"] <a href="/api/#py:math_and_logic-eq">==</a> user["id"]
).<a href="/api/#py:joins-zip">zip</a>()
</pre>

<p><em>Note:</em> <code>zip()</code> will merge the user in the post, overwriting fields in case of conflict.</p>

<p>If you have an index (primary key or secondary index) built on the field of the right table, you can perform a more efficient join with <a href="/api/#py:joins-eqJoin">eq_join</a></p>
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("posts").<a href="/api/#py:joins-eqJoin">eq_join</a>(
    "id",
    r.<a href="/api/#py:selecting_data-table">table</a>("users"),
    index="id"
).<a href="/api/#py:joins-zip">zip</a>()
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
r.<a href="/api/#py:selecting_data-table">table</a>("posts").<a href="/api/#py:joins-inner">inner_join</a>(
  r.<a href="/api/#py:selecting_data-table">table</a>("users"),
  lambda post, user:
    post["user_id"] <a href="/api/#py:math_and_logic-eq">==</a> user["id"]
).<a href="/api/#py:transformations-map">map</a>({
  "post_id": r.<a href="/api/#py:document_manipulation-row">row</a>["left"]["id"],
  "user_id": r.<a href="/api/#py:document_manipulation-row">row</a>["right"]["id"],
  "name": r.<a href="/api/#py:document_manipulation-row">row</a>["right"]["name"]
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
r.<a href="/api/#py:selecting_data-table">table</a>("posts").<a href="/api/#py:joins-outer">outer_join</a>(
    r.<a href="/api/#py:selecting_data-table">table</a>("users"),
    lambda post, user:
        post["user_id"] <a href="/api/#py:math_and_logic-eq">==</a> user["id"]
).<a href="/api/#py:joins-zip">zip</a>()
</pre>

<p><em>Note</em>: You can perform efficient <code>OUTER JOIN</code> operations with the <a href="/api/#py:transformations-concat_map">concat_map</a> command.<br/>
The <code>eq_join</code> command will eventually be able to behave like an OUTER JOIn, see <a href="https://github.com/rethinkdb/rethinkdb/issues/1223">this github issue</a>.

<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("posts").<a href="/api/#py:transformations-concat_map">concat_map</a>(lambda post:
  r.<a href="/api/#py:selecting_data-table">table</a>("users")
    .<a href="/api/#py:selecting_data-get_all">get_all</a>(post["id"],index="id")
    .<a href="/api/#py:control_structures-do">do</a>( lambda results:
      r.<a href="/api/#py:control_structures-branch">branch</a>(
        results.<a href="/api/#py:aggregation-count">count</a>() <a href="/api/#py:math_and_logic-eq">==</a> 0,
        [{"left": post}],
        results.<a href="/api/#py:transformations-map">map</a>( lambda user:
          {
            "left": post
            "right": user
          }
        )
      ) 
    )
).<a href="/api/#py:joins-zip">zip</a>()
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
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:joins-outer">outer_join</a>(
    r.<a href="/api/#py:selecting_data-table">table</a>("posts"),
    lambda user, post:
        post.user_id <a href="/api/#py:math_and_logic-eq">==</a> user.id
).<a href="/api/#py:joins-zip">zip</a>()
</pre>

<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:transformations-concat_map">concat_map</a>(lambda user:
  r.<a href="/api/#py:selecting_data-table">table</a>("posts")
    .<a href="/api/#py:selecting_data-get_all">get_all</a>(user["id"],index="id")
    .<a href="/api/#py:control_structures-do">do</a>( lambda results:
      r.<a href="/api/#py:control_structures-branch">branch</a>(
        results.<a href="/api/#py:aggregation-count">count</a>() <a href="/api/#py:math_and_logic-eq">==</a> 0,
        [{"left": user}],
        results.<a href="/api/#py:transformations-map">map</a>( lambda post:
          {
            "left": user
            "right": post
          }
        )
      ) 
    )
).<a href="/api/#py:joins-zip">zip</a>()
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
r.<a href="/api/#py:selecting_data-table">table</a>("posts").<a href="/api/#py:transformations-map">map</a>(
    r.<a href="/api/#py:document_manipulation-row">row</a>["category"]
).<a href="/api/#py:aggregation-distinct">distinct</a>()
</pre>

<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:transformations-map">map</a>(lambda user
    user["category"]
).<a href="/api/#py:aggregation-distinct">distinct</a>()
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT category,
       SUM('num_comments')
           AS total
    FROM posts
    GROUP BY category
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("posts")
  .<a href="/api/#py:aggregation-group_by">group_by</a>('category',
    r.sum('num_comments')
  ).<a href="/api/#py:transformations-map">map</a>({
    "category": r.<a href="/api/#py:document_manipulation-row">row</a>["group"],
    "total": r.<a href="/api/#py:document_manipulation-row">row</a>["reduction"]
})
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT category,
       status,
       SUM('num_comments')
           AS total
    FROM posts
    GROUP BY category, status
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("posts")
  .<a href="/api/#py:aggregation-groupedmapreduce">grouped_map_reduce</a>(
    lambda post:
      [post["category"],
       post["status"]],
    lambda post:
      post["num_comments"],
    lambda left, right:
      left + right
  ).<a href="/api/#py:transformations-map">map</a>({
    "category": r.<a href="/api/#py:document_manipulation-row">row</a>["group"][0],
    "status": r.<a href="/api/#py:document_manipulation-row">row</a>["group"][1],
    "total": r.<a href="/api/#py:document_manipulation-row">row</a>["reduction"]
})
</pre>
        </td></tr>

        <tr><td>

<pre>
SELECT id,
    SUM(num_comments)
        AS "total"
    FROM posts
    GROUP BY category, num_comments
        HAVING num_comments &gt; 7

</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("posts")
  .<a href="/api/#py:selecting_data-filter">filter</a>( r.<a href="/api/#py:document_manipulation-row">row</a>["num_comments"]&gt;7
  .<a href="/api/#py:aggregation-groupedmapreduce">grouped_map_reduce</a>(
    lambda post:
      post["category"],
    lambda post:
      post["num_comments"],
    lambda left, right:
      left + right
  ).<a href="/api/#py:transformations-map">map</a>({
    "category": r.<a href="/api/#py:document_manipulation-row">row</a>["group"],
    "total": r.<a href="/api/#py:document_manipulation-row">row</a>["reduction"]
})
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
r.<a href="/api/#py:manipulating_databases-db_create">db_create</a>('my_database')
</pre>

        </td></tr>

        <tr><td>

<pre>
DROP DATABASE my_database;
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:manipulating_databases-db_drop">db_drop</a>('my_database')
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
r.<a href="/api/#py:manipulating_tables-table_create">table_create</a>('users', primary_key="id")
</pre>
<p><em>Note:</em> RethinkDB is a NoSQL database and does not enforce
schemas.</p>

<p><em>Note²:</em> The default primary key is <code>id</code></p>



        </td></tr>


        <tr><td>

<pre>
TRUNCATE TABLE users;
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:selecting_data-table">table</a>("users").<a href="/api/#py:writing_data-delete">delete</a>()
</pre>



        </td></tr>

        <tr><td>

<pre>
DROP TABLE users;
</pre>

        </td><td>
        
<pre>
r.<a href="/api/#py:manipulating_tables-table_drop">table_drop</a>("users")
</pre>

        </td></tr>

    </tbody>
</table>



# Read More #

Browse the following resources to learn more about ReQL:

- [Lambda functions in RethinkDB](/blog/lambda-functions/)
- [Introduction to map/reduce](/docs/map-reduce/)
- [Introduction to Joins](/docs/table-joins/)
- [API Reference](/api/)
