upvote-backend
==============
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Backend implementation handling models and routes for `Review`s, `Profile`s, and more.

Server frontend can be found at parent repository: https://github.com/upvote-dev/serve-upvote (clone this one directory above for that to build)

## Docker usage

Install Docker, and then run the following, which will make a server available at http://localhost:3000:
```sh
$ docker compose up
````

NOTE: You may need to configure this for your architecture first, for example:
```sh
$ docker compose build --build-arg ARCH_VARIANT='amd64' \
                       --build-arg ARCH='x86_64'
$ docker compose up
```

Or to work with just one image and provide your own database and redis:
```sh
$ docker build -f 'debian.Dockerfile' -t "${PWD##*/}"':latest' .
$ docker run -e DATABASE_URL="$RDBMS_URI" \
             -e REDIS_URL='localhost:6379' \
             -p '3000:3000' \
             --name 'serve_api' \
             "${PWD##*/}"
```

## Native usage

Install Rust, `git`, and ensure you have your PostgreSQL and Redis/Valkey services setup.

### Environment setup

Add an `.env` file or otherwise add these environment variables; replacing connection strings with what you use:

    DATABASE_URL=postgres://rest_user:rest_pass@localhost/rest_db
    REDIS_URL=redis://127.0.0.1/

### Test

    cargo test

## Contribution guide
Ensure all tests are passing [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html) and [`rustfmt`](https://github.com/rust-lang/rustfmt) has been run. This can be with [`cargo make`](https://github.com/sagiegurari/cargo-make); installable with:

```sh
$ cargo install --force cargo-make
```

Then run:
```sh
$ cargo make
```

Finally, we recommend [feature-branches](https://martinfowler.com/bliki/FeatureBranch.html) with an accompanying [pull-request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests).
</small>

<hr/>

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.
