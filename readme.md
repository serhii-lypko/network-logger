# Network Logging Tool with Cloud Service Commit

This project is a network logging tool with a cloud service commit feature. It is primarily designed for studying purposes and PC network traffic observation.

## Overview

The system consists of a Rust application running as a daemon on your machine. It captures and stores network traffic in a buffer. Then, using gRPC the client transfers data to the remote server which aggregates statistics of the network usage. The web server handles the data and stores it in an AWS RDS (PostgreSQL) database. It also provides an API to access aggregated data.
