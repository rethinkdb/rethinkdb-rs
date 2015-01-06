---
layout: api-command
language: JavaScript
permalink: api/javascript/config/
command: config
io:
    -   - r
        - object
---
# Command syntax #

{% apibody %}
r.table('tablename').config() &rarr; object
r.db('dbname').config() &rarr; object
{% endapibody %}

# Description #

Query (read and/or update) the configurations for individual tables or databases.
