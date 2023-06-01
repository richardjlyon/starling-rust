# starling-rust

A command line utility for analysing rust transactions


## Configuration

Create a file `config.yaml` with the following content:

```
token:
  - person: "XXX"
  - business: "YYY"
db:
  user: "admin"
  password: "ZZZ"
  name: starling_db
```

## Use

```
$ money db init
$ money account add [token]
$ money account list
$ money transactions update

```