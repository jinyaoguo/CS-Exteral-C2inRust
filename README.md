# CSEC2 in Rust

An implementation of Cobalt Strike (CS) external C2 in Rust, which can avoid killing of firewalls (Windows Defender、Horong Security、360 Defender etc). This repo only contains the client code. You still need to supplement a sevrer to communicate with the client.

### Start：

1. Set the listen port in CS console.

2. Modify the hyfer-parameter of client code (pipeline name & IP)

3. Build the project. run `cargo build`

4. Run `ec2_rust.exe` on the target machine，then you will see the target machine appears on the CS console.