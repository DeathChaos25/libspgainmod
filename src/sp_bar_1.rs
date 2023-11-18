use unity::prelude::*;
use engage::menu::{
    BasicMenuResult,
    config::{
        ConfigBasicMenuItem,
        ConfigBasicMenuItemGaugeMethods
    }
};

use crate::CONFIG;

pub struct SPMultiGod;

impl ConfigBasicMenuItemGaugeMethods for SPMultiGod {
    fn init_content(this: &mut ConfigBasicMenuItem) {
        let config = CONFIG.lock().unwrap();
        this.gauge_ratio = config.multiplier_emblem_ring / 2.0;
    }

    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let mut config = CONFIG.lock().unwrap();

        let result = ConfigBasicMenuItem::change_key_value_f(config.multiplier_emblem_ring/ 2.0, 0.0, 1.0, 0.05);
    
        if config.multiplier_emblem_ring != ((result * 100.0).trunc() / 100.0) * 2.0 {
            // the bar can only visually fill from 0 to 1, so we get around this
            // by using a decimal value that takes an equal amount of steps
            // to reach max value as it would from 0 to 1 and then multiply
            // that value to get our real desired value

            let newresult = (result * 100.0).trunc() / 100.0;

            this.gauge_ratio = newresult;
            config.multiplier_emblem_ring = newresult * 2.0;
            
            Self::set_help_text(this, None);

            this.update_text();

            config.save();

            BasicMenuResult::se_cursor()
        } else {
            BasicMenuResult::new()
        }
    }

    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        this.help_text = format!("Current SP Multiplier: {:.2}", this.gauge_ratio * 2.0).into();
    }
}

#[no_mangle]
extern "C" fn sp_multi_god_callback() -> &'static mut ConfigBasicMenuItem {
    ConfigBasicMenuItem::new_gauge::<SPMultiGod>("SP Multiplier: Emblem Ring")
}

pub fn sp_main_1() {
    cobapi::install_game_setting(sp_multi_god_callback);
}