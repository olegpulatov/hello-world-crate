# hello-world-crate

A minimal Rust library for learning the crates.io publishing process. No dependencies.

```rust
use hello_world_crate::hello;

assert_eq!(hello(), "Hello, world!");
```

`hello()` returns a static string without allocating memory.

## local checks

```fish
cargo fmt --check
cargo test
cargo package --list
cargo publish --dry-run --allow-dirty
```

The dry run builds the packaged crate but does not upload it. `--allow-dirty` is
only for checking local, uncommitted changes; omit it after committing.

## first publication

- Check that `hello-world-crate` is available on crates.io. Names are shared by
  all users. If needed, change `package.name` in `Cargo.toml` and update the Rust
  import in this readme and the examples in `src/lib.rs` (hyphens become underscores)
- Review the MIT license and package contents before uploading. `Cargo.toml`
  points to the planned GitHub repository; create it before publishing, or update
  `repository` if you choose a different owner or name
- Sign in at https://crates.io/ with GitHub and verify your email at
  https://crates.io/settings/profile
- Create a short-lived token at https://crates.io/settings/tokens with only the
  permissions needed to publish this crate. Do not put it in this repository or chat
- Run `cargo login` and paste the token into its prompt. Cargo stores it locally
- Run the local checks above, then commit the release files

```fish
git add .gitignore Cargo.toml src/lib.rs README.md LICENSE
git commit -m "Prepare initial crate release"
cargo publish --dry-run
```

To create a public GitHub repository from this folder and push the committed source:

```fish
gh repo create hello-world-crate --public --source=. --remote=origin --push
```

This creates the repository under your authenticated GitHub account. Use
`--private` instead if the source repository should be private; the code uploaded
to crates.io will still be public. GitHub hosting is not required by crates.io.

Only after reviewing the archive and deciding to publish publicly:

```fish
cargo publish
git tag v0.1.0
```

Publication is public and generally permanent: the same version cannot be
replaced. `cargo yank` prevents new dependency resolutions from choosing a
version; it does not remove the uploaded code. Never package secrets.

After publication, test consumption from a separate project:

```fish
cargo new --bin /tmp/hello-world-crate-consumer
cd /tmp/hello-world-crate-consumer
cargo add hello-world-crate@0.1.0
```

Use the example above in the consumer's `src/main.rs`, inside `fn main()`, and run
`cargo run`. Substitute your chosen package name if you renamed it. API docs are
normally built automatically at https://docs.rs/hello-world-crate after publishing.

For a second release, change `package.version` in `Cargo.toml` to `0.1.1`, run the
checks, commit, publish, and tag `v0.1.1`. Follow semantic versioning for later API changes.

Official publishing guide: https://doc.rust-lang.org/cargo/reference/publishing.html

## license

MIT; see `LICENSE`.
