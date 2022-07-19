use crate::config::SETTINGS;
use crate::Lazy;

use figlet_rs::FIGfont;

pub static FIGFONT: Lazy<FIGfont> = Lazy::new(|| {
    let slant = include_str!("slant.flf");
    let small = include_str!("small.flf");
    let speed = include_str!("speed.flf");
    let starwars = include_str!("starwars.flf");

    FIGfont::from_content(match SETTINGS.app.startup_banner.to_lowercase().as_str() {
        "off" => slant,
        "slant" => slant,
        "small" => small,
        "speed" => speed,
        "starwars" => starwars,
        _ => {
            println!(
                "\n FIGfont \"{}\" not found for banner. Using \"Slant\". Check the settings file.",
                SETTINGS.app.startup_banner,
            );
            slant
        }
    })
    .unwrap()
});
