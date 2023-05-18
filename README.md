# Secret Store

Small GUI local secrets management tool. Secrets are encrypted and pushed to a local sqlite database which should not be revealed with anyone.

### Changelog

- v 0.1.0 - inital functionality added

# Installation

To install the tool, it's enough to clone the repo, run cargo build and it would generate the executable.

```bash
cargo build --release
```

# Usage

In order to assure the tool is working fine, in the executor path we should have a `db` folder with a `secrets.db` file to be used to store secrets.
Also, it's important to choose a master password that would be used to encrypt and decrypt secrets. This should not be shared with anyone and should be used for all secrets for consistency.

# Contributing

If you wish to contribute to the project, don't hesitate to open an issue/comment on available issues. Every change should be done through pull requests and code review.