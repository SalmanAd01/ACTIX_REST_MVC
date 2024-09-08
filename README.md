1. Run Diesel Cli

```
    docker compose -f .\docker-compose_diesel.yml run diesel-cli
```

2. Run Dielsel

```
    diesel setup
    diesel migration generate create_users
    diesel migration run
```