---
layout: documentation
title: "Stability report"
short_title: Stability report
docs_active: stability
permalink: stability/
alias: docs/stability/
---

RethinkDB was built over five years by a team of database experts with
the help of hundreds of contributors from around the world. It is
being [used in production][] by hundreds of technology startups,
consulting studios, and Fortune 500 companies.

[used in production]: /faq#production-use

# Single node deployments #

Single node deployments have been extensively tested and all known systemic issues have been resolved. If you run into a problem, please <a href="https://github.com/rethinkdb/rethinkdb/issues/new">open a bug report</a>.

|                              | Single node stability report |
| :--------------------------- | --------- |
| Data integrity               | There are no known data integrity issues. |
| Crash reports                | There are no known systemic stability issues. |
| Query performance            | There are no known systemic performance degradation issues. |
| Memory allocation            | There are no known memory leaks or systemic memory issues. |
| Sustained load               | There is no systemic degradation under sustained load. |
| Features                     | All of the required features are available. |
| Extent of testing            | Single node deployments have been extensively tested. |
| Migration                    | Migration for major feature releases is automatic. |

# Deployments of small clusters #

Small cluster deployments have been extensively tested and all known systemic issues have been resolved. If you run into a problem, please <a href="https://github.com/rethinkdb/rethinkdb/issues/new">open a bug report</a>.

|                              | Small cluster stability report (five servers or fewer) |
| :--------------------------- | --------- |
| Data integrity               | There are no known data integrity issues. |
| Crash reports                | There are no known systemic stability issues. |
| Query performance            | There are no known systemic performance degradation issues. |
| Memory allocation            | There are no known memory leaks or systemic memory issues. |
| Sustained load               | There is no systemic degradation under sustained load. |
| Features                     | All of the required features are available. |
| Scalability                  | There are no known systemic scalability issues in small cluster deployments. |
| Extent of testing            | Small cluster deployments have been extensively tested. |
| Migration                    | Migration for major feature releases is automatic. |

# Deployments of large clusters #

Large cluster deployments have been extensively tested and all known systemic issues have been resolved. If you run into a problem, please <a href="https://github.com/rethinkdb/rethinkdb/issues/new">open a bug report</a>.

|                              | Large cluster stability report (more than five servers) |
| :--------------------------- | --------- |
| Data integrity               | There are no known data integrity issues. |
| Crash reports                | There are no known systemic stability issues. |
| Query performance            | There are no known systemic performance degradation issues. |
| Memory allocation            | There are no known memory leaks or systemic memory issues. |
| Sustained load               | There is no systemic degradation under sustained load. |
| Features                     | All of the required features are available. |
| Scalability                  | There are no known systemic scalability issues in large cluster deployments. |
| Extent of testing            | Large cluster deployments have been extensively tested. |
| Migration                    | Migration for major feature releases is automatic. |

