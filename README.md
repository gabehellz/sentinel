# Night Sentinel

A multi-purpose 1-server Discord bot inspired by the [Night Sentinels](https://doom.fandom.com/wiki/Night_Sentinels) of DOOM.

## Requirements

You need at least:
- Rust 1.19.1+

## Setup

First, clone this repository. Then, copy the `.env.example` file to `.env`:

``` shell
cp .env.example .env
```

And fill all environment variables inside it:

``` dotenv
DISCORD_TOKEN=<paste your bot token here>
GUILD_ID=<paste your Discord server ID here>
```

## Build

Now, you will be able to compile the project:

``` shell
cargo build
```

And run it:

``` shell
cargo run
```
