use cmd_lib::*;
use std::env::consts::OS;
use sys_info::{linux_os_release, LinuxOSReleaseInfo};


fn check_system() -> String {
    let windows_message = "Ha ha ha windows peasants !";

    let apple_messsage = "iToddlers BTFO !";

    let other_unix_message = "at least you don't use apple but wrong programme";

    let result = match OS {
        "windows" => windows_message,
        "apple" | "macos" => apple_messsage,
        "linux" => "linux",
        _ => other_unix_message,
    };

    result.to_string()
}

pub fn message() {
    let current_os = check_system();

    let linux_check = "linux";

    if current_os == linux_check.to_string() {
        let distro = linux_os_release().unwrap();

        if LinuxOSReleaseInfo::name(&distro) == "Arch Linux" {
            let last_update = run_fun!(sed -n r#"/pacman -Syu/h; ${x;s/.\([0-9-]*\).*/\1/p;}"# /var/log/pacman.log).unwrap();

            println!(
                "Hello welcome to the arch system maintainer your last update was {}",
                last_update
            );
        } else {
            println!("so close to gretness, use Arch BTW");
        }
    } else {
        println!("{}", current_os)
    }
}
