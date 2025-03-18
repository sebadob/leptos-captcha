set shell := ["bash", "-uc"]

export TAG := `cat Cargo.toml | grep '^version =' | cut -d " " -f3 | xargs`

# install needed cargo utitlies
setup:
    #!/usr/bin/env bash
    set -euxo pipefail
    clear

    cargo install cargo-msrv


clippy:
    #!/usr/bin/env bash
    set -euxo pipefail
    clear

    cargo clippy


# clippy lint + check with minimal versions from nightly
check:
    #!/usr/bin/env bash
    set -euxo pipefail
    clear
    cargo update

    echo 'Clippy with defaults'
    cargo +nightly clippy -- -D warnings


# prints out the currently set version
version:
    echo {{TAG}}


# builds the code
build:
    #!/usr/bin/env bash
    set -euxo pipefail
    cargo build --release --target wasm32-unknown-unknown


# runs the full set of tests
test:
    #!/usr/bin/env bash
    set -euxo pipefail
    clear
    cargo test


# verifies the MSRV
msrv-verify:
    cargo msrv verify


# find's the new MSRV, if it needs a bump
msrv-find:
    cargo msrv find --min 1.81.0


# verify thats everything is good
verify: check test build msrv-verify


# makes sure everything is fine
verify-is-clean: verify
    #!/usr/bin/env bash
    set -euxo pipefail

    # make sure everything has been committed
    git diff --exit-code

    echo all good


# sets a new git tag and pushes it
release: verify-is-clean
    #!/usr/bin/env bash
    set -euxo pipefail

    # make sure git is clean
    git diff --quiet || exit 1

    git tag "v$TAG"
    git push origin "v$TAG"


# publishes the current version to cargo.io
publish: verify-is-clean
    #!/usr/bin/env bash
    set -euxo pipefail
    cargo publish