/**
 * system_shutdown = "4.0.1"
 *
 * 提供了一种跨平台的方式来执行关机、重启或注销操作。
 */
#[cfg(test)]
mod shutdown_test {
    use system_shutdown::{
        force_logout, force_reboot, force_shutdown, hibernate, logout, reboot, shutdown, sleep,
    };

    #[test]
    fn shutdown_test() {
        // 关机
        match shutdown() {
            Ok(_) => println!("bye bye"),
            Err(e) => println!("shutdown error: {:?}", e),
        }
    }

    #[test]
    fn reboot_test() {
        // 重启
        match reboot() {
            Ok(_) => println!("reboot"),
            Err(e) => println!("reboot error: {:?}", e),
        }
    }

    #[test]
    fn logout_test() {
        // 注销
        match logout() {
            Ok(_) => println!("logout"),
            Err(e) => println!("logout error: {:?}", e),
        }
    }

    #[test]
    fn hibernate_test() {
        // 休眠
        match hibernate() {
            Ok(_) => println!("hibernate"),
            Err(e) => println!("hibernate error: {:?}", e),
        }
    }

    #[test]
    fn sleep_test() {
        // 睡眠
        match sleep() {
            Ok(_) => println!("sleep"),
            Err(e) => println!("sleep error: {:?}", e),
        }
    }

    #[test]
    fn force_logout_test() {
        // 强制退出
        match force_logout() {
            Ok(_) => println!("force logout"),
            Err(e) => println!("force logout error: {:?}", e),
        }
    }

    #[test]
    fn force_shutdown_test() {
        // 强制关机
        match force_shutdown() {
            Ok(_) => println!("force shutdown"),
            Err(e) => println!("force shutdown error: {:?}", e),
        }
    }

    #[test]
    fn force_reboot_test() {
        // 强制重启
        match force_reboot() {
            Ok(_) => println!("force reboot"),
            Err(e) => println!("force reboot error: {:?}", e),
        }
    }
}
