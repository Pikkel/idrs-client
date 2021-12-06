use std::process::Command;

fn main() {
    if cfg!(unix) {
        let balls = Command::new("cat").arg("/etc/machine-id").output().unwrap();
        let titties = String::from_utf8_lossy(&balls.stdout);
        println!("{}", titties.trim());
    } else if cfg!(windows) {
        let balls = Command::new(
            "reg query HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Cryptography /v MachineGuid",
        )
        .output()
        .unwrap();
        let titties = String::from_utf8_lossy(&balls.stdout);
        println!("{}", titties.trim());
    } else if cfg!(target_os = "macos") {
        let balls = Command::new("ioreg -rd1 -c IOPlatformExpertDevice | grep IOPlatformUUID")
            .output()
            .unwrap();
        let titties = String::from_utf8_lossy(&balls.stdout);
        println!("{}", titties.trim());
    }
}
