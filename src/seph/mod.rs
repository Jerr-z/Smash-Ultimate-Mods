use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash_script::*;
use smash::lib::lua_const::*;
use smash::lib::{L2CValue,L2CAgent,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon,L2CFighterBase};
use smash::phx::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smashline::*;
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::libc::*;
use smash::params::*;
use skyline::logging::log;

static mut COUNTER: [i32; 8] = [0; 8];



unsafe fn entry_id(module_accessor: &mut BattleObjectModuleAccessor) -> usize { //get entry_id
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    return entry_id;
}

#[acmd_script(agent = "edge", script = "game_attacks3", category = ACMD_GAME )]
unsafe fn attack_s3s_hitbox(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(/*FSM*/ fighter, 0.3);

    frame(fighter.lua_state_agent, 11.0);
    macros::FT_MOTION_RATE(/*FSM*/ fighter, 1);
    
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::disable_tip(fighter.module_accessor);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 368, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(5.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 368, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 9.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(10.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        AttackModule::set_vec_target_pos(fighter.module_accessor,  0, Hash40::new("top"), &Vector2f{x: 15.0, y: 5.0} as *const Vector2f, 5 as u32, false);
        AttackModule::set_vec_target_pos(fighter.module_accessor,  1, Hash40::new("top"), &Vector2f{x:10.0, y:5.0} as *const Vector2f, 6 as u32, false);
    }
    
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        DamageModule::set_critical_hit(fighter.module_accessor, true);
        AttackModule::set_reaction_mul(fighter.module_accessor, 5.0);
        AttackModule::set_power_mul(fighter.module_accessor, 0.33);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 45.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 18.5, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(12.5), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 45.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 18.5, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(23.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 45.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 18.5, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(31.0), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 10.0, /*Unk*/ false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 10.0, /*Unk*/ false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 10.0, /*Unk*/ false);
    }

    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        DamageModule::set_critical_hit(fighter.module_accessor, false);
        AttackModule::set_reaction_mul(fighter.module_accessor, 1.0);
        AttackModule::set_power_mul(fighter.module_accessor, 1.0);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
        AttackModule::clear_all(fighter.module_accessor);
    }
    
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        SlowModule::clear_whole(fighter.module_accessor);
        macros::CAM_ZOOM_OUT(fighter);
    }

}

#[acmd_script(agent = "edge", script = "game_attacks3hi", category = ACMD_GAME )]
unsafe fn attack_s3hi_hitbox(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(/*FSM*/fighter, 0.3);

    frame(fighter.lua_state_agent, 11.0);
    macros::FT_MOTION_RATE(/*FSM*/ fighter, 1);
    
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::disable_tip(fighter.module_accessor);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 368, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(5.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 368, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 9.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(10.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
    
        AttackModule::set_vec_target_pos(fighter.module_accessor,  0, Hash40::new("top"), &Vector2f{x: 15.0, y: 5.0} as *const Vector2f, 5 as u32, false);
        AttackModule::set_vec_target_pos(fighter.module_accessor,  1, Hash40::new("top"), &Vector2f{x:10.0, y:5.0} as *const Vector2f, 6 as u32, false);
    }
    
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        DamageModule::set_critical_hit(fighter.module_accessor, true);
        AttackModule::set_reaction_mul(fighter.module_accessor, 5.0);
        AttackModule::set_power_mul(fighter.module_accessor, 0.33);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 45.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 18.5, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(12.5), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 45.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 18.5, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(23.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 45.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 18.5, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(31.0), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 10.0, /*Unk*/ false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 10.0, /*Unk*/ false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 10.0, /*Unk*/ false)        
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        DamageModule::set_critical_hit(fighter.module_accessor, false);
        AttackModule::set_reaction_mul(fighter.module_accessor, 1.0);
        AttackModule::set_power_mul(fighter.module_accessor, 1.0);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        SlowModule::clear_whole(fighter.module_accessor);
        macros::CAM_ZOOM_OUT(fighter);
    }
}

#[acmd_script(agent = "edge", script =  "game_attacks3lw", category = ACMD_GAME )]
unsafe fn attack_s3lw_hitbox(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(/*FSM*/fighter, 0.3);
    
    frame(fighter.lua_state_agent, 11.0);
    macros::FT_MOTION_RATE(/*FSM*/ fighter,1);
    
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::disable_tip(fighter.module_accessor);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 368, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(5.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 368, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 9.5, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(23.0), /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
    
        AttackModule::set_vec_target_pos(fighter.module_accessor,  0, Hash40::new("top"), &Vector2f{x: 15.0, y: 5.0} as *const Vector2f, 5 as u32, false);
        AttackModule::set_vec_target_pos(fighter.module_accessor,  1, Hash40::new("top"), &Vector2f{x:8.0, y:5.0} as *const Vector2f, 6 as u32, false);
    }
    
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        DamageModule::set_critical_hit(fighter.module_accessor, true);
        AttackModule::set_reaction_mul(fighter.module_accessor, 5.0);
        AttackModule::set_power_mul(fighter.module_accessor, 0.33);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 45.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 18.5, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(12.5), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 45.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 18.5, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(23.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 45.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 20, /*BKB*/ 0, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 18.5, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(31.0), /*Hitlag*/ 0.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 10.0, /*Unk*/ false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 10.0, /*Unk*/ false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 10.0, /*Unk*/ false);
    }

    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        DamageModule::set_critical_hit(fighter.module_accessor, false);
        AttackModule::set_reaction_mul(fighter.module_accessor, 1.0);
        AttackModule::set_power_mul(fighter.module_accessor, 1.0);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_finishhit"), false, false);
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        SlowModule::clear_whole(fighter.module_accessor);
        macros::CAM_ZOOM_OUT(fighter);
    }
}

