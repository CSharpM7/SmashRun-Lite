use super::*;
/*
#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
pub unsafe fn fighter_pickup_light(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry = get_entry_from_boma(fighter.module_accessor) as usize;
    if vars::SPIRIT_TYPE[entry] >= 0 &&
    //MotionModule::frame(fighter.module_accessor) >= 6.0 && 
    (   !ItemModule::is_have_item(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_TEMPORARY)
        || ItemModule::is_have_item(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN)
    ) 
    {
        let tabeType = vars::SPIRIT_TYPE[entry];
        if ItemModule::is_have_item(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN)
        {
            //macros::EFFECT(fighter, Hash40::new("sys_item_arrival"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
            ItemModule::remove_item(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN);
        }
        let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        let rot = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        let size = 1.25;
        let time = 30;

        if tabeType == 100 {
            tabeType = app::sv_math::rand(hash40("fighter"), 37);
        }
        let is_All = [*ITEM_VARIATION_TABEMONO_PEACH,*ITEM_VARIATION_TABEMONO_DAISY,101].contains(&tabeType);
        let is_Attack = [0,1,2,3,4,5,6].contains(&tabeType) && vars::LEVEL_ATTACK[entry] < vars::LEVEL_MAX;
        let is_Def = [7,8,9,10,11,12,13].contains(&tabeType) && vars::LEVEL_DEF[entry] < vars::LEVEL_MAX;
        let is_Speed = [14,15,16,17,18,19,20].contains(&tabeType) && vars::LEVEL_SPEED[entry] < vars::LEVEL_MAX;
        let is_Jump = [21,22,23,24,25,26,27].contains(&tabeType) && vars::LEVEL_JUMP[entry] < vars::LEVEL_MAX;
        let is_Turbo = tabeType >= 28 && tabeType < 100 && vars::LEVEL_TURBO[entry] < vars::LEVEL_MAX;

        let mut eff = "sys_special_all_up";
        let mut eff_color = Vector3f{x: 1.0, y: 1.0, z: 1.0};
        //Attack
        if is_Attack || is_All
        {
            //sys_flies_up
            eff = "sys_special_attack_up";
            vars::LEVEL_ATTACK[entry] = vars::LEVEL_MAX.min(vars::LEVEL_ATTACK[entry]+1);
            println!("Food type: {}, Attack level: {}",tabeType,vars::LEVEL_ATTACK[entry]);
        }
        //Def
        if is_Def || is_All
        {
            eff_color.z=0.0;
            vars::LEVEL_DEF[entry] = vars::LEVEL_MAX.min(vars::LEVEL_DEF[entry]+1);
            println!("Food type: {}, Def level: {}",tabeType,vars::LEVEL_DEF[entry]);
        }
        //Speed
        if is_Speed || is_All
        {
            eff = "sys_special_defense_up";
            vars::LEVEL_SPEED[entry] = vars::LEVEL_MAX.min(vars::LEVEL_SPEED[entry]+1);
            println!("Food type: {}, Speed level: {}",tabeType,vars::LEVEL_SPEED[entry]);
        }
        //Jump
        if is_Jump || is_All
        {
            eff = "sys_special_speed_up";
            vars::LEVEL_JUMP[entry] = vars::LEVEL_MAX.min(vars::LEVEL_JUMP[entry]+1);
            println!("Food type: {}, Jump level: {}",tabeType,vars::LEVEL_JUMP[entry]);
        }
        //Turbo
        if is_Turbo || is_All
        {
            eff_color.y=0.0;
            vars::LEVEL_TURBO[entry] = vars::LEVEL_MAX.min(vars::LEVEL_TURBO[entry]+1);
            println!("Food type: {}, Turbo level: {}",tabeType,vars::LEVEL_TURBO[entry]);
        }
        if is_All
        {
            eff = "sys_special_all_up";
            eff_color = Vector3f{x: 1.0, y: 1.0, z: 1.0};
        }
        EffectModule::req_time_follow(fighter.module_accessor, Hash40::new(eff), Hash40::new("hip"), time, &pos, &rot, size, false, 0);
        LAST_EFFECT_SET_COLOR(fighter,eff_color.x,eff_color.y,eff_color.z);

        vars::SPIRIT_TYPE[entry] = -1;
    }
    else if (vars::SPIRIT_TYPE[entry] <= 0 &&
    MotionModule::frame(fighter.module_accessor) >= 2.0 &&
    MotionModule::frame(fighter.module_accessor) < 5.0)
    {
        let write_text = format!("Main: {} Temp {} Curry: {}",
        ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN),
        ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_TEMPORARY),
        *ITEM_KIND_CURRY);
        //std::fs::write("sd:/ultimate/mods/SmashRun/log.txt",write_text);
        print(write_text);

        if ((ItemModule::get_pickable_item_kind(fighter.module_accessor) as i32) == *ITEM_KIND_TABEMONO)
        || ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_TEMPORARY) == *ITEM_KIND_TABEMONO
        {
            let item_id = ItemModule::get_have_item_id(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
            let item = get_battle_object_from_id(item_id as u32);
            let variant = WorkModule::get_int((*item).module_accessor, *ITEM_INSTANCE_WORK_INT_VARIATION);
            vars::SPIRIT_TYPE[entry] = variant;
        }
        //It's about to get YandereDev in here
        else if ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_TEMPORARY) == *ITEM_KIND_MAXIMTOMATO
        {
            vars::SPIRIT_TYPE[entry] = 100;
        }
        else if ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_TEMPORARY) == *ITEM_KIND_CURRY
        {
            vars::SPIRIT_TYPE[entry] = 1;
        }
        else if ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN) == *ITEM_KIND_SMOKESCREEN
        {
            vars::SPIRIT_TYPE[entry] = 7;
        }
        else if ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN) == *ITEM_KIND_BANANAGUN
        {
            vars::SPIRIT_TYPE[entry] = 14;
        }
        else if ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN) == *ITEM_KIND_BEETLE
        {
            vars::SPIRIT_TYPE[entry] = 21;
        }
        else if ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN) == *ITEM_KIND_STEELDIVER
        {
            vars::SPIRIT_TYPE[entry] = 28;
        }
        
        if vars::SPIRIT_TYPE[entry]>=0 {
            MotionModule::set_rate(fighter.module_accessor,0.75);
        }
    }
    WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_CURRY_SUSPEND);
    return false.into();
}
#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn fighter_pickup_light_end(fighter: &mut L2CFighterCommon) -> L2CValue {

    let entry = get_entry_from_boma(fighter.module_accessor) as usize;
    vars::SPIRIT_TYPE[entry] = -1;
    return false.into();
}
*/
#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_DEAD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn fighter_dead(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry = get_entry_from_boma(fighter.module_accessor) as usize;
    let levelLoss = vars::LEVEL_MAX/3;
    vars::SPIRIT_TYPE[entry] = -1;

    vars::LEVEL_ATTACK[entry] = 0.max(vars::LEVEL_ATTACK[entry]-levelLoss);
    vars::LEVEL_SPEED[entry] = 0.max(vars::LEVEL_SPEED[entry]-levelLoss);
    vars::LEVEL_DEF[entry] = 0.max(vars::LEVEL_DEF[entry]-levelLoss);
    vars::LEVEL_JUMP[entry] = 0.max(vars::LEVEL_JUMP[entry]-levelLoss);
    vars::LEVEL_TURBO[entry] = 0.max(vars::LEVEL_TURBO[entry]-levelLoss);

    return false.into();
}

pub fn install() {
    install_status_scripts!(
        //fighter_pickup_light,
        //fighter_pickup_light_end,
        fighter_dead
    );
}