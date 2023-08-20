use {
    smash::{
        lua2cpp::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        phx::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

// Use this for general per-frame fighter-level hooks
#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        if fighter_kind == *FIGHTER_KIND_LUCINA {
            let stickx = ControlModule::get_stick_x(module_accessor);
            let sticky = ControlModule::get_stick_y(module_accessor);
            let button_atk = ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_ATTACK);
            let button_special = ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL);
            let button_jump = ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP);
            let button_shield = ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_GUARD);
            let button_grab = ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_CATCH);
            let shorthop = ControlModule::is_jump_mini_button(module_accessor);
            let posx = PostureModule::pos_x(module_accessor);
            let posy = PostureModule::pos_y(module_accessor);
            let cliffpos = GroundModule::hang_cliff_pos_3f(module_accessor); 
            let status = StatusModule::status_kind(module_accessor);
            let attack = AttackModule::is_attack(module_accessor, 0, false);
            let hit = AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT);
            let situation = StatusModule::situation_kind(module_accessor);
            let input = ControlModule::get_pad_flag(module_accessor);
            let percent = DamageModule::damage(module_accessor,0);
            let cancellable = CancelModule::is_enable_cancel(module_accessor);
            let fastfall = WorkModule::is_flag(module_accessor,*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
            
            println!(
                "stick_x:{}, 
                stick_y:{}, 
                A_button:{}, 
                B_button:{}, 
                jump_button:{}, 
                shield_button:{},
                grab_button:{}, 
                is_shorthop:{}, 
                pos_x:{}, 
                pos_y:{},
                cliff_x:{},
                cliff_y{},
                status:{},
                attacking:{},
                is_hit:{},
                situation:{},
                input:{},
                percent:{},
                actionable:{},
                fastfall:{},
                entry_id:{}",
                
                stickx, 
                sticky, 
                button_atk, 
                button_special, 
                button_jump, 
                button_shield, 
                button_grab, 
                shorthop, 
                posx, 
                posy, 
                cliffpos.x,
                cliffpos.y, 
                status, 
                attack, 
                hit, 
                situation, 
                input, 
                percent, 
                cancellable, 
                fastfall,
                entry_id
            );
        }

    }
}

// Use this for general per-frame weapon-level hooks
#[weapon_frame_callback]
pub fn global_weapon_frame(fighter_base : &mut L2CFighterBase) {
    unsafe {
        let module_accessor = sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
        let frame = MotionModule::frame(module_accessor) as i32;

        if frame % 10 == 0 {
            println!("[Weapon Hook] Frame : {}", frame);
        }
    }
}


#[fighter_frame(agent = FIGHTER_KIND_LUCINA)]
fn lucina_frame(fighter: &mut L2CFighterCommon){
    unsafe{
        let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        let stickx = ControlModule::get_stick_x(module_accessor);
        let sticky = ControlModule::get_stick_y(module_accessor);
        let button_atk = ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_ATTACK);
        let button_special = ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL);
        let button_jump = ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP);
        let button_shield = ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_GUARD);
        let button_grab = ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_CATCH);
        let shorthop = ControlModule::is_jump_mini_button(module_accessor);
        let posx = PostureModule::pos_x(module_accessor);
        let posy = PostureModule::pos_y(module_accessor);
        let cliffpos = GroundModule::hang_cliff_pos_3f(module_accessor); 
        let status = StatusModule::status_kind(module_accessor);
        let attack = AttackModule::is_attack(module_accessor, 0, false);
        let hit = AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT);
        let situation = StatusModule::situation_kind(module_accessor);
        let input = ControlModule::get_pad_flag(module_accessor);
        let percent = DamageModule::damage(module_accessor,0);
        let cancellable = CancelModule::is_enable_cancel(module_accessor);
        let fastfall = WorkModule::is_flag(module_accessor,*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        println!("stick_x:{}, stick_y:{}, A_button:{}, B_button:{}, jump_button:{}, shield_button:{},
        grab_button:{}, is_shorthop:{}, pos_x:{}, pos_y:{},cliff_x:{},cliff_y{},status:{},attacking:{},
        is_hit:{},situation:{},input:{},percent:{},actionable:{},fastfall:{}",
        stickx, sticky, button_atk, button_special, button_jump, button_shield, button_grab, shorthop, posx, posy, cliffpos.x,
        cliffpos.y, status, attack, hit, situation, input, percent, cancellable, fastfall);
        
        
    }
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame,
        global_weapon_frame
    );
    smashline::install_agent_frames!(
        lucina_frame
    );
}

#[smashline::installer]
pub fn installer() {
    smashline::install_agent_frame_callbacks!(global_fighter_frame);
}