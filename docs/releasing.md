# Releasing

Releases are built by `.github/workflows/release.yml` from a version tag.

## Cut a release

1. Set the version in `Cargo.toml` (`[workspace.package] version` and the six
   `version = "..."` entries of the internal crates under `[workspace.dependencies]`).
2. `cargo build` (updates `Cargo.lock`), commit, and push to `main`. Wait for CI to pass:
   it runs the full test suite on all six targets.
3. Tag and push: `git tag v0.1.0 && git push origin v0.1.0`.

The workflow then:

| Job | What it does |
|---|---|
| build | Release binary per target on a native runner, a smoke test (`init`, `status`, `explain` on a fresh repo), and an archive: `myrn-<target>.tar.gz` (Linux, macOS) or `.zip` (Windows) with the binary, `LICENSE` and `README.md` |
| release | `sha256sums.txt`, the manifests from `dist/render.py`, and a GitHub release with the archives, checksums, `install.sh`, `install.ps1`, `myrn.json` (Scoop), `myrn.rb` (Homebrew) and `winget-manifests.zip` |
| npm | `npm publish` of the wrapper in `dist/npm` at the release version |
| crates | `cargo publish --workspace --exclude myrn-bench` (dependencies first) |
| homebrew | Copies `myrn.rb` to `Formula/myrn.rb` in the tap and pushes |

Run the workflow by hand (Actions → release → Run workflow) to build and render
everything without a tag; nothing is published then.

| Target | Runner |
|---|---|
| `x86_64-unknown-linux-musl` | `ubuntu-latest` (static, runs on any distribution) |
| `aarch64-unknown-linux-musl` | `ubuntu-24.04-arm` |
| `x86_64-apple-darwin` | `macos-15-intel` |
| `aarch64-apple-darwin` | `macos-latest` |
| `x86_64-pc-windows-msvc` | `windows-latest` |
| `aarch64-pc-windows-msvc` | `windows-11-arm` |

The ARM runners are free for public repositories. If GitHub renames a runner
label (the Intel macOS image is the one most likely to change), update it in
both `ci.yml` and `release.yml`.

## One-time setup

Each publishing job skips itself, with a message, until its secret exists
(Settings → Secrets and variables → Actions):

| Secret | For | How |
|---|---|---|
| `NPM_TOKEN` | npm | An npm automation token with publish rights to the `myrn` package |
| `CARGO_REGISTRY_TOKEN` | crates.io | A crates.io API token with `publish-new` and `publish-update` |
| `HOMEBREW_TAP_TOKEN` | Homebrew | A fine-grained GitHub token with contents write on the tap repository |

The tap is `<owner>/homebrew-tap` unless the repository variable `HOMEBREW_TAP`
names another. Create it once as an empty public repository; users then run
`brew install NickStrain/tap/myrn`.

**Scoop** needs no setup: the release carries `myrn.json`, so
`scoop install https://github.com/NickStrain/Myrn-/releases/latest/download/myrn.json`
works, and `checkver`/`autoupdate` let a bucket track new versions.

**winget** takes manifests by pull request to `microsoft/winget-pkgs`. Unzip
`winget-manifests.zip` from the release and submit it, for example with
`wingetcreate submit <folder>`, or put the three files under
`manifests/n/NickStrain/Myrn/<version>/` in a fork and open the PR. The first
submission is reviewed by hand; later versions can be automated.

**Name**: `myrn` is still a working name. Before the first public release, check
that it is free on crates.io, npm and Homebrew (see the open questions at the
end of `docs/BUILD_PROMPTS.md`). Renaming touches the crate names, the npm
`name`, `dist/render.py` and the install scripts.

## Install paths this produces

| Platform | Command |
|---|---|
| Linux, macOS | `curl -fsSL https://github.com/NickStrain/Myrn-/releases/latest/download/install.sh \| sh` |
| Windows | `irm https://github.com/NickStrain/Myrn-/releases/latest/download/install.ps1 \| iex` |
| Homebrew | `brew install NickStrain/tap/myrn` |
| Scoop | `scoop install https://github.com/NickStrain/Myrn-/releases/latest/download/myrn.json` |
| winget | `winget install NickStrain.Myrn` (after the manifests are merged) |
| npm | `npm install -g myrn` |
| Cargo | `cargo install myrn`, or from source: `cargo install --git https://github.com/NickStrain/Myrn- myrn` |

The install scripts verify the download against `sha256sums.txt` and install
per user, without administrator rights (`~/.local/bin`, or
`%LOCALAPPDATA%\Programs\myrn` added to the user PATH). The npm wrapper
downloads and verifies the same archive on install, or on first run when the
package manager skips install scripts (npm 12 does by default for dependencies).
