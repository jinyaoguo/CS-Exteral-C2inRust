# CSEC2 in Rust

An implementation of Cobalt Strike external C2 in Rust, which can avoid killing of firewalls (Windows Defender、Horong Security、360 Defender etc.)

### Start：

1. Set the CS console listen  port.
2. Build the project. `cargo build`

3. Run `ec2_rust.exe` on the target machine，then the CS console will get the 