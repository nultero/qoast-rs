use std::env::{current_dir, var};

const ARROW: &'static str = "❯➤";
const TILDE: &'static str = "~";
const HOME: &'static str = "HOME";

//  $? turns out to be kind of unreliable
const RET_CODE: &'static str = "ZSH_LAST_RETVAL";

fn get_home() -> String {
    let h = var(HOME).expect("err getting homedir");
    return h;
}

fn last_ret_code() -> u8 {
    let rtc_v = var(RET_CODE);
    match rtc_v {
        Ok(rtc_str) => {
            let rtc = rtc_str.parse::<u8>().expect("err converting last return code");
            return rtc;
        },
        
        Err(e) => {
            println!("\x1b[31m promptvar err => \x1b[0m {e:?}");
            return 0;
        }
    }
}

fn main() {
    let home = get_home();
    let d = current_dir().expect("err getting current dir");
    let mut cwd = d.to_str()
                .expect("cwd str conversion error")
                .to_owned();

    cwd = cwd.replace(&home, TILDE);    
    println!("\n{cwd}");

    let rtc = last_ret_code();
    if rtc == 0 {
        println!("\x1b[34m{ARROW}\x1b[0m  ");
    } else {
        println!("\x1b[31m{ARROW}\x1b[0m  ");
    }
}
