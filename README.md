# Zorvyn Finance Tracker - Backend API Architecture

This repository holds the production-level system infrastructure powering the Zorvyn Finance Tracker dashboard. Built meticulously evaluating the strictest architectural paradigms around scaling, data isolation, performance security, and robust logical hierarchies.

## Overview & The "Why" 
The implementation transcends basic boilerplate API standards by adopting tightly-controlled constraints ensuring system robustness:

1. **Granular Multi-Tier Middleware & Component Separation**: Instead of an unwieldy single route-checking function routing all requests, security layers are specifically fractured across two distinct modular constraints (`block_viewer`, `require_admin`). The inner router trees explicitly combine using purely defined access rules preventing horizontal privilege climbing inherently regardless of future expansion scaling faults!
2. **Built-in Concurrent Fixed Rate-Limitor Guard**: Utilizing pure raw rust `tokio` concurrency maps wrapping native IP lookups gracefully handling thread-bound memory to instantly squash potential volumetric DOS attacks beyond normal API constraints securely maintaining API sanity (**25 requests / min globally** limitation).
3. **Rust `validator` Crates Mapping Data**: Payload fields explicitly drop bad behavior far prior to controller executions mapping accurate business constraints!
4. **SQLX Embedded Compile-time Validations and Zero-Step Booting**: The migrations are explicitly injected right to the runtime via `#![sqlx(migrations)]` logic guaranteeing databases launch entirely cleanly upon zero setup commands via pure docker composition without developers running manual CLI triggers globally avoiding state misalignments.
5. **Non-Blocking File Rolling**: Tracing operates efficiently, pushing logging over IO off to a dedicated worker explicitly ensuring requests do not backpedal off asynchronous lock holds wrapping day logs automatically! 

## API Explorer
You can view the full interactible layout listing exact permissions, variables, and API structure locally by navigating to:
**[http://localhost:7878/docs](http://localhost:7878/docs)** 

*(Generates a pristine, full Swagger UI automatically inside the network locally!)*

## Getting Started (Docker Magic)
The repository leverages extremely stripped down lightweight multi-stage builds guaranteeing total operation abstraction! 
Absolutely zero Rust environment, variables, or database tracking components needed on the host machine. 
1. `docker compose up --build`
*(The backend sequentially halts itself securely until the internal Postgres network is actively listening, configures schemas, then boots cleanly over `port: 7878`)*

## Database Structure Configuration
![ER Diagram](public/ER-Diagram.svg)

## Explicit Project Structure
- `.env.example / docker-compose.yml`: Top level deployment rules, configuring standard routing interfaces mirroring mapping across Alpine images locally and on bare metal.
- `src/controllers`: Clean API handlers completely abstracted out of data persistence constraints. These components directly invoke pure state parameters without internal data handling.
- `src/middlewares`: Modular security enforcement files including `JWT Header Parses`, `IP Global Fixed Window Request Pinging Tracker`, and the exact `Role Guard Rejections`.
- `src/models`: System data layout structs tied natively to `serde` tracking enforcing strict validation checks directly at standard interface reception explicitly compressing network layouts (like explicitly dropping unset attributes implicitly minimizing payloads).
- `src/routes`: Handles nested grouping explicit maps allowing robust layers! Highly clustered configurations tying controllers securely back over modular endpoints cleanly wrapping endpoints strictly.
- `src/services`: Shared system handlers establishing DB states (`db.rs`) injecting exact transaction queries without blocking controller logic heavily optimizing separation routines, and authentication handlers generating and checking token cryptography maps (`authentication_services.rs`).
