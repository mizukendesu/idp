# IdP

## Stack

### Frontend

[![TypeScript](https://img.shields.io/badge/TypeScript-black?logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Next.js](https://img.shields.io/badge/Next.js-black?logo=next.js&logoColor=white)](https://nextjs.org/)

### Backend

[![Rust](https://img.shields.io/badge/Rust-black?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Actix Web](https://img.shields.io/badge/Actix_Web-black?logo=actix&logoColor=white)](https://actix.rs/)

### Database

[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-black?logo=postgresql&logoColor=white)](https://www.postgresql.org/)

### Other

[![Docker](https://img.shields.io/badge/Docker-black?logo=docker&logoColor=white)](https://www.docker.com/)
[![Nginx](https://img.shields.io/badge/Nginx-black?logo=nginx&logoColor=white)](https://www.nginx.com/)

## How to run

1. Build docker image

```shell
make build
```

2. Migrate database

```shell
make migrate
```

3. Run Server

```shell
docker compose up -d
```
