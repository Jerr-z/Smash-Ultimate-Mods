use {
    smash::{
        lua2cpp::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        phx::*,
        lib::lua_const::*,
        lib::L2CValue
    },
    smash_script::*,
    smashline::*
};

static mut SPECIAL_AIR_HI_USED: [bool; 8] = [false; 8];

/*
Only modifying the aerial version because I would still like him to keep the grounded one the same.
iirc hi2 is the first slash, hi1 doesn't do anything.
*/
#[acmd_script( agent = "chrom", script = "game_specialairhi2", category = ACMD_GAME, low_priority )]
unsafe fn chrom_specialairhi2(agent: &mut L2CAgentBase) {
    // replaced everything below with lucina's hitboxes
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        SPECIAL_AIR_HI_USED[entry_id] = true;
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 11.0, 361, 74, 0, 70, 4.0, 0.0, 8.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 11.0, 74, 74, 0, 70, 4.0, 0.0, 8.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 7.0, 361, 90, 0, 20, 5.0, 0.0, 0.0, 4.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 7.0, 74, 90, 0, 20, 5.0, 0.0, 0.0, -1.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 2, false);
        AttackModule::clear(agent.module_accessor, 3, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "chrom", script = "effect_specialairhi1", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialairhi1(agent: &mut L2CAgentBase) {
    
}



#[acmd_script( agent = "chrom", script = "effect_specialairhi2", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialairhi2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_dolphin_swing"), Hash40::new("top"), 0, 12, -1, 14, -30, 37, 1, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 6, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_dolphin_jump"), Hash40::new("top"), -0.0, 0, -5, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("lucina_dolphin_jump"), -1);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("lucina_dolphin_swing"), -1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_dolphin_shadow"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucina_sword_blue"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, -10, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 6);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucina_sword_blue"), false, true);
    }
}



/* decompiled code for status script, cleanup later */

// main
#[status_script(agent = "chrom", status = FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn chrom_specialhi2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_ROY_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_MUL_2);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_CONTINUE_MOT);
    fighter.sub_shift_status_main(L2CValue::Ptr(SpecialHi2_main_loop as *const () as _));
    return 0.into();
}

// main loop
unsafe fn SpecialHi2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }

    if !StatusModule::is_changing(fighter.module_accessor) {
        if fighter.global_table[0x17].get_i32() == *SITUATION_KIND_GROUND {
            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
                return LAB_71000209d8(fighter); // prev ground curr air
            }
        }
    
        if fighter.global_table[0x17].get_i32() == *SITUATION_KIND_GROUND {
            return LAB_7100021040(fighter); // prev is ground curr not air
        }

        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            return LAB_71000209d8(fighter); // current is ground
        }
    } else {
        return LAB_71000209d8(fighter);
    }


    return LAB_7100021040(fighter);
}

unsafe fn LAB_71000209d8(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {

        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP) {         
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_CONTINUE_MOT) {
                // MotionModule::change_motion(
                //     fighter.module_accessor, 
                //     Hash40::new_raw(0x105739d665), // special air hi 2
                //     MotionModule::frame(fighter.module_accessor),
                //     1.0,
                //     false,
                //     0.0,
                //     false,
                //     false
                // ); this one might not be it
    
                MotionModule::change_motion_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new_raw(0x105739d665),
                    -1.0,
                    1.0,
                    0.0,
                    false,
                    false
                );
    
                // goto LAB_7100020e50 you essentially call
                return LAB_7100021040(fighter);
            }
            
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(0x105739d665), // special air hi 2
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_CONTINUE_MOT);
            return LAB_7100021040(fighter);
        }
    } else {
        
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_CONTINUE_MOT) {
                MotionModule::change_motion_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new_raw(0xcef73e16f), // special hi 2
                    -1.0,
                    1.0,
                    0.0,
                    false,
                    false
                );
        // LAB_7100020e50:
            } else {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new_raw(0xcef73e16f), // special hi 2
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_CONTINUE_MOT);
            }
        //LAB_7100021030:
        }
    }
    return LAB_7100021040(fighter);
}

unsafe fn LAB_7100021040(fighter: &mut L2CFighterCommon) -> L2CValue {

    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE_CHROM) {
        
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP) {
            
            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_JUMP_START_AIR);
            }

            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.set_situation(SITUATION_KIND_AIR.into());
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE_CHROM);
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        // ADDTION HERE, force change into free fall if aerial up b is used
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
        if  SPECIAL_AIR_HI_USED[entry_id] == true {
            SPECIAL_AIR_HI_USED[entry_id] = false;
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }

        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_3.into(), false.into());
            // goto LAB_71000214ec;
            return 0.into();
        }
        
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_4.into(), false.into());
            return 0.into();
        }
    }

    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
    
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP) {
            
            if MotionModule::trans_move_speed(fighter.module_accessor).y < 0.0 {
                fighter.change_status(FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_4.into(), false.into());
                return 0.into();
            }
        }
    }
    return 0.into();
}


/*-----------------------------------------------*/
pub fn install() {
    install_acmd_scripts!(
        chrom_specialairhi2,
        effect_specialairhi1,
        effect_specialairhi2
    );

    install_status_scripts!(chrom_specialhi2_main);
}