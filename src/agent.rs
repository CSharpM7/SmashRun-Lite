use super::*;

unsafe fn agent_start(fighter: &mut L2CFighterCommon)
{
    let entry = get_entry_from_boma(fighter.module_accessor) as usize;
    vars::SPIRIT_TYPE[entry] = -1;

    println!(
        "Pear {}",*ITEM_VARIATION_TABEMONO_PEAR
    );
    println!(
        "Kiwi {}",*ITEM_VARIATION_TABEMONO_KIWI
    );

    if is_Results()
    || is_training_mode() {
        vars::LEVEL_ATTACK[entry] = 1;
        vars::LEVEL_SPEED[entry] = 1;
        vars::LEVEL_DEF[entry] = 1;
        vars::LEVEL_JUMP[entry] = 1;
        vars::LEVEL_TURBO[entry] = 1;
    }
}

#[smashline::fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}
#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}

pub fn install() {
    smashline::install_agent_init_callbacks!(
        agent_init
    );
    install_agent_resets!(
        agent_reset
    );
}