#[acmd_script(agent = "edge", script = "effect_attacks3hi", category = ACMD_EFFECT )]
unsafe fn stab_effect(fighter: &mut L2CAgentBase) {
    // clear residual effects
}

#[acmd_script(agent = "edge", script = "effect_attacks3", category = ACMD_EFFECT )]
unsafe fn stab_effect2(fighter: &mut L2CAgentBase) {
    // clear residual effects
}
#[acmd_script(agent = "edge", script = "effect_attacks3lw", category = ACMD_EFFECT )]

unsafe fn stab_effect3(fighter: &mut L2CAgentBase) {
    // clear residual effects
}
#[inline(always)]
unsafe fn wingmode_effects(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
	EffectModule::req_screen(module_accessor, smash::phx::Hash40::new("edge_wing_screen"),false,false,false);
    WorkModule::on_flag(module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED);
    WorkModule::set_int(module_accessor, 1,*FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_WING_STATE);
    WorkModule::set_int(module_accessor, *FIGHTER_EDGE_ONE_WINGED_HAIR_ON, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_HAIR_STATE) ;    
    macros::EFFECT_FOLLOW(fighter, Hash40::new("edge_onewinged_feather"), Hash40::new("wingr03"), 0, 0, 0, 0, 0, 0, 1, false);
	macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("edge_onewinged_flare"), Hash40::new("wingr01"), 1, 0, -1, 0, 0, 0, 0.8, false, 1);
	macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("edge_onewinged_flare"), Hash40::new("wingr02"), 0, 0, 0, 0, 0, 0, 0.8, false, 1);
	macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("edge_onewinged_flare"), Hash40::new("wingr02"), 2.5, 0, -1, 0, 0, 0, 0.8, false, 1);
	macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("edge_onewinged_flare"), Hash40::new("wingr03"), 0, 0, -0.5, 0, 0, 0, 0.8, false, 0.8);
	macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("edge_onewinged_flare"), Hash40::new("wingr03"), 2.5, 0, 0, 0, 0, 0, 0.8, false, 0.6);
	macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("edge_onewinged_flare"), Hash40::new("wingr04"), 0, 0, 0, 0, 0, 0, 0.8, false, 0.4);
	macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("edge_onewinged_flare"), Hash40::new("wingr04"), 2.5, 0, 0.5, 0, 0, 0, 0.8, false, 0.25);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("edge_wing"),true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("edge_wing1"),true);
    ModelModule::set_mesh_visibility(module_accessor, Hash40::new("edge_wing2"),true);
}


#[smashline::fighter_frame(agent = FIGHTER_KIND_EDGE)]
fn criticalhiteffect(fighter: &mut L2CFighterCommon){
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S3 {
            if AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT) == true {
                if COUNTER[entry_id(module_accessor)] < 2 && MotionModule::frame(module_accessor)> 12.0 {
                    SlowModule::set_whole(module_accessor,4,0);
                    macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
                    macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/5.0, 0.0, /*zoom*/2.0, /*yrot*/0.0, /*xrot*/0.0);
                    wingmode_effects(fighter);                                              
                }   
            }   
        }
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_EDGE)]
fn wingmechs(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        
        if WorkModule::get_int(module_accessor,*FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_WING_STATE) != 1 {
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("edge_wing3"),false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("edge_wing4"),false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("edge_wing"),false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("edge_wing1"),false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("edge_wing2"),false);
            WorkModule::set_int(module_accessor, 2, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
            WorkModule::on_flag(module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_CANNOT_JUMP_PLUS);
            WorkModule::off_flag(module_accessor,*FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_JUMP_PLUS_ON);
        }

        if WorkModule::get_int(module_accessor,*FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_WING_STATE) == 1{
                WorkModule::off_flag(module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_CANNOT_JUMP_PLUS);
                WorkModule::on_flag(module_accessor,*FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_JUMP_PLUS_ON);
                WorkModule::set_int(module_accessor, 3, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
                WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
                WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
        }
    }
}

pub fn install() {  
   install_acmd_scripts!(attack_s3s_hitbox, stab_effect, stab_effect2, stab_effect3, attack_s3hi_hitbox, attack_s3lw_hitbox);  
   install_agent_frames!(criticalhiteffect, wingmechs);
}