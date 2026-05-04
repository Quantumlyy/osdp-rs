# Security Policy

The `osdp` crate implements the SIA Open Supervised Device Protocol (OSDP)
v2.2, including its AES-128 secure-channel session layer used by physical
access-control devices. Because the crate is part of a security-relevant stack,
we take vulnerability reports seriously and aim to handle them through
coordinated disclosure.

## Supported Versions

`osdp-rs` is pre-1.0 software. Only the latest published minor release line
receives security fixes; once a new minor line is released, the previous one is
no longer supported.

| Version | Supported          |
| ------- | ------------------ |
| 0.2.x   | :white_check_mark: |
| < 0.2   | :x:                |

When `0.3.0` is released, `0.2.x` will move to unsupported. The `1.0` release
will switch this project to standard semver-based support windows; this policy
will be updated at that time.

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues,
pull requests, or discussions.**

Use one of the following private channels:

1. **Preferred — GitHub private vulnerability report:**
   <https://github.com/Quantumlyy/osdp-rs/security/advisories/new>
2. **Email fallback:** `nejc@nejc.dev`

Please include as much of the following as you can:

- The affected `osdp` version (and Rust toolchain, if relevant)
- A description of the issue and its impact (confidentiality, integrity,
  availability, secure-channel compromise, etc.)
- Steps to reproduce, ideally with a minimal proof of concept
- Any known mitigations or workarounds

### What to expect

- **Acknowledgement** of your report within **7 days**.
- **Initial triage update** (accepted, needs more info, or declined) within
  **14 days**.
- For accepted reports, we will work with you on a coordinated disclosure
  timeline. A patched release will be published to crates.io and an advisory
  filed in the [GitHub Security Advisories](https://github.com/Quantumlyy/osdp-rs/security/advisories)
  database and the [RustSec Advisory Database](https://rustsec.org/) before
  public disclosure.
- Reporters are credited in the advisory unless they request to remain
  anonymous.
- For declined reports, we will explain the reasoning so you can follow up if
  you disagree.

### Scope

In scope:

- Vulnerabilities in the `osdp` crate's source code (parsing, secure channel,
  state machine, command/reply handling, MAC/encryption use, etc.).

Out of scope:

- Vulnerabilities in third-party dependencies — please report those upstream
  and to [RustSec](https://rustsec.org/).
- Vulnerabilities in downstream applications that use this crate.
- Issues that require a malicious or already-compromised peer where the
  protocol itself does not claim a defense (consult OSDP v2.2 §A.5 threat
  model).
