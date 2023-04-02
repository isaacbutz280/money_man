//! Demo app for egui

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::path::PathBuf;

// When compiling natively:
fn main() {
    {
        // Silence wgpu log spam (https://github.com/gfx-rs/wgpu/issues/3206)
        let mut rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned());
        for loud_crate in ["naga", "wgpu_core", "wgpu_hal"] {
            if !rust_log.contains(&format!("{loud_crate}=")) {
                rust_log += &format!(",{loud_crate}=warn");
            }
        }
        std::env::set_var("RUST_LOG", rust_log);
    }


    let options = eframe::NativeOptions {
        drag_and_drop_support: true,

        initial_window_size: Some([900.0, 500.0].into()),

        #[cfg(feature = "wgpu")]
        renderer: eframe::Renderer::Wgpu,

        ..Default::default()
    };


    let acc_path = PathBuf::from("C:\\Users\\Isaac\\Git_Repos\\money_man\\app\\acc\\isaac.json");
    let a = app::Account::from(acc_path);

    let b = match a {
        Ok(a) => a,
        Err(e) => {
            println!("{}", e);
            app::Account::default()
        },
    };

    eframe::run_native(
        "Money Man",
        options,
        Box::new(|cc| Box::new(gui::wrap_app::WrapApp::new(cc, b))),
    )
}


// fn main() {
//     let acc_path = PathBuf::from("C:\\Users\\Isaac\\Git_Repos\\money_man\\app\\acc\\isaac.json");

//     let a = money_man::Account::from(acc_path);

//     if let Err(e) = a {
//         log_error(Box::from(e));
//         process::exit(0)
//     } 
//     else if let Err(e) = cmd_loop(& mut a.unwrap()) {
//         log_error(Box::from(e));
//         process::exit(0)
//     }
//     else {
//         process::exit(-1)
//     }
// }

// fn cmd_loop(acc: &mut Account) -> Result<(), Box<dyn Error>> {
//     let mut b_flag = true;

//     while b_flag {  // This is 
//         let actions: Vec<Box<dyn Action>> = vec![
//             actions::View::new(),
//             actions::Add::new(),
//             actions::Remove::new(),
//             actions::Transfer::new(),
//             actions::Categorize::new(),
//             // actions::Rebalance::new(),
//             // actions::HardReset::new(),
//             actions::Quit::new(),
//         ];
//         let ans = inquire::Select::new("\nWhat can I do for you today?", actions).prompt()?;
//         b_flag = ans.action(acc)?;
//     }

//     println!("Good bye!");

//     Ok(())
// }

// fn log_error(e: Box<dyn Error>) {
//     println!("FAILURE: {}\n Press ENTER key to exit...", e);
//     let mut _buf = String::new();
//     let _res = std::io::stdin().read_line(&mut _buf);
// }