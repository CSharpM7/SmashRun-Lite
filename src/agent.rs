use super::*;

unsafe fn agent_start(fighter: &mut L2CFighterCommon)
{
    let entry = get_entry_from_boma(fighter.module_accessor) as usize;
    vars::SPIRIT_TYPE[entry] = -1;

    if is_Results()
    || is_training_mode() {
        println!("SmashRun Buffs Reset");
        vars::LEVEL_ATTACK[entry] = 1;
        vars::LEVEL_SPEED[entry] = 1;
        vars::LEVEL_DEF[entry] = 1;
        vars::LEVEL_JUMP[entry] = 1;
        vars::LEVEL_TURBO[entry] = 1;
    }
    else
    {
        if vars::LEVEL_ATTACK[entry] > 1
        || vars::LEVEL_SPEED[entry] > 1
        || vars::LEVEL_DEF[entry] > 1
        || vars::LEVEL_JUMP[entry] > 1
        || vars::LEVEL_TURBO[entry] > 1
        {
            println!("SmashRun Buffs Reloaded");
            let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
            let rot = Vector3f{x: 0.0, y: 0.0, z: 0.0};
            let size = 1.25;
            let time = 30;

            let mut eff = "sys_special_all_up";
            let mut eff_color = Vector3f{x: 1.0, y: 1.0, z: 1.0};
            
            EffectModule::req_time_follow(fighter.module_accessor, Hash40::new(eff), Hash40::new("hip"), time, &pos, &rot, size, false, 0);
            LAST_EFFECT_SET_COLOR(fighter,eff_color.x,eff_color.y,eff_color.z);
        }
    }
}

//#[smashline::fighter_init]
extern "C" fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}
//#[fighter_reset]
extern "C" fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        if is_Results()
        || is_training_mode() {
            agent_start(fighter);
        }
    }
}

pub fn install() {
    Agent::new("fighter")
        .on_start(agent_init)
        .on_start(agent_reset)
        .install();
}