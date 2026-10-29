# Threat model

Boxwren is software fault isolation for one untrusted WebAssembly guest inside a trusted host process. It is not a virtual machine, a container runtime, or a multi-tenant service.

## Assets

- Host memory and the host process's address space
- Files outside an explicit directory grant
- Network destinations that were not granted
- Environment variables and secrets that were not granted
- CPU and memory on the host, bounded by the limits the embedder left on

## Attacker

The guest module is hostile. It may be agent-generated, prompt-injected, or simply malformed. The embedder that constructs `RunRequest` is trusted. A grant of `~/.ssh` or `/` is the embedder's decision; Boxwren records it and does not rewrite it. Whether the agent is allowed to run code at all is Keywarden's decision, not Boxwren's.

## What the sandbox stops

When limits and the kill switch are left on, and grants stay narrow:

- The guest cannot read or write host memory except through WASI imports.
- The guest cannot open a directory that was not preopened. cap-std rejects `..` that would leave a preopen. The attempt is in the report.
- The guest cannot connect, listen, or send except to an address a `NetGrant` allows. The check runs before the socket system call. DNS is off, so a name cannot be resolved to a new address.
- The guest cannot read the host environment. Only named grants are passed in.
- Fuel, the memory limiter, and the epoch deadline stop a tight loop, a huge `memory.grow`, and a run that exceeds its wall clock. `KillHandle::kill` stops one run without stopping other runs on the same engine.
- Stdio capture drops bytes past the cap instead of letting the guest fill host memory by printing.
- A module larger than the size cap is rejected before compilation.

## What the sandbox does not stop

- A vulnerability in Wasmtime, in Boxwren, or in the host operating system.
- Spectre-class side channels and other microarchitectural leaks. This is not a claim about constant-time execution.
- A grant the embedder chose to issue. A read-write preopen of a home directory can read and write that directory.
- Anything the host process can already do outside the sandbox: other threads, other files, the network stack of the process, credentials in the process environment that the embedder copies into a grant.
- A hostile multi-tenant cloud. Software fault isolation is not a VM. An outer operating-system boundary is defense in depth and is out of scope.
- Compilation cost. Epoch interruption applies after the module is instantiated. A malicious module can still spend CPU while Wasmtime compiles it. The module size cap is the mitigation. Compilation is not cancellable.

## Residual notes

- Preview 1 guests that use `std::fs` may fail inside libc before `path_open` when no preopen matches an absolute path. The host then has no path to record. Guests that must show the attempt in the report call `path_open` themselves. The embedded secret demo does this.
- Preview 1 has no `sock_connect`. Network attempts are journaled for preview 2 components, through the socket address check.
- An implicit bind to an unspecified address and port 0 is allowed so that a later connect can be checked. It is not recorded as egress. Binding a concrete address is checked and journaled.
- If name lookup is turned on in a future version, DNS rebinding becomes a residual risk. It is off.

## Non-goals

Hosted execution, accounts, a web report UI, container orchestration, an embedded Python or JavaScript runtime, Keywarden policy inside the sandbox, and any claim of perfect isolation.
