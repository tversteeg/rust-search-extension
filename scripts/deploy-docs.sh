#!/usr/bin/env bash
set -e

BRANCH="now"

build() {
  echo "Starting building..."

  cd docs
  sudo snap install --edge zola
  zola build
  mv public /tmp/public
  cd ..
  RUST_BACKTRACE=full cargo run --target-dir /tmp --manifest-path=rust/Cargo.toml books -d /tmp/public/index/books.js
  RUST_BACKTRACE=full cargo run --target-dir /tmp --manifest-path=rust/Cargo.toml lints -d /tmp/public/index/lints.js
  RUST_BACKTRACE=full cargo run --target-dir /tmp --manifest-path=rust/Cargo.toml labels -d /tmp/public/index/labels.js
  RUST_BACKTRACE=full cargo run --target-dir /tmp --manifest-path=rust/Cargo.toml caniuse -r /tmp/caniuse -d /tmp/public/index/caniuse.js
  RUST_BACKTRACE=full cargo run --target-dir /tmp --manifest-path=rust/Cargo.toml blog_urls -d /tmp/public/rust-blog-urls.json
}

deploy() {
  echo "Starting deploying..."
  git config --global url."https://".insteadOf git://
  git config --global url."https://github.com/".insteadOf git@github.com:

  git checkout ${BRANCH}
  cp -vr /tmp/public/* .
  git config user.name "GitHub Actions"
  git config user.email "github-actions-bot@users.noreply.github.com"
  git add .
  git commit -m "Deploy new version docs"
  git push --force "https://${GITHUB_TOKEN}@github.com/${GITHUB_REPOSITORY}.git" ${BRANCH}

  echo "Deploy complete"
}

build
deploy