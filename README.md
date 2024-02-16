# Melody

Melody is an API to access Develop for Good AI services. It can be accessed
through the Almond UI (not currently online).

Built with love, with Rust.

## Getting Started

- Install rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Use nightly toolchain for rust: `rustup toolchain install nightly`
- Make nightly your default toolchain: `rustup default nightly`
- Install cargo watch: `cargo install cargo-watch`

- Install docker
- Start the docker containers: `docker compose up` (if you run into issues and
  want to clean restart, use `docker compose down && docker compose up --build`)

- Create a `.env` file following the `.env.example`. Set the values.

- Start the rust server in watch mode: `cargo watch -x run`

#### NOTES

- Set the AUTH_PROVIDER value in .env to `noop` to disable authentication. Use
  `auth0` if you have valid Auth0 credentials

## Contributing

1. Fork it
2. Create your feature branch: `git checkout -b my-new-feature`
3. Commit your changes: `git commit -am "Add some feature"`
4. Push to your branch: `git push origin my-new-feature`
5. Submit a pull request

## License

The MIT License (MIT)

Copyright (c) 2024 Anish Sinha

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
