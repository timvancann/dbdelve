# Snowflake

DBDelve talks to Snowflake over its SQL REST API and signs in with a key pair.
This page is what you need to know before filling in the form, and what does
not work yet.

## Signing in

Key-pair authentication only. There is no password, OAuth, browser SSO or
access-token sign-in.

The private key has to be:

- **RSA**, 2048 bits or more.
- **Unencrypted.** A key protected by a passphrase is refused with "is
  encrypted". To make an unencrypted copy of one you already have:

  ```sh
  openssl pkcs8 -topk8 -nocrypt -in rsa_key.p8 -out rsa_key_plain.p8
  ```

- **PEM**, either PKCS#8 (`BEGIN PRIVATE KEY`, what Snowflake's own
  instructions produce) or PKCS#1 (`BEGIN RSA PRIVATE KEY`).

To make a new pair and register it:

```sh
openssl genrsa 2048 | openssl pkcs8 -topk8 -nocrypt -out snowflake.p8
openssl rsa -in snowflake.p8 -pubout -out snowflake.pub
chmod 600 snowflake.p8
```

```sql
ALTER USER my_user SET RSA_PUBLIC_KEY='MIIBIjANBgkq...';  -- the body of snowflake.pub, without its BEGIN/END lines
```

Give the form the key one of two ways:

- **Private key file**: a path. The profile stores the path and nothing else.
- **Or paste the key**: the text itself, kept in the macOS Keychain like a
  password and never written to `profiles.toml`. It can be the PEM, the PEM's
  base64 body alone, or the whole PEM base64-encoded again, which is how a key
  usually sits in a secrets store. If both are filled in, the file wins.

## The form

| Field | |
| --- | --- |
| Account | The account identifier, such as `myorg-myaccount`. Pasting the sign-in URL works; the identifier is taken out of it. |
| Username | The Snowflake user the public key is registered on. |
| Database | Required. A profile browses one database, as a Postgres connection does. |
| Warehouse | Optional; blank uses the user's default. **Needed in practice**: see below. |
| Role | Optional; blank uses the user's default. |
| Host | Optional. Only for privatelink, a regional domain or a proxy. Blank means `<account>.snowflakecomputing.com`. |
| Statement timeout | Seconds, sent with each request. Blank is the account's own limit. |

## Things that behave differently here

- **Connecting resumes the warehouse.** The catalog is read from
  `INFORMATION_SCHEMA`, which needs a running warehouse, and so does opening a
  Structure tab. With no warehouse at all the connection opens and the catalog
  fails with Snowflake's own message.
- **There is no session.** Each run is a separate request, so `USE SCHEMA`,
  `ALTER SESSION` and an open transaction do not carry over to the next run.
  Put them in the same submission as the statements that need them, or qualify
  names.
- **Results are read-only.** Snowflake does not enforce primary keys and does
  not say which table a result column came from, so there is no row DBDelve can
  safely name. Inserting a row from a table's tab works.
- **Keys show only if they are declared.** Most pipelines never declare them;
  dbt only does under an enforced contract.
- **A filter on a `BINARY` column fails**, including following a foreign key
  whose columns are binary.
- **Explain is not offered.**
- **Several statements in one run** show the last statement's result.
- **`TIMESTAMP_LTZ` is shown in UTC**, marked `Z`.
- A whole result is fetched before any of it is shown, so a very large one
  costs memory and time in proportion.
