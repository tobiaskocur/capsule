<p align="center">
  <img src="https://i.imgur.com/36KqELh.png" alt="Capsule" width="120"/>
</p>

<h1 align="center">Capsule</h1>

<p align="center">
  A lightweight Linux process sandbox written in Rust.
  <br/>
  Run untrusted applications in an isolated environment — without affecting your system.
</p>

***

## What it does

Capsule runs a target process in a sandboxed environment using Linux kernel isolation primitives. The sandboxed process gets its own identity, filesystem view, and process tree, completely separated from the host system.

## How it works

Capsule builds isolation in layers, using native Linux features:

The rootless model is built on user namespaces, namespace-local root maps to your unprivileged host account, so the sandboxed process never gains real host privileges.

## Project structure

```
src/
├── main.rs        — entry point and execution flow
├── errors.rs      — shared error types
├── namespaces.rs  — namespace isolation logic
├── sandbox.rs     — high-level sandbox API (in progress)
└── utils.rs       — utility functions
```

## Current state

The user namespace layer is fully working:

- Creates a new user namespace via `unshare(CLONE_NEWUSER)`
- Configures `uid_map` and `gid_map` for rootless identity mapping
- Verified that the sandboxed process moves into a new isolated namespace

The typical UID/GID mapping looks like this:

```text
0 1000 1
```

Namespace-local UID `0` → host UID `1000`. Root inside, unprivileged outside.

## Roadmap

- [x] User namespace + UID/GID mapping
- [x] Switch to namespace-local root via `setuid(0)` / `setgid(0)`
- [ ] PID namespace isolation
- [ ] Mount namespace + filesystem view control
- [ ] Seccomp BPF syscall filter profile
- [ ] cgroup v2 memory and CPU limits
- [ ] High-level `sandbox run <binary>` CLI

## Requirements

- Linux kernel 5.x+
- Rust 1.70+
- No root privileges required

## Usage

```bash
cargo build
cargo run
```

## License

MIT