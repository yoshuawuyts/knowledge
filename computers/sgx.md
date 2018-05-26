# SGX
CPU mode which can protect data, and authorize data access. Allows applications
to protect it's own memory from the OS, VM, Hypervisor. This is enforced by
Intel's (hardware) Memory Encryption Engine.

In practice it's used like escalating privilege to ring 0; but instead it's
enforced by hardware.

Remote Attestation is done by the SGX computing a hash value of data passed,
which allows you to verify it's running the right code.

## Pitch
- High performance memory encryption engine
- Local/Remote attestation support
- Best choice for "My algorithm, your data, whoever's CPU"
- And Secure-Multiparty-Computing
- Previous sample: PowerDVD, WolfSSL

## Limitations
- Supported amount of memory is limited (128GB).

## See Also
- https://github.com/baidu/rust-sgx-sdk
