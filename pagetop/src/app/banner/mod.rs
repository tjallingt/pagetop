use crate::Lazy;
use crate::config::SETTINGS;

use figlet_rs::FIGfont;

static FIGFONT: Lazy<FIGfont> = Lazy::new(|| {
    let slant    = include_str!("slant.flf");
    let small    = include_str!("small.flf");
    let speed    = include_str!("speed.flf");
    let starwars = include_str!("starwars.flf");

    FIGfont::from_content(
        match SETTINGS.app.startup_banner.to_lowercase().as_str() {
            "off"      => slant,
            "slant"    => slant,
            "small"    => small,
            "speed"    => speed,
            "starwars" => starwars,
            _ => {
                println!(
                    "\n FIGfont \"{}\" not found for banner. {}. {}.",
                    SETTINGS.app.startup_banner,
                    "Using \"Slant\"",
                    "Check the settings file",
                );
                slant
            }
        }
    ).unwrap()
});

pub fn print_on_startup() {
    if SETTINGS.app.startup_banner.to_lowercase() != "off" {
        println!("\n{} {}\n\n Powered by PageTop {}\n",
            FIGFONT.convert(&SETTINGS.app.name).unwrap(),
            &SETTINGS.app.description,
            env!("CARGO_PKG_VERSION")
        );
    }
}
