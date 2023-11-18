#![feature(lazy_cell, ptr_sub_ptr)]

use std::sync::{Mutex, LazyLock};

use skyline::install_hook;
use engage::gamedata::unit::Unit;

use num::clamp;
use serde::{Deserialize, Serialize};

mod sp_bar_1;
mod sp_bar_2;
mod sp_bar_3;

#[derive(Default, Serialize, Deserialize)]
pub struct SpMultiplierValues {
    multiplier_emblem_ring: f32,
    multiplier_bond_ring: f32,
    multiplier_no_ring: f32,
}

impl SpMultiplierValues {
    pub fn save(&self) {
        let out_toml = toml::to_string_pretty(&self).unwrap();
        std::fs::write("sd:/engage/config/sp_multipliers.toml", out_toml).expect("should be able to write to write default configuration");
    }
}

pub static CONFIG: LazyLock<Mutex<SpMultiplierValues>> = LazyLock::new(|| {
    let mut multipliers: SpMultiplierValues = match std::fs::read_to_string("sd:/engage/config/sp_multipliers.toml") {
        Ok(content) => {
            toml::from_str(&content).unwrap_or_else(|err| {
                panic!("Failed to parse TOML: {}", err);
            })
        },
        Err(_) => {
            println!("failed to open the configuration file, generating a default one");

            let multiplier: SpMultiplierValues = SpMultiplierValues{
                multiplier_emblem_ring : 2.0,
                multiplier_bond_ring : 1.0,
                multiplier_no_ring : 0.5,
            };
           
            // Save the configuration we just made
            multiplier.save();
            multiplier
        },
    };

    multipliers.multiplier_emblem_ring = clamp(multipliers.multiplier_emblem_ring, 0.0, 2.0);
    multipliers.multiplier_bond_ring = clamp(multipliers.multiplier_bond_ring, 0.0, 2.0);
    multipliers.multiplier_no_ring = clamp(multipliers.multiplier_no_ring, 0.0, 2.0);

    println!("libspgainmod: emblem ring SP multiplier is {}", multipliers.multiplier_emblem_ring);
    println!("libspgainmod: bond ring SP multiplier is {}", multipliers.multiplier_bond_ring);
    println!("libspgainmod: no ring SP multiplier is {}", multipliers.multiplier_no_ring);
    
    Mutex::new(multipliers)
});

#[unity::hook("App", "Unit", "ExpToSkillPoint")]
pub fn unit_exptoskillpoint(this: &Unit, exp: i32, _method_info : u64) -> i32
{
    let config = CONFIG.lock().unwrap();

    if this.m_GodUnit.is_some()
    {
        (exp as f32 * config.multiplier_emblem_ring) as i32
    }
    else if this.m_Ring.is_some()
    {
        (exp as f32 * config.multiplier_bond_ring) as i32
    }
    else 
    {
        (exp as f32 * config.multiplier_no_ring) as i32
    }
}

#[skyline::main(name = "libspgainmod")]
pub fn main() {
    println!("libspgainmod plugin loaded");

    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        // Some magic thing to turn what was provided to the panic into a string. Don't mind it too much.
        // The message will be stored in the msg variable for you to use.
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        // This creates a new String with a message of your choice, writing the location of the panic and its message inside of it.
        // Note the \0 at the end. This is needed because show_error is a C function and expects a C string.
        // This is actually just a result of bad old code and shouldn't be necessary most of the time.
        let err_msg = format!(
            "Custom plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        // We call the native Error dialog of the Nintendo Switch with this convenient method.
        // The error code is set to 69 because we do need a value, while the first message displays in the popup and the second shows up when pressing Details.
        skyline::error::show_error(
            69,
            "Custom plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));
    
    install_hook!(unit_exptoskillpoint);
    LazyLock::force(&CONFIG);
    sp_bar_1::sp_main_1();
    sp_bar_2::sp_main_2();
    sp_bar_3::sp_main_3();
}
