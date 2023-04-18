use super::*;


unsafe fn apply_buffs(fighter: &mut L2CFighterCommon)
{
    let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
    let control_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyController>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL));
    let grav_energy = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyGravity>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY));

    let entry = get_entry_from_boma(fighter.module_accessor) as usize;
    let status = StatusModule::status_kind(fighter.module_accessor);
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    let lvl_attack = LEVEL_ATTACK[entry] as f32;
    let lvl_speed = LEVEL_SPEED[entry] as f32;
    let lvl_def = LEVEL_DEF[entry] as f32;
    let lvl_jump = LEVEL_JUMP[entry] as f32;
    let lvl_turbo = LEVEL_TURBO[entry] as f32;
    let lvl_max = vars::LEVEL_MAX as f32;
    if lvl_attack > 1.0
    {            
        if (status != *FIGHTER_STATUS_KIND_ATTACK)
        || (motion == Hash40::new("attack_13").hash)
        {
            AttackModule::set_power_up(fighter.module_accessor, 1.0+(lvl_attack/lvl_max));
            AttackModule::set_reaction_mul(fighter.module_accessor, 1.0+(lvl_attack/lvl_max));
        }
    }
    if lvl_def > 1.0
    {
        DamageModule::set_damage_mul(fighter.module_accessor, 1.0-(lvl_def/lvl_max));
        DamageModule::set_reaction_mul(fighter.module_accessor, 1.0-(lvl_def/lvl_max));
        DamageModule::set_force_damage_mul(fighter.module_accessor, 1.0-(lvl_def/lvl_max)*0.5);
        ShieldModule::set_attack_mul(fighter.module_accessor, 1.0-(lvl_def/lvl_max), *FIGHTER_SHIELD_KIND_GUARD);
        ShieldModule::set_hit_stop_mul(fighter.module_accessor, 1.0-(lvl_def/lvl_max));
    }
    if lvl_turbo > 1.0
    {
        if (status == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR)
        {
            let rate = 1.0+(lvl_turbo/(lvl_max));
            MotionModule::set_rate(fighter.module_accessor, rate);
        }
        else if ([
            *FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_ATTACK_DASH,
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4
        ].contains(&status))
        {
            let mut cancelFrame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,smash::phx::Hash40::new_raw(motion), false) as f32;
            let turboFactor = 1.0-((lvl_turbo/(lvl_max))*0.75);
            if (MotionModule::frame(fighter.module_accessor) >= cancelFrame*turboFactor)
            && cancelFrame > 0.0
            {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
        
    }
    if lvl_jump > 1.0
    {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_JUMP_AERIAL && MotionModule::frame(fighter.module_accessor) < 1.0 {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 2.0*(lvl_jump/lvl_max), z: 0.0} as *const Vector3f);
        }
        else if //StatusModule::prev_status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_JUMP_SQUAT && 
        StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_JUMP
        && MotionModule::frame(fighter.module_accessor) < 1.0 {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.5*(lvl_jump/lvl_max), z: 0.0} as *const Vector3f);
        }
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            let factor = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {1.0} else {1.0-(lvl_jump/lvl_max)*0.9};

            //smash::app::lua_bind::FighterKineticEnergyGravity::set_gravity_coefficient(grav_energy,factor);
            //smash::app::lua_bind::FighterKineticEnergyGravity::set_accel(grav_energy,factor);
            let lua_state = fighter.lua_state_agent;
            acmd!(lua_state, {
                sv_kinetic_energy::set_accel_y_mul(FIGHTER_KINETIC_ENERGY_ID_GRAVITY, factor)
                //sv_kinetic_energy::set_stable_speed(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1611000.0)
                //sv_kinetic_energy::friction_off()
            });
        }
    }
    if lvl_speed > 1.0
    {
        smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 1.0+(lvl_speed/lvl_max));
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            //smash::app::lua_bind::FighterKineticEnergyController::mul_x_speed_max(control_energy, 1.0+(lvl_jump/lvl_max)*0.5);
            let lua_state = fighter.lua_state_agent;
            acmd!(lua_state, {
                //sv_kinetic_energy::set_accel_x_add(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.0*(lvl_jump/lvl_max))
                sv_kinetic_energy::set_speed_mul(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.0+(lvl_jump/lvl_max))
                //sv_kinetic_energy::set_stable_speed(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1611000.0)
                //sv_kinetic_energy::friction_off()
            });
            //smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 1.0+(lvl_jump/lvl_max)*0.5);
        }
    }
}
unsafe fn add_buff(fighter: &mut L2CFighterCommon, entry: usize, mut buff_type: i32)
{
    if ItemModule::is_have_item(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN)
    {
        ItemModule::remove_item(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN);
    }
    if ItemModule::is_have_item(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_TEMPORARY)
    {
        ItemModule::remove_item(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_TEMPORARY);
    }

    let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let rot = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let size = 1.25;
    let time = 30;

    if buff_type == 100 {
        buff_type = app::sv_math::rand(hash40("fighter"), 37);
    }
    let is_All = buff_type == 101;
    let is_Attack = [0,1,2,3,4,5,6].contains(&buff_type) && vars::LEVEL_ATTACK[entry] < vars::LEVEL_MAX;
    let is_Def = [7,8,9,10,11,12,13].contains(&buff_type) && vars::LEVEL_DEF[entry] < vars::LEVEL_MAX;
    let is_Speed = [14,15,16,17,18,19,20].contains(&buff_type) && vars::LEVEL_SPEED[entry] < vars::LEVEL_MAX;
    let is_Jump = [21,22,23,24,25,26,27].contains(&buff_type) && vars::LEVEL_JUMP[entry] < vars::LEVEL_MAX;
    let is_Turbo = buff_type >= 28 && buff_type < 100 && vars::LEVEL_TURBO[entry] < vars::LEVEL_MAX;

    let mut eff = "sys_special_all_up";
    let mut eff_color = Vector3f{x: 1.0, y: 1.0, z: 1.0};
    //Attack
    if is_Attack || is_All
    {
        //sys_flies_up
        eff = "sys_special_attack_up";
        vars::LEVEL_ATTACK[entry] = vars::LEVEL_MAX.min(vars::LEVEL_ATTACK[entry]+1);
        println!("Food type: {}, Attack level: {}",buff_type,vars::LEVEL_ATTACK[entry]);
    
    }
    //Def
    if is_Def || is_All
    {
        eff_color.z=0.0;
        vars::LEVEL_DEF[entry] = vars::LEVEL_MAX.min(vars::LEVEL_DEF[entry]+1);
        println!("Food type: {}, Def level: {}",buff_type,vars::LEVEL_DEF[entry]);
    }
    //Speed
    if is_Speed || is_All
    {
        eff = "sys_special_defense_up";
        vars::LEVEL_SPEED[entry] = vars::LEVEL_MAX.min(vars::LEVEL_SPEED[entry]+1);
        println!("Food type: {}, Speed level: {}",buff_type,vars::LEVEL_SPEED[entry]);
    }
    //Jump
    if is_Jump || is_All
    {
        eff = "sys_special_speed_up";
        vars::LEVEL_JUMP[entry] = vars::LEVEL_MAX.min(vars::LEVEL_JUMP[entry]+1);
        println!("Food type: {}, Jump level: {}",buff_type,vars::LEVEL_JUMP[entry]);
    }
    //Turbo
    if is_Turbo || is_All
    {
        eff_color.y=0.0;
        vars::LEVEL_TURBO[entry] = vars::LEVEL_MAX.min(vars::LEVEL_TURBO[entry]+1);
        println!("Food type: {}, Turbo level: {}",buff_type,vars::LEVEL_TURBO[entry]);
    }
    if is_All
    {
        eff = "sys_special_all_up";
        eff_color = Vector3f{x: 1.0, y: 1.0, z: 1.0};
    }
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_special_attack_up"), false, true);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_special_defense_up"), false, true);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_special_speed_up"), false, true);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_special_all_up"), false, true);

    EffectModule::req_time_follow(fighter.module_accessor, Hash40::new(eff), Hash40::new("hip"), time, &pos, &rot, size, false, 0);
    LAST_EFFECT_SET_COLOR(fighter,eff_color.x,eff_color.y,eff_color.z);

    if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_GEKIKARA){
            
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_GEKIKARA);
        //prevent all curry vfx
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_curry_shot"), false, true);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_curry_fire"), false, true);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_curry_steam"), false, true);
        //prevent all curry sfx
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_item_curry_fire"), 0);
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_item_curry_fire_sp"), 0);
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_item_curry_shot"), 0);
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_item_curry_shot_b"), 0);
    }

}
unsafe fn check_for_buffs(fighter: &mut L2CFighterCommon)
{
    let entry = get_entry_from_boma(fighter.module_accessor) as usize;
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    if ((motion == Hash40::new("item_light_get").hash
    || motion == Hash40::new("item_light_eat").hash)
    ) && MotionModule::frame(fighter.module_accessor) < 5.0  {
       
    }

    if //vars::SPIRIT_TYPE[entry] == 0 &&
    (   
        ItemModule::is_have_item(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_TEMPORARY)
        || ItemModule::is_have_item(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN)
    ) && 
    MotionModule::frame(fighter.module_accessor) < 7.0       
    {
        let write_text = format!("Main: {} Temp {} Extra {} Special {} Heart {}",
        ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN),
        ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_TEMPORARY),
        ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_EXTRA),
        ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_SPECIAL),
         *ITEM_KIND_HEART);
        //std::fs::write("sd:/ultimate/mods/SmashRun/log.txt",write_text);
        //print(write_text); 
        println!("{}",write_text);

        let mut buff_type = 0;
        if ((ItemModule::get_pickable_item_kind(fighter.module_accessor) as i32) == *ITEM_KIND_TABEMONO)
        || ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN) == *ITEM_KIND_TABEMONO
        {
            let item_id = ItemModule::get_have_item_id(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN);
            let item = get_battle_object_from_id(item_id as u32);
            let variant = WorkModule::get_int((*item).module_accessor, *ITEM_INSTANCE_WORK_INT_VARIATION);
            buff_type = variant;
        }
        //It's about to get YandereDev in here
        else if ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN) == *ITEM_KIND_MAXIMTOMATO
        {
            buff_type = 100;
        }
        else if ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN) == *ITEM_KIND_HEART
        {
            buff_type = 101;
        }
        else if ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN) == *ITEM_KIND_CURRY
        {
            buff_type = 1;
        }
        else if ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN) == *ITEM_KIND_SMOKESCREEN
        {
            buff_type = 7;
        }
        else if ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN) == *ITEM_KIND_BANANAGUN
        {
            buff_type = 14;
        }
        else if ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN) == *ITEM_KIND_BEETLE
        {
            buff_type = 21;
        }
        else if ItemModule::get_have_item_kind(fighter.module_accessor,*FIGHTER_HAVE_ITEM_WORK_MAIN) == *ITEM_KIND_STEELDIVER
        {
            buff_type = 28;
        }
        
        if buff_type>0 {
            add_buff(fighter,entry,buff_type);
        }
    }
}


#[fighter_frame_callback]
fn fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        check_for_buffs(fighter);
        apply_buffs(fighter);
    }
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(
        fighter_frame
    );
}