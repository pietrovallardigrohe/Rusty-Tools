pub fn clear_terminal() -> i32 {
    use std::process::Command;
    let exit_code = if cfg!(windows) {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
                
    } else {
        Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
    };

    match exit_code {
        Ok(exit_status) => {
            //println!("{:?}", exit_status.code().expect("Exited by signal"));
            exit_status.code().expect("Exited by signal")
        },
        Err(_) => {
            println!("failed to wait");
            -1
        }
    }
    // println!("{:?}", exit_status);
}

// https://stackoverflow.com/a/55041833
pub fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_terminal_test() {
        assert!(clear_terminal() == 0);
        // assert_eq!(result, 4);
    }
}
