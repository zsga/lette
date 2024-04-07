# Lette

## Installation

### 1. Clone the repository

```
git clone https://github.com/zsga/lette
cd lette
```

### 2. Database migration

1. Generate `.env` file:

```
cargo run
```

2. Install `sqlx-cli`:

```sh
cargo install sqlx-cli
```

3. Create database:

```
sqlx database create
```

4. Run migrations:

```
sqlx migrate run
```

more information to read [sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md) document

### 3. Start service

```
cargo run serve
```
