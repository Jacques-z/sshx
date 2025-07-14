# sshx

## Why fork

The original version of `sshx` has a severe security issue: It writes and write
key in the URL, which enables anyone sees it to run shell commands on your
computer, especially, _the server_.

I wasn't sure about whether it's simply that the author is too lazy to prevent
it or a backdoor until I found him blacklisted me. This is what I did:

- Created 3 PRs:
  - Added a feature of
    [setting custom enc key and write key](https://github.com/ekzhang/sshx/pull/134).
  - Added a feature of
    [requesting custom room name](https://github.com/ekzhang/sshx/pull/135).
  - Added a feature of
    [fill in the write key in frontend prompt](https://github.com/ekzhang/sshx/pull/136)
- Pushed a commit
  [remove back door](https://github.com/Jacques-z/sshx/commit/d1ca797f85a917ef93ab67775ab8985b438f29a5)
  to my personal fork

I guess he behaved this rude because my patch will turn his URL-monitoring
middleware into trash, given that he is incredibly against self-hosting.

So I fix all security concerns I know and try to give instructions of
self-hosting. But you should still be cautious about the code and welcome to
tell me or fix bugs you find. I may create a docker image for self-hosting one
day, or it would be wonderful if someone could do it.

Lastly, please star and share this repo to fight against malware. Thanks💖.

## What is

A secure web-based, collaborative terminal.

![](https://i.imgur.com/Q3qKAHW.png)

**Features:**

- Run a single command to share your terminal with anyone.
- Resize, move windows, and freely zoom and pan on an infinite canvas.
- See other people's cursors moving in real time.
- Connect to the nearest server in a globally distributed mesh.
- End-to-end encryption with Argon2 and AES.
- Automatic reconnection and real-time latency estimates.
- Predictive echo for faster local editing (à la Mosh).

Visit [sshx.io](https://sshx.io) to learn more.

## Installation

Currently I'm afraid you have to compile this by yourself. Detailed info of
compiling will be given below.

Should have had an action to build.

### CI/CD

You can run sshx in continuous integration workflows to help debug tricky
issues, like in GitHub Actions.

```yaml
name: CI
on: push

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      # ... other steps ...

      - run: curl -sSf https://sshx.io/get | sh -s run
      #      ^
      #      └ This will open a remote terminal session and print the URL. It
      #        should take under a second.
```

We don't have a prepackaged action because it's just a single command. It works
anywhere: GitLab CI, CircleCI, Buildkite, CI on your Raspberry Pi, etc.

Be careful adding this to a public GitHub repository, as any user can view the
logs of a CI job while it is running.

## Development

Here's how to work on the project, if you want to contribute.

### Building from source

To build the latest version of the client from source, clone this repository and
run, with [Rust](https://rust-lang.com/) (and maybe `protobuf` or others)
installed:

Firstly, start a server:

```shell
npm install
npm run build
cargo run --bin sshx-server
```

Then client:

```shell
cargo run --bin sshx --server http://127.0.0.1:8051
```

## Deployment

You can easily self host the backend by running `cargo build -r`, run the
executable `target/release/sshx-server`, and bind it to a domain name or open
corresponding port.

If you don't have any domain name or public IP, I recommend
[zrok](https://zrok.io/).
