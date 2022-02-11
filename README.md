# Itse

## Running locally

- start postgres
```sh
docker-compose up
```
- run migrations
```sh
sqlx migrate run
```

- run with cargo
```sh
cargo run
```



## Generate ed25519 key pair

```bash
scripts/generate-ed25519-keys

```

Fitting those files to single line:
```bash
 awk 'NF {sub(/\r/, ""); printf "%s\\n",$0;}'  private-key.pem
```

```bash
 awk 'NF {sub(/\r/, ""); printf "%s\\n",$0;}'  public-key.pem
```
