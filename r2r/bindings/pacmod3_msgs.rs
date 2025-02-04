  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct AccelAuxRpt {

                              pub header: std_msgs::msg::Header,
pub operator_interaction: bool,
pub accel_limiting_active: bool,
pub park_brake_interlock_active: bool,
pub brake_interlock_active: bool,
pub operator_interaction_avail: bool,
pub accel_limiting_active_avail: bool,
pub park_brake_interlock_active_avail: bool,
pub brake_interlock_active_avail: bool,

                          }

                          impl WrappedTypesupport for AccelAuxRpt { 

            type CStruct = pacmod3_msgs__msg__AccelAuxRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__AccelAuxRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__AccelAuxRpt {

                unsafe { pacmod3_msgs__msg__AccelAuxRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__AccelAuxRpt) -> () {

                unsafe { pacmod3_msgs__msg__AccelAuxRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> AccelAuxRpt {
  AccelAuxRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
operator_interaction: msg.operator_interaction,
accel_limiting_active: msg.accel_limiting_active,
park_brake_interlock_active: msg.park_brake_interlock_active,
brake_interlock_active: msg.brake_interlock_active,
operator_interaction_avail: msg.operator_interaction_avail,
accel_limiting_active_avail: msg.accel_limiting_active_avail,
park_brake_interlock_active_avail: msg.park_brake_interlock_active_avail,
brake_interlock_active_avail: msg.brake_interlock_active_avail,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.operator_interaction = self.operator_interaction;
msg.accel_limiting_active = self.accel_limiting_active;
msg.park_brake_interlock_active = self.park_brake_interlock_active;
msg.brake_interlock_active = self.brake_interlock_active;
msg.operator_interaction_avail = self.operator_interaction_avail;
msg.accel_limiting_active_avail = self.accel_limiting_active_avail;
msg.park_brake_interlock_active_avail = self.park_brake_interlock_active_avail;
msg.brake_interlock_active_avail = self.brake_interlock_active_avail;
}



        }


                          
                          impl Default for AccelAuxRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<AccelAuxRpt>::new();
                                  AccelAuxRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct AllSystemStatuses {

                              pub header: std_msgs::msg::Header,
pub enabled_status: Vec<pacmod3_msgs::msg::KeyValuePair>,
pub overridden_status: Vec<pacmod3_msgs::msg::KeyValuePair>,
pub fault_status: Vec<pacmod3_msgs::msg::KeyValuePair>,

                          }

                          impl WrappedTypesupport for AllSystemStatuses { 

            type CStruct = pacmod3_msgs__msg__AllSystemStatuses; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__AllSystemStatuses() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__AllSystemStatuses {

                unsafe { pacmod3_msgs__msg__AllSystemStatuses__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__AllSystemStatuses) -> () {

                unsafe { pacmod3_msgs__msg__AllSystemStatuses__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> AllSystemStatuses {
  AllSystemStatuses {
header: std_msgs::msg::Header::from_native(&msg.header),
// is_upper_bound_: false
// member.array_size_ : 0
enabled_status : {
let mut temp = Vec::with_capacity(msg.enabled_status.size);
let slice = unsafe { std::slice::from_raw_parts(msg.enabled_status.data, msg.enabled_status.size)};
for s in slice { temp.push(pacmod3_msgs::msg::KeyValuePair::from_native(s)); }
temp },
// is_upper_bound_: false
// member.array_size_ : 0
overridden_status : {
let mut temp = Vec::with_capacity(msg.overridden_status.size);
let slice = unsafe { std::slice::from_raw_parts(msg.overridden_status.data, msg.overridden_status.size)};
for s in slice { temp.push(pacmod3_msgs::msg::KeyValuePair::from_native(s)); }
temp },
// is_upper_bound_: false
// member.array_size_ : 0
fault_status : {
let mut temp = Vec::with_capacity(msg.fault_status.size);
let slice = unsafe { std::slice::from_raw_parts(msg.fault_status.data, msg.fault_status.size)};
for s in slice { temp.push(pacmod3_msgs::msg::KeyValuePair::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
unsafe { pacmod3_msgs__msg__KeyValuePair__Sequence__fini(&mut msg.enabled_status) };
unsafe { pacmod3_msgs__msg__KeyValuePair__Sequence__init(&mut msg.enabled_status, self.enabled_status.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.enabled_status.data, msg.enabled_status.size)};
for (t,s) in slice.iter_mut().zip(&self.enabled_status) { s.copy_to_native(t);}
unsafe { pacmod3_msgs__msg__KeyValuePair__Sequence__fini(&mut msg.overridden_status) };
unsafe { pacmod3_msgs__msg__KeyValuePair__Sequence__init(&mut msg.overridden_status, self.overridden_status.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.overridden_status.data, msg.overridden_status.size)};
for (t,s) in slice.iter_mut().zip(&self.overridden_status) { s.copy_to_native(t);}
unsafe { pacmod3_msgs__msg__KeyValuePair__Sequence__fini(&mut msg.fault_status) };
unsafe { pacmod3_msgs__msg__KeyValuePair__Sequence__init(&mut msg.fault_status, self.fault_status.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.fault_status.data, msg.fault_status.size)};
for (t,s) in slice.iter_mut().zip(&self.fault_status) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for AllSystemStatuses {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<AllSystemStatuses>::new();
                                  AllSystemStatuses::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct AngVelRpt {

                              pub header: std_msgs::msg::Header,
pub pitch_new_data_rx: bool,
pub roll_new_data_rx: bool,
pub yaw_new_data_rx: bool,
pub pitch_valid: bool,
pub roll_valid: bool,
pub yaw_valid: bool,
pub pitch_vel: f64,
pub roll_vel: f64,
pub yaw_vel: f64,

                          }

                          impl WrappedTypesupport for AngVelRpt { 

            type CStruct = pacmod3_msgs__msg__AngVelRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__AngVelRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__AngVelRpt {

                unsafe { pacmod3_msgs__msg__AngVelRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__AngVelRpt) -> () {

                unsafe { pacmod3_msgs__msg__AngVelRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> AngVelRpt {
  AngVelRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
pitch_new_data_rx: msg.pitch_new_data_rx,
roll_new_data_rx: msg.roll_new_data_rx,
yaw_new_data_rx: msg.yaw_new_data_rx,
pitch_valid: msg.pitch_valid,
roll_valid: msg.roll_valid,
yaw_valid: msg.yaw_valid,
pitch_vel: msg.pitch_vel,
roll_vel: msg.roll_vel,
yaw_vel: msg.yaw_vel,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.pitch_new_data_rx = self.pitch_new_data_rx;
msg.roll_new_data_rx = self.roll_new_data_rx;
msg.yaw_new_data_rx = self.yaw_new_data_rx;
msg.pitch_valid = self.pitch_valid;
msg.roll_valid = self.roll_valid;
msg.yaw_valid = self.yaw_valid;
msg.pitch_vel = self.pitch_vel;
msg.roll_vel = self.roll_vel;
msg.yaw_vel = self.yaw_vel;
}



        }


                          
                          impl Default for AngVelRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<AngVelRpt>::new();
                                  AngVelRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct BrakeAuxRpt {

                              pub header: std_msgs::msg::Header,
pub brake_pressure: f64,
pub operator_interaction: bool,
pub brake_on_off: bool,
pub brake_limiting_active: bool,
pub brake_reduced_assist: bool,
pub brake_pressure_avail: bool,
pub operator_interaction_avail: bool,
pub brake_on_off_avail: bool,
pub brake_limiting_active_avail: bool,
pub brake_reduced_assist_avail: bool,

                          }

                          impl WrappedTypesupport for BrakeAuxRpt { 

            type CStruct = pacmod3_msgs__msg__BrakeAuxRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__BrakeAuxRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__BrakeAuxRpt {

                unsafe { pacmod3_msgs__msg__BrakeAuxRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__BrakeAuxRpt) -> () {

                unsafe { pacmod3_msgs__msg__BrakeAuxRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> BrakeAuxRpt {
  BrakeAuxRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
brake_pressure: msg.brake_pressure,
operator_interaction: msg.operator_interaction,
brake_on_off: msg.brake_on_off,
brake_limiting_active: msg.brake_limiting_active,
brake_reduced_assist: msg.brake_reduced_assist,
brake_pressure_avail: msg.brake_pressure_avail,
operator_interaction_avail: msg.operator_interaction_avail,
brake_on_off_avail: msg.brake_on_off_avail,
brake_limiting_active_avail: msg.brake_limiting_active_avail,
brake_reduced_assist_avail: msg.brake_reduced_assist_avail,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.brake_pressure = self.brake_pressure;
msg.operator_interaction = self.operator_interaction;
msg.brake_on_off = self.brake_on_off;
msg.brake_limiting_active = self.brake_limiting_active;
msg.brake_reduced_assist = self.brake_reduced_assist;
msg.brake_pressure_avail = self.brake_pressure_avail;
msg.operator_interaction_avail = self.operator_interaction_avail;
msg.brake_on_off_avail = self.brake_on_off_avail;
msg.brake_limiting_active_avail = self.brake_limiting_active_avail;
msg.brake_reduced_assist_avail = self.brake_reduced_assist_avail;
}



        }


                          
                          impl Default for BrakeAuxRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<BrakeAuxRpt>::new();
                                  BrakeAuxRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct BrakeDecelAuxRpt {

                              pub header: std_msgs::msg::Header,
pub xbr_active_control_mode: u16,
pub xbr_system_state: u16,
pub foundation_brake_use: u16,
pub hill_holder_mode: u16,
pub xbr_active_control_mode_avail: bool,
pub xbr_system_state_avail: bool,
pub foundation_brake_use_avail: bool,
pub hill_holder_mode_avail: bool,

                          }

                          impl WrappedTypesupport for BrakeDecelAuxRpt { 

            type CStruct = pacmod3_msgs__msg__BrakeDecelAuxRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__BrakeDecelAuxRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__BrakeDecelAuxRpt {

                unsafe { pacmod3_msgs__msg__BrakeDecelAuxRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__BrakeDecelAuxRpt) -> () {

                unsafe { pacmod3_msgs__msg__BrakeDecelAuxRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> BrakeDecelAuxRpt {
  BrakeDecelAuxRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
xbr_active_control_mode: msg.xbr_active_control_mode,
xbr_system_state: msg.xbr_system_state,
foundation_brake_use: msg.foundation_brake_use,
hill_holder_mode: msg.hill_holder_mode,
xbr_active_control_mode_avail: msg.xbr_active_control_mode_avail,
xbr_system_state_avail: msg.xbr_system_state_avail,
foundation_brake_use_avail: msg.foundation_brake_use_avail,
hill_holder_mode_avail: msg.hill_holder_mode_avail,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.xbr_active_control_mode = self.xbr_active_control_mode;
msg.xbr_system_state = self.xbr_system_state;
msg.foundation_brake_use = self.foundation_brake_use;
msg.hill_holder_mode = self.hill_holder_mode;
msg.xbr_active_control_mode_avail = self.xbr_active_control_mode_avail;
msg.xbr_system_state_avail = self.xbr_system_state_avail;
msg.foundation_brake_use_avail = self.foundation_brake_use_avail;
msg.hill_holder_mode_avail = self.hill_holder_mode_avail;
}



        }


                          
                          impl Default for BrakeDecelAuxRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<BrakeDecelAuxRpt>::new();
                                  BrakeDecelAuxRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct BrakeDecelCmd {

                              pub header: std_msgs::msg::Header,
pub enable: bool,
pub ignore_overrides: bool,
pub clear_override: bool,
pub brake_deccel_command: f64,
pub xbr_ebi_mode: u16,
pub xbr_priority: u16,
pub xbr_control_mode: u16,

                          }

                          impl WrappedTypesupport for BrakeDecelCmd { 

            type CStruct = pacmod3_msgs__msg__BrakeDecelCmd; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__BrakeDecelCmd() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__BrakeDecelCmd {

                unsafe { pacmod3_msgs__msg__BrakeDecelCmd__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__BrakeDecelCmd) -> () {

                unsafe { pacmod3_msgs__msg__BrakeDecelCmd__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> BrakeDecelCmd {
  BrakeDecelCmd {
header: std_msgs::msg::Header::from_native(&msg.header),
enable: msg.enable,
ignore_overrides: msg.ignore_overrides,
clear_override: msg.clear_override,
brake_deccel_command: msg.brake_deccel_command,
xbr_ebi_mode: msg.xbr_ebi_mode,
xbr_priority: msg.xbr_priority,
xbr_control_mode: msg.xbr_control_mode,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.enable = self.enable;
msg.ignore_overrides = self.ignore_overrides;
msg.clear_override = self.clear_override;
msg.brake_deccel_command = self.brake_deccel_command;
msg.xbr_ebi_mode = self.xbr_ebi_mode;
msg.xbr_priority = self.xbr_priority;
msg.xbr_control_mode = self.xbr_control_mode;
}



        }


                          
                          impl Default for BrakeDecelCmd {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<BrakeDecelCmd>::new();
                                  BrakeDecelCmd::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct CabinClimateCmd {

                              pub header: std_msgs::msg::Header,
pub enable: bool,
pub ignore_overrides: bool,
pub clear_override: bool,
pub cmd_ac_off_on: u16,
pub cmd_max_ac_off_on: u16,
pub cmd_defrost_off_on: u16,
pub cmd_max_defrost_off_on: u16,
pub cmd_dir_up_off_on: u16,
pub cmd_dir_down_off_on: u16,

                          }

                          impl WrappedTypesupport for CabinClimateCmd { 

            type CStruct = pacmod3_msgs__msg__CabinClimateCmd; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__CabinClimateCmd() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__CabinClimateCmd {

                unsafe { pacmod3_msgs__msg__CabinClimateCmd__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__CabinClimateCmd) -> () {

                unsafe { pacmod3_msgs__msg__CabinClimateCmd__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> CabinClimateCmd {
  CabinClimateCmd {
header: std_msgs::msg::Header::from_native(&msg.header),
enable: msg.enable,
ignore_overrides: msg.ignore_overrides,
clear_override: msg.clear_override,
cmd_ac_off_on: msg.cmd_ac_off_on,
cmd_max_ac_off_on: msg.cmd_max_ac_off_on,
cmd_defrost_off_on: msg.cmd_defrost_off_on,
cmd_max_defrost_off_on: msg.cmd_max_defrost_off_on,
cmd_dir_up_off_on: msg.cmd_dir_up_off_on,
cmd_dir_down_off_on: msg.cmd_dir_down_off_on,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.enable = self.enable;
msg.ignore_overrides = self.ignore_overrides;
msg.clear_override = self.clear_override;
msg.cmd_ac_off_on = self.cmd_ac_off_on;
msg.cmd_max_ac_off_on = self.cmd_max_ac_off_on;
msg.cmd_defrost_off_on = self.cmd_defrost_off_on;
msg.cmd_max_defrost_off_on = self.cmd_max_defrost_off_on;
msg.cmd_dir_up_off_on = self.cmd_dir_up_off_on;
msg.cmd_dir_down_off_on = self.cmd_dir_down_off_on;
}



        }


                          
                          impl Default for CabinClimateCmd {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<CabinClimateCmd>::new();
                                  CabinClimateCmd::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct CabinClimateRpt {

                              pub header: std_msgs::msg::Header,
pub enabled: bool,
pub override_active: bool,
pub command_output_fault: bool,
pub input_output_fault: bool,
pub output_reported_fault: bool,
pub pacmod_fault: bool,
pub vehicle_fault: bool,
pub command_timeout: bool,
pub man_ac_off_on: u16,
pub man_max_ac_off_on: u16,
pub man_defrost_off_on: u16,
pub man_max_defrost_off_on: u16,
pub man_dir_up_off_on: u16,
pub man_dir_down_off_on: u16,
pub cmd_ac_off_on: u16,
pub cmd_max_ac_off_on: u16,
pub cmd_defrost_off_on: u16,
pub cmd_max_defrost_off_on: u16,
pub cmd_dir_up_off_on: u16,
pub cmd_dir_down_off_on: u16,
pub out_ac_off_on: u16,
pub out_max_ac_off_on: u16,
pub out_defrost_off_on: u16,
pub out_max_defrost_off_on: u16,
pub out_dir_up_off_on: u16,
pub out_dir_down_off_on: u16,

                          }

                          impl WrappedTypesupport for CabinClimateRpt { 

            type CStruct = pacmod3_msgs__msg__CabinClimateRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__CabinClimateRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__CabinClimateRpt {

                unsafe { pacmod3_msgs__msg__CabinClimateRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__CabinClimateRpt) -> () {

                unsafe { pacmod3_msgs__msg__CabinClimateRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> CabinClimateRpt {
  CabinClimateRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
enabled: msg.enabled,
override_active: msg.override_active,
command_output_fault: msg.command_output_fault,
input_output_fault: msg.input_output_fault,
output_reported_fault: msg.output_reported_fault,
pacmod_fault: msg.pacmod_fault,
vehicle_fault: msg.vehicle_fault,
command_timeout: msg.command_timeout,
man_ac_off_on: msg.man_ac_off_on,
man_max_ac_off_on: msg.man_max_ac_off_on,
man_defrost_off_on: msg.man_defrost_off_on,
man_max_defrost_off_on: msg.man_max_defrost_off_on,
man_dir_up_off_on: msg.man_dir_up_off_on,
man_dir_down_off_on: msg.man_dir_down_off_on,
cmd_ac_off_on: msg.cmd_ac_off_on,
cmd_max_ac_off_on: msg.cmd_max_ac_off_on,
cmd_defrost_off_on: msg.cmd_defrost_off_on,
cmd_max_defrost_off_on: msg.cmd_max_defrost_off_on,
cmd_dir_up_off_on: msg.cmd_dir_up_off_on,
cmd_dir_down_off_on: msg.cmd_dir_down_off_on,
out_ac_off_on: msg.out_ac_off_on,
out_max_ac_off_on: msg.out_max_ac_off_on,
out_defrost_off_on: msg.out_defrost_off_on,
out_max_defrost_off_on: msg.out_max_defrost_off_on,
out_dir_up_off_on: msg.out_dir_up_off_on,
out_dir_down_off_on: msg.out_dir_down_off_on,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.enabled = self.enabled;
msg.override_active = self.override_active;
msg.command_output_fault = self.command_output_fault;
msg.input_output_fault = self.input_output_fault;
msg.output_reported_fault = self.output_reported_fault;
msg.pacmod_fault = self.pacmod_fault;
msg.vehicle_fault = self.vehicle_fault;
msg.command_timeout = self.command_timeout;
msg.man_ac_off_on = self.man_ac_off_on;
msg.man_max_ac_off_on = self.man_max_ac_off_on;
msg.man_defrost_off_on = self.man_defrost_off_on;
msg.man_max_defrost_off_on = self.man_max_defrost_off_on;
msg.man_dir_up_off_on = self.man_dir_up_off_on;
msg.man_dir_down_off_on = self.man_dir_down_off_on;
msg.cmd_ac_off_on = self.cmd_ac_off_on;
msg.cmd_max_ac_off_on = self.cmd_max_ac_off_on;
msg.cmd_defrost_off_on = self.cmd_defrost_off_on;
msg.cmd_max_defrost_off_on = self.cmd_max_defrost_off_on;
msg.cmd_dir_up_off_on = self.cmd_dir_up_off_on;
msg.cmd_dir_down_off_on = self.cmd_dir_down_off_on;
msg.out_ac_off_on = self.out_ac_off_on;
msg.out_max_ac_off_on = self.out_max_ac_off_on;
msg.out_defrost_off_on = self.out_defrost_off_on;
msg.out_max_defrost_off_on = self.out_max_defrost_off_on;
msg.out_dir_up_off_on = self.out_dir_up_off_on;
msg.out_dir_down_off_on = self.out_dir_down_off_on;
}



        }


                          
                          impl Default for CabinClimateRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<CabinClimateRpt>::new();
                                  CabinClimateRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct ComponentRpt {

                              pub header: std_msgs::msg::Header,
pub component_type: u8,
pub accel: bool,
pub brake: bool,
pub cruise_control_buttons: bool,
pub dash_controls_left: bool,
pub dash_controls_right: bool,
pub hazard_lights: bool,
pub headlight: bool,
pub horn: bool,
pub media_controls: bool,
pub parking_brake: bool,
pub shift: bool,
pub sprayer: bool,
pub steering: bool,
pub turn: bool,
pub wiper: bool,
pub watchdog: bool,
pub brake_deccel: bool,
pub rear_pass_door: bool,
pub engine_brake: bool,
pub marker_lamp: bool,
pub cabin_climate: bool,
pub cabin_fan_speed: bool,
pub cabin_temp: bool,
pub counter: u8,
pub complement: u8,
pub config_fault: bool,
pub can_timeout_fault: bool,
pub internal_supply_voltage_fault: bool,
pub supervisory_timeout: bool,
pub supervisory_sanity_fault: bool,
pub watchdog_sanity_fault: bool,
pub watchdog_system_present_fault: bool,

                          }

                          impl WrappedTypesupport for ComponentRpt { 

            type CStruct = pacmod3_msgs__msg__ComponentRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__ComponentRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__ComponentRpt {

                unsafe { pacmod3_msgs__msg__ComponentRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__ComponentRpt) -> () {

                unsafe { pacmod3_msgs__msg__ComponentRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> ComponentRpt {
  ComponentRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
component_type: msg.component_type,
accel: msg.accel,
brake: msg.brake,
cruise_control_buttons: msg.cruise_control_buttons,
dash_controls_left: msg.dash_controls_left,
dash_controls_right: msg.dash_controls_right,
hazard_lights: msg.hazard_lights,
headlight: msg.headlight,
horn: msg.horn,
media_controls: msg.media_controls,
parking_brake: msg.parking_brake,
shift: msg.shift,
sprayer: msg.sprayer,
steering: msg.steering,
turn: msg.turn,
wiper: msg.wiper,
watchdog: msg.watchdog,
brake_deccel: msg.brake_deccel,
rear_pass_door: msg.rear_pass_door,
engine_brake: msg.engine_brake,
marker_lamp: msg.marker_lamp,
cabin_climate: msg.cabin_climate,
cabin_fan_speed: msg.cabin_fan_speed,
cabin_temp: msg.cabin_temp,
counter: msg.counter,
complement: msg.complement,
config_fault: msg.config_fault,
can_timeout_fault: msg.can_timeout_fault,
internal_supply_voltage_fault: msg.internal_supply_voltage_fault,
supervisory_timeout: msg.supervisory_timeout,
supervisory_sanity_fault: msg.supervisory_sanity_fault,
watchdog_sanity_fault: msg.watchdog_sanity_fault,
watchdog_system_present_fault: msg.watchdog_system_present_fault,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.component_type = self.component_type;
msg.accel = self.accel;
msg.brake = self.brake;
msg.cruise_control_buttons = self.cruise_control_buttons;
msg.dash_controls_left = self.dash_controls_left;
msg.dash_controls_right = self.dash_controls_right;
msg.hazard_lights = self.hazard_lights;
msg.headlight = self.headlight;
msg.horn = self.horn;
msg.media_controls = self.media_controls;
msg.parking_brake = self.parking_brake;
msg.shift = self.shift;
msg.sprayer = self.sprayer;
msg.steering = self.steering;
msg.turn = self.turn;
msg.wiper = self.wiper;
msg.watchdog = self.watchdog;
msg.brake_deccel = self.brake_deccel;
msg.rear_pass_door = self.rear_pass_door;
msg.engine_brake = self.engine_brake;
msg.marker_lamp = self.marker_lamp;
msg.cabin_climate = self.cabin_climate;
msg.cabin_fan_speed = self.cabin_fan_speed;
msg.cabin_temp = self.cabin_temp;
msg.counter = self.counter;
msg.complement = self.complement;
msg.config_fault = self.config_fault;
msg.can_timeout_fault = self.can_timeout_fault;
msg.internal_supply_voltage_fault = self.internal_supply_voltage_fault;
msg.supervisory_timeout = self.supervisory_timeout;
msg.supervisory_sanity_fault = self.supervisory_sanity_fault;
msg.watchdog_sanity_fault = self.watchdog_sanity_fault;
msg.watchdog_system_present_fault = self.watchdog_system_present_fault;
}



        }


                          
                          impl Default for ComponentRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<ComponentRpt>::new();
                                  ComponentRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct DateTimeRpt {

                              pub header: std_msgs::msg::Header,
pub year: i32,
pub month: u8,
pub day: u8,
pub hour: u8,
pub minute: u8,
pub second: u8,

                          }

                          impl WrappedTypesupport for DateTimeRpt { 

            type CStruct = pacmod3_msgs__msg__DateTimeRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__DateTimeRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__DateTimeRpt {

                unsafe { pacmod3_msgs__msg__DateTimeRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__DateTimeRpt) -> () {

                unsafe { pacmod3_msgs__msg__DateTimeRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> DateTimeRpt {
  DateTimeRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
year: msg.year,
month: msg.month,
day: msg.day,
hour: msg.hour,
minute: msg.minute,
second: msg.second,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.year = self.year;
msg.month = self.month;
msg.day = self.day;
msg.hour = self.hour;
msg.minute = self.minute;
msg.second = self.second;
}



        }


                          
                          impl Default for DateTimeRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<DateTimeRpt>::new();
                                  DateTimeRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct DetectedObjectRpt {

                              pub header: std_msgs::msg::Header,
pub front_object_distance_low_res: f64,
pub front_object_distance_high_res: f64,

                          }

                          impl WrappedTypesupport for DetectedObjectRpt { 

            type CStruct = pacmod3_msgs__msg__DetectedObjectRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__DetectedObjectRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__DetectedObjectRpt {

                unsafe { pacmod3_msgs__msg__DetectedObjectRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__DetectedObjectRpt) -> () {

                unsafe { pacmod3_msgs__msg__DetectedObjectRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> DetectedObjectRpt {
  DetectedObjectRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
front_object_distance_low_res: msg.front_object_distance_low_res,
front_object_distance_high_res: msg.front_object_distance_high_res,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.front_object_distance_low_res = self.front_object_distance_low_res;
msg.front_object_distance_high_res = self.front_object_distance_high_res;
}



        }


                          
                          impl Default for DetectedObjectRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<DetectedObjectRpt>::new();
                                  DetectedObjectRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct DoorRpt {

                              pub header: std_msgs::msg::Header,
pub driver_door_open: bool,
pub passenger_door_open: bool,
pub rear_driver_door_open: bool,
pub rear_passenger_door_open: bool,
pub hood_open: bool,
pub trunk_open: bool,
pub fuel_door_open: bool,
pub driver_door_open_avail: bool,
pub passenger_door_open_avail: bool,
pub rear_driver_door_open_avail: bool,
pub rear_passenger_door_open_avail: bool,
pub hood_open_avail: bool,
pub trunk_open_avail: bool,
pub fuel_door_open_avail: bool,

                          }

                          impl WrappedTypesupport for DoorRpt { 

            type CStruct = pacmod3_msgs__msg__DoorRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__DoorRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__DoorRpt {

                unsafe { pacmod3_msgs__msg__DoorRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__DoorRpt) -> () {

                unsafe { pacmod3_msgs__msg__DoorRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> DoorRpt {
  DoorRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
driver_door_open: msg.driver_door_open,
passenger_door_open: msg.passenger_door_open,
rear_driver_door_open: msg.rear_driver_door_open,
rear_passenger_door_open: msg.rear_passenger_door_open,
hood_open: msg.hood_open,
trunk_open: msg.trunk_open,
fuel_door_open: msg.fuel_door_open,
driver_door_open_avail: msg.driver_door_open_avail,
passenger_door_open_avail: msg.passenger_door_open_avail,
rear_driver_door_open_avail: msg.rear_driver_door_open_avail,
rear_passenger_door_open_avail: msg.rear_passenger_door_open_avail,
hood_open_avail: msg.hood_open_avail,
trunk_open_avail: msg.trunk_open_avail,
fuel_door_open_avail: msg.fuel_door_open_avail,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.driver_door_open = self.driver_door_open;
msg.passenger_door_open = self.passenger_door_open;
msg.rear_driver_door_open = self.rear_driver_door_open;
msg.rear_passenger_door_open = self.rear_passenger_door_open;
msg.hood_open = self.hood_open;
msg.trunk_open = self.trunk_open;
msg.fuel_door_open = self.fuel_door_open;
msg.driver_door_open_avail = self.driver_door_open_avail;
msg.passenger_door_open_avail = self.passenger_door_open_avail;
msg.rear_driver_door_open_avail = self.rear_driver_door_open_avail;
msg.rear_passenger_door_open_avail = self.rear_passenger_door_open_avail;
msg.hood_open_avail = self.hood_open_avail;
msg.trunk_open_avail = self.trunk_open_avail;
msg.fuel_door_open_avail = self.fuel_door_open_avail;
}



        }


                          
                          impl Default for DoorRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<DoorRpt>::new();
                                  DoorRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct DriveTrainFeatureRpt {

                              pub header: std_msgs::msg::Header,
pub antilock_brake_active: bool,
pub traction_control_active: bool,
pub four_wheel_drive_active: bool,
pub antilock_brake_active_avail: bool,
pub traction_control_active_avail: bool,
pub four_wheel_drive_active_avail: bool,
pub drive_mode: u8,
pub drive_mode_avail: bool,

                          }

                          impl WrappedTypesupport for DriveTrainFeatureRpt { 

            type CStruct = pacmod3_msgs__msg__DriveTrainFeatureRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__DriveTrainFeatureRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__DriveTrainFeatureRpt {

                unsafe { pacmod3_msgs__msg__DriveTrainFeatureRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__DriveTrainFeatureRpt) -> () {

                unsafe { pacmod3_msgs__msg__DriveTrainFeatureRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> DriveTrainFeatureRpt {
  DriveTrainFeatureRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
antilock_brake_active: msg.antilock_brake_active,
traction_control_active: msg.traction_control_active,
four_wheel_drive_active: msg.four_wheel_drive_active,
antilock_brake_active_avail: msg.antilock_brake_active_avail,
traction_control_active_avail: msg.traction_control_active_avail,
four_wheel_drive_active_avail: msg.four_wheel_drive_active_avail,
drive_mode: msg.drive_mode,
drive_mode_avail: msg.drive_mode_avail,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.antilock_brake_active = self.antilock_brake_active;
msg.traction_control_active = self.traction_control_active;
msg.four_wheel_drive_active = self.four_wheel_drive_active;
msg.antilock_brake_active_avail = self.antilock_brake_active_avail;
msg.traction_control_active_avail = self.traction_control_active_avail;
msg.four_wheel_drive_active_avail = self.four_wheel_drive_active_avail;
msg.drive_mode = self.drive_mode;
msg.drive_mode_avail = self.drive_mode_avail;
}



        }


                          
                          impl Default for DriveTrainFeatureRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<DriveTrainFeatureRpt>::new();
                                  DriveTrainFeatureRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct EStopRpt {

                              pub header: std_msgs::msg::Header,
pub estop_status: bool,
pub estop_fault: bool,

                          }

                          impl WrappedTypesupport for EStopRpt { 

            type CStruct = pacmod3_msgs__msg__EStopRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__EStopRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__EStopRpt {

                unsafe { pacmod3_msgs__msg__EStopRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__EStopRpt) -> () {

                unsafe { pacmod3_msgs__msg__EStopRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> EStopRpt {
  EStopRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
estop_status: msg.estop_status,
estop_fault: msg.estop_fault,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.estop_status = self.estop_status;
msg.estop_fault = self.estop_fault;
}



        }


                          
                          impl Default for EStopRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<EStopRpt>::new();
                                  EStopRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct EngineRpt {

                              pub header: std_msgs::msg::Header,
pub engine_speed: f64,
pub engine_torque: f64,
pub engine_coolant_temp: i16,
pub engine_speed_avail: bool,
pub engine_torque_avail: bool,
pub engine_coolant_temp_avail: bool,
pub fuel_level_avail: bool,
pub fuel_level: f64,

                          }

                          impl WrappedTypesupport for EngineRpt { 

            type CStruct = pacmod3_msgs__msg__EngineRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__EngineRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__EngineRpt {

                unsafe { pacmod3_msgs__msg__EngineRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__EngineRpt) -> () {

                unsafe { pacmod3_msgs__msg__EngineRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> EngineRpt {
  EngineRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
engine_speed: msg.engine_speed,
engine_torque: msg.engine_torque,
engine_coolant_temp: msg.engine_coolant_temp,
engine_speed_avail: msg.engine_speed_avail,
engine_torque_avail: msg.engine_torque_avail,
engine_coolant_temp_avail: msg.engine_coolant_temp_avail,
fuel_level_avail: msg.fuel_level_avail,
fuel_level: msg.fuel_level,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.engine_speed = self.engine_speed;
msg.engine_torque = self.engine_torque;
msg.engine_coolant_temp = self.engine_coolant_temp;
msg.engine_speed_avail = self.engine_speed_avail;
msg.engine_torque_avail = self.engine_torque_avail;
msg.engine_coolant_temp_avail = self.engine_coolant_temp_avail;
msg.fuel_level_avail = self.fuel_level_avail;
msg.fuel_level = self.fuel_level;
}



        }


                          
                          impl Default for EngineRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<EngineRpt>::new();
                                  EngineRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct GlobalCmd {

                              pub header: std_msgs::msg::Header,
pub clear_faults: bool,

                          }

                          impl WrappedTypesupport for GlobalCmd { 

            type CStruct = pacmod3_msgs__msg__GlobalCmd; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__GlobalCmd() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__GlobalCmd {

                unsafe { pacmod3_msgs__msg__GlobalCmd__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__GlobalCmd) -> () {

                unsafe { pacmod3_msgs__msg__GlobalCmd__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> GlobalCmd {
  GlobalCmd {
header: std_msgs::msg::Header::from_native(&msg.header),
clear_faults: msg.clear_faults,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.clear_faults = self.clear_faults;
}



        }


                          
                          impl Default for GlobalCmd {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<GlobalCmd>::new();
                                  GlobalCmd::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct GlobalRpt {

                              pub header: std_msgs::msg::Header,
pub enabled: bool,
pub override_active: bool,
pub user_can_timeout: bool,
pub steering_can_timeout: bool,
pub brake_can_timeout: bool,
pub subsystem_can_timeout: bool,
pub vehicle_can_timeout: bool,
pub pacmod_sys_fault_active: bool,
pub supervisory_enable_required: bool,
pub config_fault_active: bool,
pub user_can_read_errors: u16,

                          }

                          impl WrappedTypesupport for GlobalRpt { 

            type CStruct = pacmod3_msgs__msg__GlobalRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__GlobalRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__GlobalRpt {

                unsafe { pacmod3_msgs__msg__GlobalRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__GlobalRpt) -> () {

                unsafe { pacmod3_msgs__msg__GlobalRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> GlobalRpt {
  GlobalRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
enabled: msg.enabled,
override_active: msg.override_active,
user_can_timeout: msg.user_can_timeout,
steering_can_timeout: msg.steering_can_timeout,
brake_can_timeout: msg.brake_can_timeout,
subsystem_can_timeout: msg.subsystem_can_timeout,
vehicle_can_timeout: msg.vehicle_can_timeout,
pacmod_sys_fault_active: msg.pacmod_sys_fault_active,
supervisory_enable_required: msg.supervisory_enable_required,
config_fault_active: msg.config_fault_active,
user_can_read_errors: msg.user_can_read_errors,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.enabled = self.enabled;
msg.override_active = self.override_active;
msg.user_can_timeout = self.user_can_timeout;
msg.steering_can_timeout = self.steering_can_timeout;
msg.brake_can_timeout = self.brake_can_timeout;
msg.subsystem_can_timeout = self.subsystem_can_timeout;
msg.vehicle_can_timeout = self.vehicle_can_timeout;
msg.pacmod_sys_fault_active = self.pacmod_sys_fault_active;
msg.supervisory_enable_required = self.supervisory_enable_required;
msg.config_fault_active = self.config_fault_active;
msg.user_can_read_errors = self.user_can_read_errors;
}



        }


                          
                          impl Default for GlobalRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<GlobalRpt>::new();
                                  GlobalRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct GlobalRpt2 {

                              pub header: std_msgs::msg::Header,
pub system_enabled: bool,
pub system_override_active: bool,
pub system_fault_active: bool,
pub supervisory_enable_required: bool,

                          }

                          impl WrappedTypesupport for GlobalRpt2 { 

            type CStruct = pacmod3_msgs__msg__GlobalRpt2; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__GlobalRpt2() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__GlobalRpt2 {

                unsafe { pacmod3_msgs__msg__GlobalRpt2__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__GlobalRpt2) -> () {

                unsafe { pacmod3_msgs__msg__GlobalRpt2__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> GlobalRpt2 {
  GlobalRpt2 {
header: std_msgs::msg::Header::from_native(&msg.header),
system_enabled: msg.system_enabled,
system_override_active: msg.system_override_active,
system_fault_active: msg.system_fault_active,
supervisory_enable_required: msg.supervisory_enable_required,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.system_enabled = self.system_enabled;
msg.system_override_active = self.system_override_active;
msg.system_fault_active = self.system_fault_active;
msg.supervisory_enable_required = self.supervisory_enable_required;
}



        }


                          
                          impl Default for GlobalRpt2 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<GlobalRpt2>::new();
                                  GlobalRpt2::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct HeadlightAuxRpt {

                              pub header: std_msgs::msg::Header,
pub headlights_on: bool,
pub headlights_on_bright: bool,
pub fog_lights_on: bool,
pub headlights_mode: u8,
pub headlights_on_avail: bool,
pub headlights_on_bright_avail: bool,
pub fog_lights_on_avail: bool,
pub headlights_mode_avail: bool,

                          }

                          impl WrappedTypesupport for HeadlightAuxRpt { 

            type CStruct = pacmod3_msgs__msg__HeadlightAuxRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__HeadlightAuxRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__HeadlightAuxRpt {

                unsafe { pacmod3_msgs__msg__HeadlightAuxRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__HeadlightAuxRpt) -> () {

                unsafe { pacmod3_msgs__msg__HeadlightAuxRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> HeadlightAuxRpt {
  HeadlightAuxRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
headlights_on: msg.headlights_on,
headlights_on_bright: msg.headlights_on_bright,
fog_lights_on: msg.fog_lights_on,
headlights_mode: msg.headlights_mode,
headlights_on_avail: msg.headlights_on_avail,
headlights_on_bright_avail: msg.headlights_on_bright_avail,
fog_lights_on_avail: msg.fog_lights_on_avail,
headlights_mode_avail: msg.headlights_mode_avail,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.headlights_on = self.headlights_on;
msg.headlights_on_bright = self.headlights_on_bright;
msg.fog_lights_on = self.fog_lights_on;
msg.headlights_mode = self.headlights_mode;
msg.headlights_on_avail = self.headlights_on_avail;
msg.headlights_on_bright_avail = self.headlights_on_bright_avail;
msg.fog_lights_on_avail = self.fog_lights_on_avail;
msg.headlights_mode_avail = self.headlights_mode_avail;
}



        }


                          
                          impl Default for HeadlightAuxRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<HeadlightAuxRpt>::new();
                                  HeadlightAuxRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct InteriorLightsRpt {

                              pub header: std_msgs::msg::Header,
pub front_dome_lights_on: bool,
pub rear_dome_lights_on: bool,
pub mood_lights_on: bool,
pub ambient_light_sensor: bool,
pub dim_level: u8,
pub front_dome_lights_on_avail: bool,
pub rear_dome_lights_on_avail: bool,
pub mood_lights_on_avail: bool,
pub dim_level_avail: bool,
pub ambient_light_sensor_avail: bool,

                          }

                          impl WrappedTypesupport for InteriorLightsRpt { 

            type CStruct = pacmod3_msgs__msg__InteriorLightsRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__InteriorLightsRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__InteriorLightsRpt {

                unsafe { pacmod3_msgs__msg__InteriorLightsRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__InteriorLightsRpt) -> () {

                unsafe { pacmod3_msgs__msg__InteriorLightsRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> InteriorLightsRpt {
  InteriorLightsRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
front_dome_lights_on: msg.front_dome_lights_on,
rear_dome_lights_on: msg.rear_dome_lights_on,
mood_lights_on: msg.mood_lights_on,
ambient_light_sensor: msg.ambient_light_sensor,
dim_level: msg.dim_level,
front_dome_lights_on_avail: msg.front_dome_lights_on_avail,
rear_dome_lights_on_avail: msg.rear_dome_lights_on_avail,
mood_lights_on_avail: msg.mood_lights_on_avail,
dim_level_avail: msg.dim_level_avail,
ambient_light_sensor_avail: msg.ambient_light_sensor_avail,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.front_dome_lights_on = self.front_dome_lights_on;
msg.rear_dome_lights_on = self.rear_dome_lights_on;
msg.mood_lights_on = self.mood_lights_on;
msg.ambient_light_sensor = self.ambient_light_sensor;
msg.dim_level = self.dim_level;
msg.front_dome_lights_on_avail = self.front_dome_lights_on_avail;
msg.rear_dome_lights_on_avail = self.rear_dome_lights_on_avail;
msg.mood_lights_on_avail = self.mood_lights_on_avail;
msg.dim_level_avail = self.dim_level_avail;
msg.ambient_light_sensor_avail = self.ambient_light_sensor_avail;
}



        }


                          
                          impl Default for InteriorLightsRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<InteriorLightsRpt>::new();
                                  InteriorLightsRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct KeyValuePair {

                              pub key: std::string::String,
pub value: std::string::String,

                          }

                          impl WrappedTypesupport for KeyValuePair { 

            type CStruct = pacmod3_msgs__msg__KeyValuePair; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__KeyValuePair() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__KeyValuePair {

                unsafe { pacmod3_msgs__msg__KeyValuePair__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__KeyValuePair) -> () {

                unsafe { pacmod3_msgs__msg__KeyValuePair__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> KeyValuePair {
  KeyValuePair {
key: msg.key.to_str().to_owned(),
value: msg.value.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.key.assign(&self.key);
msg.value.assign(&self.value);
}



        }


                          
                          impl Default for KeyValuePair {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<KeyValuePair>::new();
                                  KeyValuePair::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct LatLonHeadingRpt {

                              pub header: std_msgs::msg::Header,
pub latitude_degrees: i8,
pub latitude_minutes: u8,
pub latitude_seconds: u8,
pub longitude_degrees: i8,
pub longitude_minutes: u8,
pub longitude_seconds: u8,
pub heading: f64,

                          }

                          impl WrappedTypesupport for LatLonHeadingRpt { 

            type CStruct = pacmod3_msgs__msg__LatLonHeadingRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__LatLonHeadingRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__LatLonHeadingRpt {

                unsafe { pacmod3_msgs__msg__LatLonHeadingRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__LatLonHeadingRpt) -> () {

                unsafe { pacmod3_msgs__msg__LatLonHeadingRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> LatLonHeadingRpt {
  LatLonHeadingRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
latitude_degrees: msg.latitude_degrees,
latitude_minutes: msg.latitude_minutes,
latitude_seconds: msg.latitude_seconds,
longitude_degrees: msg.longitude_degrees,
longitude_minutes: msg.longitude_minutes,
longitude_seconds: msg.longitude_seconds,
heading: msg.heading,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.latitude_degrees = self.latitude_degrees;
msg.latitude_minutes = self.latitude_minutes;
msg.latitude_seconds = self.latitude_seconds;
msg.longitude_degrees = self.longitude_degrees;
msg.longitude_minutes = self.longitude_minutes;
msg.longitude_seconds = self.longitude_seconds;
msg.heading = self.heading;
}



        }


                          
                          impl Default for LatLonHeadingRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<LatLonHeadingRpt>::new();
                                  LatLonHeadingRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct LinearAccelRpt {

                              pub header: std_msgs::msg::Header,
pub lateral_new_data_rx: bool,
pub longitudinal_new_data_rx: bool,
pub vertical_new_data_rx: bool,
pub lateral_valid: bool,
pub longitudinal_valid: bool,
pub vertical_valid: bool,
pub lateral_accel: f64,
pub longitudinal_accel: f64,
pub vertical_accel: f64,

                          }

                          impl WrappedTypesupport for LinearAccelRpt { 

            type CStruct = pacmod3_msgs__msg__LinearAccelRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__LinearAccelRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__LinearAccelRpt {

                unsafe { pacmod3_msgs__msg__LinearAccelRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__LinearAccelRpt) -> () {

                unsafe { pacmod3_msgs__msg__LinearAccelRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> LinearAccelRpt {
  LinearAccelRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
lateral_new_data_rx: msg.lateral_new_data_rx,
longitudinal_new_data_rx: msg.longitudinal_new_data_rx,
vertical_new_data_rx: msg.vertical_new_data_rx,
lateral_valid: msg.lateral_valid,
longitudinal_valid: msg.longitudinal_valid,
vertical_valid: msg.vertical_valid,
lateral_accel: msg.lateral_accel,
longitudinal_accel: msg.longitudinal_accel,
vertical_accel: msg.vertical_accel,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.lateral_new_data_rx = self.lateral_new_data_rx;
msg.longitudinal_new_data_rx = self.longitudinal_new_data_rx;
msg.vertical_new_data_rx = self.vertical_new_data_rx;
msg.lateral_valid = self.lateral_valid;
msg.longitudinal_valid = self.longitudinal_valid;
msg.vertical_valid = self.vertical_valid;
msg.lateral_accel = self.lateral_accel;
msg.longitudinal_accel = self.longitudinal_accel;
msg.vertical_accel = self.vertical_accel;
}



        }


                          
                          impl Default for LinearAccelRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<LinearAccelRpt>::new();
                                  LinearAccelRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct MotorRpt1 {

                              pub header: std_msgs::msg::Header,
pub current: f64,
pub position: f64,

                          }

                          impl WrappedTypesupport for MotorRpt1 { 

            type CStruct = pacmod3_msgs__msg__MotorRpt1; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__MotorRpt1() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__MotorRpt1 {

                unsafe { pacmod3_msgs__msg__MotorRpt1__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__MotorRpt1) -> () {

                unsafe { pacmod3_msgs__msg__MotorRpt1__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> MotorRpt1 {
  MotorRpt1 {
header: std_msgs::msg::Header::from_native(&msg.header),
current: msg.current,
position: msg.position,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.current = self.current;
msg.position = self.position;
}



        }


                          
                          impl Default for MotorRpt1 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<MotorRpt1>::new();
                                  MotorRpt1::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct MotorRpt2 {

                              pub header: std_msgs::msg::Header,
pub encoder_temp: f64,
pub motor_temp: f64,
pub angular_speed: f64,

                          }

                          impl WrappedTypesupport for MotorRpt2 { 

            type CStruct = pacmod3_msgs__msg__MotorRpt2; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__MotorRpt2() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__MotorRpt2 {

                unsafe { pacmod3_msgs__msg__MotorRpt2__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__MotorRpt2) -> () {

                unsafe { pacmod3_msgs__msg__MotorRpt2__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> MotorRpt2 {
  MotorRpt2 {
header: std_msgs::msg::Header::from_native(&msg.header),
encoder_temp: msg.encoder_temp,
motor_temp: msg.motor_temp,
angular_speed: msg.angular_speed,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.encoder_temp = self.encoder_temp;
msg.motor_temp = self.motor_temp;
msg.angular_speed = self.angular_speed;
}



        }


                          
                          impl Default for MotorRpt2 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<MotorRpt2>::new();
                                  MotorRpt2::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct MotorRpt3 {

                              pub header: std_msgs::msg::Header,
pub torque_output: f64,
pub torque_input: f64,

                          }

                          impl WrappedTypesupport for MotorRpt3 { 

            type CStruct = pacmod3_msgs__msg__MotorRpt3; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__MotorRpt3() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__MotorRpt3 {

                unsafe { pacmod3_msgs__msg__MotorRpt3__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__MotorRpt3) -> () {

                unsafe { pacmod3_msgs__msg__MotorRpt3__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> MotorRpt3 {
  MotorRpt3 {
header: std_msgs::msg::Header::from_native(&msg.header),
torque_output: msg.torque_output,
torque_input: msg.torque_input,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.torque_output = self.torque_output;
msg.torque_input = self.torque_input;
}



        }


                          
                          impl Default for MotorRpt3 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<MotorRpt3>::new();
                                  MotorRpt3::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct NotificationCmd {

                              pub header: std_msgs::msg::Header,
pub buzzer_mute: bool,
pub underdash_lights_white: bool,

                          }

                          impl WrappedTypesupport for NotificationCmd { 

            type CStruct = pacmod3_msgs__msg__NotificationCmd; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__NotificationCmd() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__NotificationCmd {

                unsafe { pacmod3_msgs__msg__NotificationCmd__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__NotificationCmd) -> () {

                unsafe { pacmod3_msgs__msg__NotificationCmd__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> NotificationCmd {
  NotificationCmd {
header: std_msgs::msg::Header::from_native(&msg.header),
buzzer_mute: msg.buzzer_mute,
underdash_lights_white: msg.underdash_lights_white,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.buzzer_mute = self.buzzer_mute;
msg.underdash_lights_white = self.underdash_lights_white;
}



        }


                          
                          impl Default for NotificationCmd {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<NotificationCmd>::new();
                                  NotificationCmd::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct OccupancyRpt {

                              pub header: std_msgs::msg::Header,
pub driver_seat_occupied: bool,
pub passenger_seat_occupied: bool,
pub rear_seat_occupied: bool,
pub driver_seatbelt_buckled: bool,
pub passenger_seatbelt_buckled: bool,
pub driver_rear_seatbelt_buckled: bool,
pub pass_rear_seatbelt_buckled: bool,
pub center_rear_seatbelt_buckled: bool,
pub driver_seat_occupied_avail: bool,
pub passenger_seat_occupied_avail: bool,
pub rear_seat_occupied_avail: bool,
pub driver_seatbelt_buckled_avail: bool,
pub passenger_seatbelt_buckled_avail: bool,
pub driver_rear_seatbelt_buckled_avail: bool,
pub pass_rear_seatbelt_buckled_avail: bool,
pub center_rear_seatbelt_buckled_avail: bool,

                          }

                          impl WrappedTypesupport for OccupancyRpt { 

            type CStruct = pacmod3_msgs__msg__OccupancyRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__OccupancyRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__OccupancyRpt {

                unsafe { pacmod3_msgs__msg__OccupancyRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__OccupancyRpt) -> () {

                unsafe { pacmod3_msgs__msg__OccupancyRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> OccupancyRpt {
  OccupancyRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
driver_seat_occupied: msg.driver_seat_occupied,
passenger_seat_occupied: msg.passenger_seat_occupied,
rear_seat_occupied: msg.rear_seat_occupied,
driver_seatbelt_buckled: msg.driver_seatbelt_buckled,
passenger_seatbelt_buckled: msg.passenger_seatbelt_buckled,
driver_rear_seatbelt_buckled: msg.driver_rear_seatbelt_buckled,
pass_rear_seatbelt_buckled: msg.pass_rear_seatbelt_buckled,
center_rear_seatbelt_buckled: msg.center_rear_seatbelt_buckled,
driver_seat_occupied_avail: msg.driver_seat_occupied_avail,
passenger_seat_occupied_avail: msg.passenger_seat_occupied_avail,
rear_seat_occupied_avail: msg.rear_seat_occupied_avail,
driver_seatbelt_buckled_avail: msg.driver_seatbelt_buckled_avail,
passenger_seatbelt_buckled_avail: msg.passenger_seatbelt_buckled_avail,
driver_rear_seatbelt_buckled_avail: msg.driver_rear_seatbelt_buckled_avail,
pass_rear_seatbelt_buckled_avail: msg.pass_rear_seatbelt_buckled_avail,
center_rear_seatbelt_buckled_avail: msg.center_rear_seatbelt_buckled_avail,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.driver_seat_occupied = self.driver_seat_occupied;
msg.passenger_seat_occupied = self.passenger_seat_occupied;
msg.rear_seat_occupied = self.rear_seat_occupied;
msg.driver_seatbelt_buckled = self.driver_seatbelt_buckled;
msg.passenger_seatbelt_buckled = self.passenger_seatbelt_buckled;
msg.driver_rear_seatbelt_buckled = self.driver_rear_seatbelt_buckled;
msg.pass_rear_seatbelt_buckled = self.pass_rear_seatbelt_buckled;
msg.center_rear_seatbelt_buckled = self.center_rear_seatbelt_buckled;
msg.driver_seat_occupied_avail = self.driver_seat_occupied_avail;
msg.passenger_seat_occupied_avail = self.passenger_seat_occupied_avail;
msg.rear_seat_occupied_avail = self.rear_seat_occupied_avail;
msg.driver_seatbelt_buckled_avail = self.driver_seatbelt_buckled_avail;
msg.passenger_seatbelt_buckled_avail = self.passenger_seatbelt_buckled_avail;
msg.driver_rear_seatbelt_buckled_avail = self.driver_rear_seatbelt_buckled_avail;
msg.pass_rear_seatbelt_buckled_avail = self.pass_rear_seatbelt_buckled_avail;
msg.center_rear_seatbelt_buckled_avail = self.center_rear_seatbelt_buckled_avail;
}



        }


                          
                          impl Default for OccupancyRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<OccupancyRpt>::new();
                                  OccupancyRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct ParkingBrakeAuxRpt {

                              pub header: std_msgs::msg::Header,
pub parking_brake_status: u8,
pub parking_brake_status_avail: bool,

                          }

                          impl WrappedTypesupport for ParkingBrakeAuxRpt { 

            type CStruct = pacmod3_msgs__msg__ParkingBrakeAuxRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__ParkingBrakeAuxRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__ParkingBrakeAuxRpt {

                unsafe { pacmod3_msgs__msg__ParkingBrakeAuxRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__ParkingBrakeAuxRpt) -> () {

                unsafe { pacmod3_msgs__msg__ParkingBrakeAuxRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> ParkingBrakeAuxRpt {
  ParkingBrakeAuxRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
parking_brake_status: msg.parking_brake_status,
parking_brake_status_avail: msg.parking_brake_status_avail,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.parking_brake_status = self.parking_brake_status;
msg.parking_brake_status_avail = self.parking_brake_status_avail;
}



        }


                          
                          impl Default for ParkingBrakeAuxRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<ParkingBrakeAuxRpt>::new();
                                  ParkingBrakeAuxRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct RearLightsRpt {

                              pub header: std_msgs::msg::Header,
pub brake_lights_on: bool,
pub reverse_lights_on: bool,
pub brake_lights_on_avail: bool,
pub reverse_lights_on_avail: bool,

                          }

                          impl WrappedTypesupport for RearLightsRpt { 

            type CStruct = pacmod3_msgs__msg__RearLightsRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__RearLightsRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__RearLightsRpt {

                unsafe { pacmod3_msgs__msg__RearLightsRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__RearLightsRpt) -> () {

                unsafe { pacmod3_msgs__msg__RearLightsRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> RearLightsRpt {
  RearLightsRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
brake_lights_on: msg.brake_lights_on,
reverse_lights_on: msg.reverse_lights_on,
brake_lights_on_avail: msg.brake_lights_on_avail,
reverse_lights_on_avail: msg.reverse_lights_on_avail,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.brake_lights_on = self.brake_lights_on;
msg.reverse_lights_on = self.reverse_lights_on;
msg.brake_lights_on_avail = self.brake_lights_on_avail;
msg.reverse_lights_on_avail = self.reverse_lights_on_avail;
}



        }


                          
                          impl Default for RearLightsRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<RearLightsRpt>::new();
                                  RearLightsRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SafetyBrakeCmd {

                              pub header: std_msgs::msg::Header,
pub safety_brake_cmd: bool,

                          }

                          impl WrappedTypesupport for SafetyBrakeCmd { 

            type CStruct = pacmod3_msgs__msg__SafetyBrakeCmd; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__SafetyBrakeCmd() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__SafetyBrakeCmd {

                unsafe { pacmod3_msgs__msg__SafetyBrakeCmd__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__SafetyBrakeCmd) -> () {

                unsafe { pacmod3_msgs__msg__SafetyBrakeCmd__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SafetyBrakeCmd {
  SafetyBrakeCmd {
header: std_msgs::msg::Header::from_native(&msg.header),
safety_brake_cmd: msg.safety_brake_cmd,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.safety_brake_cmd = self.safety_brake_cmd;
}



        }


                          
                          impl Default for SafetyBrakeCmd {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SafetyBrakeCmd>::new();
                                  SafetyBrakeCmd::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SafetyBrakeRpt {

                              pub header: std_msgs::msg::Header,
pub commanded_val: bool,
pub output_val: u8,
pub cmd_reported_fault: bool,
pub cmd_timeout: bool,
pub cmd_permitted: bool,
pub reported_fault: bool,

                          }

                          impl WrappedTypesupport for SafetyBrakeRpt { 

            type CStruct = pacmod3_msgs__msg__SafetyBrakeRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__SafetyBrakeRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__SafetyBrakeRpt {

                unsafe { pacmod3_msgs__msg__SafetyBrakeRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__SafetyBrakeRpt) -> () {

                unsafe { pacmod3_msgs__msg__SafetyBrakeRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SafetyBrakeRpt {
  SafetyBrakeRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
commanded_val: msg.commanded_val,
output_val: msg.output_val,
cmd_reported_fault: msg.cmd_reported_fault,
cmd_timeout: msg.cmd_timeout,
cmd_permitted: msg.cmd_permitted,
reported_fault: msg.reported_fault,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.commanded_val = self.commanded_val;
msg.output_val = self.output_val;
msg.cmd_reported_fault = self.cmd_reported_fault;
msg.cmd_timeout = self.cmd_timeout;
msg.cmd_permitted = self.cmd_permitted;
msg.reported_fault = self.reported_fault;
}



        }


                          
                          impl Default for SafetyBrakeRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SafetyBrakeRpt>::new();
                                  SafetyBrakeRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SafetyFuncCmd {

                              pub header: std_msgs::msg::Header,
pub command: u16,

                          }

                          impl WrappedTypesupport for SafetyFuncCmd { 

            type CStruct = pacmod3_msgs__msg__SafetyFuncCmd; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__SafetyFuncCmd() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__SafetyFuncCmd {

                unsafe { pacmod3_msgs__msg__SafetyFuncCmd__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__SafetyFuncCmd) -> () {

                unsafe { pacmod3_msgs__msg__SafetyFuncCmd__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SafetyFuncCmd {
  SafetyFuncCmd {
header: std_msgs::msg::Header::from_native(&msg.header),
command: msg.command,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.command = self.command;
}



        }


                          
                          impl Default for SafetyFuncCmd {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SafetyFuncCmd>::new();
                                  SafetyFuncCmd::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SafetyFuncCriticalStopRpt {

                              pub header: std_msgs::msg::Header,
pub automaual_opctrl_fault: bool,
pub remote_stop_fault: bool,
pub safety_brake_opctrl_off: bool,
pub safety_brake_cmd_timeout: bool,
pub safety_func_cmd_timeout: bool,
pub safety_func_critical_stop_1_cmd: bool,
pub safety_func_critical_stop_2_cmd: bool,
pub safety_func_none_cmd: bool,
pub pacmod_system_timeout: bool,
pub pacmod_system_fault: bool,
pub pacmod_system_not_active: bool,
pub vehicle_report_timeout: bool,
pub vehicle_report_fault: bool,
pub low_engine_rpm: bool,
pub pri_safety_brake_signal_1_fault: bool,
pub pri_safety_brake_signal_2_fault: bool,
pub sec_safety_brake_signal_1_fault: bool,
pub sec_safety_brake_signal_2_fault: bool,
pub primary_processor_fault: bool,
pub secondary_processor_fault: bool,
pub remote_stop_cmd: bool,
pub pri_safety_brake_pressure_fault: bool,

                          }

                          impl WrappedTypesupport for SafetyFuncCriticalStopRpt { 

            type CStruct = pacmod3_msgs__msg__SafetyFuncCriticalStopRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__SafetyFuncCriticalStopRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__SafetyFuncCriticalStopRpt {

                unsafe { pacmod3_msgs__msg__SafetyFuncCriticalStopRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__SafetyFuncCriticalStopRpt) -> () {

                unsafe { pacmod3_msgs__msg__SafetyFuncCriticalStopRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SafetyFuncCriticalStopRpt {
  SafetyFuncCriticalStopRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
automaual_opctrl_fault: msg.automaual_opctrl_fault,
remote_stop_fault: msg.remote_stop_fault,
safety_brake_opctrl_off: msg.safety_brake_opctrl_off,
safety_brake_cmd_timeout: msg.safety_brake_cmd_timeout,
safety_func_cmd_timeout: msg.safety_func_cmd_timeout,
safety_func_critical_stop_1_cmd: msg.safety_func_critical_stop_1_cmd,
safety_func_critical_stop_2_cmd: msg.safety_func_critical_stop_2_cmd,
safety_func_none_cmd: msg.safety_func_none_cmd,
pacmod_system_timeout: msg.pacmod_system_timeout,
pacmod_system_fault: msg.pacmod_system_fault,
pacmod_system_not_active: msg.pacmod_system_not_active,
vehicle_report_timeout: msg.vehicle_report_timeout,
vehicle_report_fault: msg.vehicle_report_fault,
low_engine_rpm: msg.low_engine_rpm,
pri_safety_brake_signal_1_fault: msg.pri_safety_brake_signal_1_fault,
pri_safety_brake_signal_2_fault: msg.pri_safety_brake_signal_2_fault,
sec_safety_brake_signal_1_fault: msg.sec_safety_brake_signal_1_fault,
sec_safety_brake_signal_2_fault: msg.sec_safety_brake_signal_2_fault,
primary_processor_fault: msg.primary_processor_fault,
secondary_processor_fault: msg.secondary_processor_fault,
remote_stop_cmd: msg.remote_stop_cmd,
pri_safety_brake_pressure_fault: msg.pri_safety_brake_pressure_fault,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.automaual_opctrl_fault = self.automaual_opctrl_fault;
msg.remote_stop_fault = self.remote_stop_fault;
msg.safety_brake_opctrl_off = self.safety_brake_opctrl_off;
msg.safety_brake_cmd_timeout = self.safety_brake_cmd_timeout;
msg.safety_func_cmd_timeout = self.safety_func_cmd_timeout;
msg.safety_func_critical_stop_1_cmd = self.safety_func_critical_stop_1_cmd;
msg.safety_func_critical_stop_2_cmd = self.safety_func_critical_stop_2_cmd;
msg.safety_func_none_cmd = self.safety_func_none_cmd;
msg.pacmod_system_timeout = self.pacmod_system_timeout;
msg.pacmod_system_fault = self.pacmod_system_fault;
msg.pacmod_system_not_active = self.pacmod_system_not_active;
msg.vehicle_report_timeout = self.vehicle_report_timeout;
msg.vehicle_report_fault = self.vehicle_report_fault;
msg.low_engine_rpm = self.low_engine_rpm;
msg.pri_safety_brake_signal_1_fault = self.pri_safety_brake_signal_1_fault;
msg.pri_safety_brake_signal_2_fault = self.pri_safety_brake_signal_2_fault;
msg.sec_safety_brake_signal_1_fault = self.sec_safety_brake_signal_1_fault;
msg.sec_safety_brake_signal_2_fault = self.sec_safety_brake_signal_2_fault;
msg.primary_processor_fault = self.primary_processor_fault;
msg.secondary_processor_fault = self.secondary_processor_fault;
msg.remote_stop_cmd = self.remote_stop_cmd;
msg.pri_safety_brake_pressure_fault = self.pri_safety_brake_pressure_fault;
}



        }


                          
                          impl Default for SafetyFuncCriticalStopRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SafetyFuncCriticalStopRpt>::new();
                                  SafetyFuncCriticalStopRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SafetyFuncRpt {

                              pub header: std_msgs::msg::Header,
pub commanded_val: u16,
pub state: u16,
pub automanual_opctrl: u16,
pub cabin_safety_brake_opctrl: u16,
pub remote_stop_status: u16,
pub engine_status: bool,
pub pacmod_system_status: bool,
pub user_pc_fault: u16,
pub pacmod_system_fault: u16,
pub manual_state_obtainable: bool,
pub auto_ready_state_obtainable: bool,
pub auto_state_obtainable: bool,
pub manual_ready_state_obtainable: bool,
pub critical_stop1_state_obtainable: bool,
pub critical_stop2_state_obtainable: bool,

                          }

                          impl WrappedTypesupport for SafetyFuncRpt { 

            type CStruct = pacmod3_msgs__msg__SafetyFuncRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__SafetyFuncRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__SafetyFuncRpt {

                unsafe { pacmod3_msgs__msg__SafetyFuncRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__SafetyFuncRpt) -> () {

                unsafe { pacmod3_msgs__msg__SafetyFuncRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SafetyFuncRpt {
  SafetyFuncRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
commanded_val: msg.commanded_val,
state: msg.state,
automanual_opctrl: msg.automanual_opctrl,
cabin_safety_brake_opctrl: msg.cabin_safety_brake_opctrl,
remote_stop_status: msg.remote_stop_status,
engine_status: msg.engine_status,
pacmod_system_status: msg.pacmod_system_status,
user_pc_fault: msg.user_pc_fault,
pacmod_system_fault: msg.pacmod_system_fault,
manual_state_obtainable: msg.manual_state_obtainable,
auto_ready_state_obtainable: msg.auto_ready_state_obtainable,
auto_state_obtainable: msg.auto_state_obtainable,
manual_ready_state_obtainable: msg.manual_ready_state_obtainable,
critical_stop1_state_obtainable: msg.critical_stop1_state_obtainable,
critical_stop2_state_obtainable: msg.critical_stop2_state_obtainable,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.commanded_val = self.commanded_val;
msg.state = self.state;
msg.automanual_opctrl = self.automanual_opctrl;
msg.cabin_safety_brake_opctrl = self.cabin_safety_brake_opctrl;
msg.remote_stop_status = self.remote_stop_status;
msg.engine_status = self.engine_status;
msg.pacmod_system_status = self.pacmod_system_status;
msg.user_pc_fault = self.user_pc_fault;
msg.pacmod_system_fault = self.pacmod_system_fault;
msg.manual_state_obtainable = self.manual_state_obtainable;
msg.auto_ready_state_obtainable = self.auto_ready_state_obtainable;
msg.auto_state_obtainable = self.auto_state_obtainable;
msg.manual_ready_state_obtainable = self.manual_ready_state_obtainable;
msg.critical_stop1_state_obtainable = self.critical_stop1_state_obtainable;
msg.critical_stop2_state_obtainable = self.critical_stop2_state_obtainable;
}



        }


                          
                          impl Default for SafetyFuncRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SafetyFuncRpt>::new();
                                  SafetyFuncRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct ShiftAuxRpt {

                              pub header: std_msgs::msg::Header,
pub between_gears: bool,
pub stay_in_neutral_mode: bool,
pub brake_interlock_active: bool,
pub speed_interlock_active: bool,
pub write_to_config: bool,
pub between_gears_avail: bool,
pub stay_in_neutral_mode_avail: bool,
pub brake_interlock_active_avail: bool,
pub speed_interlock_active_avail: bool,
pub write_to_config_is_valid: bool,
pub gear_number_avail: bool,
pub gear_number: i8,

                          }

                          impl WrappedTypesupport for ShiftAuxRpt { 

            type CStruct = pacmod3_msgs__msg__ShiftAuxRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__ShiftAuxRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__ShiftAuxRpt {

                unsafe { pacmod3_msgs__msg__ShiftAuxRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__ShiftAuxRpt) -> () {

                unsafe { pacmod3_msgs__msg__ShiftAuxRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> ShiftAuxRpt {
  ShiftAuxRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
between_gears: msg.between_gears,
stay_in_neutral_mode: msg.stay_in_neutral_mode,
brake_interlock_active: msg.brake_interlock_active,
speed_interlock_active: msg.speed_interlock_active,
write_to_config: msg.write_to_config,
between_gears_avail: msg.between_gears_avail,
stay_in_neutral_mode_avail: msg.stay_in_neutral_mode_avail,
brake_interlock_active_avail: msg.brake_interlock_active_avail,
speed_interlock_active_avail: msg.speed_interlock_active_avail,
write_to_config_is_valid: msg.write_to_config_is_valid,
gear_number_avail: msg.gear_number_avail,
gear_number: msg.gear_number,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.between_gears = self.between_gears;
msg.stay_in_neutral_mode = self.stay_in_neutral_mode;
msg.brake_interlock_active = self.brake_interlock_active;
msg.speed_interlock_active = self.speed_interlock_active;
msg.write_to_config = self.write_to_config;
msg.between_gears_avail = self.between_gears_avail;
msg.stay_in_neutral_mode_avail = self.stay_in_neutral_mode_avail;
msg.brake_interlock_active_avail = self.brake_interlock_active_avail;
msg.speed_interlock_active_avail = self.speed_interlock_active_avail;
msg.write_to_config_is_valid = self.write_to_config_is_valid;
msg.gear_number_avail = self.gear_number_avail;
msg.gear_number = self.gear_number;
}



        }


                          
                          impl Default for ShiftAuxRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<ShiftAuxRpt>::new();
                                  ShiftAuxRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SoftwareVersionRpt {

                              pub header: std_msgs::msg::Header,
pub mjr: u16,
pub mnr: u16,
pub patch: u16,
pub build0: u8,
pub build1: u8,
pub build2: u8,
pub build3: u8,

                          }

                          impl WrappedTypesupport for SoftwareVersionRpt { 

            type CStruct = pacmod3_msgs__msg__SoftwareVersionRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__SoftwareVersionRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__SoftwareVersionRpt {

                unsafe { pacmod3_msgs__msg__SoftwareVersionRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__SoftwareVersionRpt) -> () {

                unsafe { pacmod3_msgs__msg__SoftwareVersionRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SoftwareVersionRpt {
  SoftwareVersionRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
mjr: msg.mjr,
mnr: msg.mnr,
patch: msg.patch,
build0: msg.build0,
build1: msg.build1,
build2: msg.build2,
build3: msg.build3,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.mjr = self.mjr;
msg.mnr = self.mnr;
msg.patch = self.patch;
msg.build0 = self.build0;
msg.build1 = self.build1;
msg.build2 = self.build2;
msg.build3 = self.build3;
}



        }


                          
                          impl Default for SoftwareVersionRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SoftwareVersionRpt>::new();
                                  SoftwareVersionRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SteeringAuxRpt {

                              pub header: std_msgs::msg::Header,
pub steering_torque: f64,
pub rotation_rate: f64,
pub operator_interaction: bool,
pub rotation_rate_sign: bool,
pub vehicle_angle_calib_status: bool,
pub steering_limiting_active: bool,
pub steering_torque_avail: bool,
pub rotation_rate_avail: bool,
pub operator_interaction_avail: bool,
pub rotation_rate_sign_avail: bool,
pub vehicle_angle_calib_status_avail: bool,
pub steering_limiting_active_avail: bool,

                          }

                          impl WrappedTypesupport for SteeringAuxRpt { 

            type CStruct = pacmod3_msgs__msg__SteeringAuxRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__SteeringAuxRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__SteeringAuxRpt {

                unsafe { pacmod3_msgs__msg__SteeringAuxRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__SteeringAuxRpt) -> () {

                unsafe { pacmod3_msgs__msg__SteeringAuxRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SteeringAuxRpt {
  SteeringAuxRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
steering_torque: msg.steering_torque,
rotation_rate: msg.rotation_rate,
operator_interaction: msg.operator_interaction,
rotation_rate_sign: msg.rotation_rate_sign,
vehicle_angle_calib_status: msg.vehicle_angle_calib_status,
steering_limiting_active: msg.steering_limiting_active,
steering_torque_avail: msg.steering_torque_avail,
rotation_rate_avail: msg.rotation_rate_avail,
operator_interaction_avail: msg.operator_interaction_avail,
rotation_rate_sign_avail: msg.rotation_rate_sign_avail,
vehicle_angle_calib_status_avail: msg.vehicle_angle_calib_status_avail,
steering_limiting_active_avail: msg.steering_limiting_active_avail,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.steering_torque = self.steering_torque;
msg.rotation_rate = self.rotation_rate;
msg.operator_interaction = self.operator_interaction;
msg.rotation_rate_sign = self.rotation_rate_sign;
msg.vehicle_angle_calib_status = self.vehicle_angle_calib_status;
msg.steering_limiting_active = self.steering_limiting_active;
msg.steering_torque_avail = self.steering_torque_avail;
msg.rotation_rate_avail = self.rotation_rate_avail;
msg.operator_interaction_avail = self.operator_interaction_avail;
msg.rotation_rate_sign_avail = self.rotation_rate_sign_avail;
msg.vehicle_angle_calib_status_avail = self.vehicle_angle_calib_status_avail;
msg.steering_limiting_active_avail = self.steering_limiting_active_avail;
}



        }


                          
                          impl Default for SteeringAuxRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SteeringAuxRpt>::new();
                                  SteeringAuxRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SteeringCmd {

                              pub header: std_msgs::msg::Header,
pub enable: bool,
pub ignore_overrides: bool,
pub clear_override: bool,
pub command: f64,
pub rotation_rate: f64,

                          }

                          impl WrappedTypesupport for SteeringCmd { 

            type CStruct = pacmod3_msgs__msg__SteeringCmd; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__SteeringCmd() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__SteeringCmd {

                unsafe { pacmod3_msgs__msg__SteeringCmd__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__SteeringCmd) -> () {

                unsafe { pacmod3_msgs__msg__SteeringCmd__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SteeringCmd {
  SteeringCmd {
header: std_msgs::msg::Header::from_native(&msg.header),
enable: msg.enable,
ignore_overrides: msg.ignore_overrides,
clear_override: msg.clear_override,
command: msg.command,
rotation_rate: msg.rotation_rate,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.enable = self.enable;
msg.ignore_overrides = self.ignore_overrides;
msg.clear_override = self.clear_override;
msg.command = self.command;
msg.rotation_rate = self.rotation_rate;
}



        }


                          
                          impl Default for SteeringCmd {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SteeringCmd>::new();
                                  SteeringCmd::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SteeringCmdLimitRpt {

                              pub header: std_msgs::msg::Header,
pub pos_cmd_limit: f64,
pub limited_pos_cmd: f64,
pub rotation_rate_cmd_limit: f64,
pub limited_rotation_rate_cmd: f64,

                          }

                          impl WrappedTypesupport for SteeringCmdLimitRpt { 

            type CStruct = pacmod3_msgs__msg__SteeringCmdLimitRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__SteeringCmdLimitRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__SteeringCmdLimitRpt {

                unsafe { pacmod3_msgs__msg__SteeringCmdLimitRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__SteeringCmdLimitRpt) -> () {

                unsafe { pacmod3_msgs__msg__SteeringCmdLimitRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SteeringCmdLimitRpt {
  SteeringCmdLimitRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
pos_cmd_limit: msg.pos_cmd_limit,
limited_pos_cmd: msg.limited_pos_cmd,
rotation_rate_cmd_limit: msg.rotation_rate_cmd_limit,
limited_rotation_rate_cmd: msg.limited_rotation_rate_cmd,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.pos_cmd_limit = self.pos_cmd_limit;
msg.limited_pos_cmd = self.limited_pos_cmd;
msg.rotation_rate_cmd_limit = self.rotation_rate_cmd_limit;
msg.limited_rotation_rate_cmd = self.limited_rotation_rate_cmd;
}



        }


                          
                          impl Default for SteeringCmdLimitRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SteeringCmdLimitRpt>::new();
                                  SteeringCmdLimitRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SupervisoryCtrl {

                              pub header: std_msgs::msg::Header,
pub enable: bool,
pub counter: u8,
pub complement: u8,

                          }

                          impl WrappedTypesupport for SupervisoryCtrl { 

            type CStruct = pacmod3_msgs__msg__SupervisoryCtrl; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__SupervisoryCtrl() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__SupervisoryCtrl {

                unsafe { pacmod3_msgs__msg__SupervisoryCtrl__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__SupervisoryCtrl) -> () {

                unsafe { pacmod3_msgs__msg__SupervisoryCtrl__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SupervisoryCtrl {
  SupervisoryCtrl {
header: std_msgs::msg::Header::from_native(&msg.header),
enable: msg.enable,
counter: msg.counter,
complement: msg.complement,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.enable = self.enable;
msg.counter = self.counter;
msg.complement = self.complement;
}



        }


                          
                          impl Default for SupervisoryCtrl {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SupervisoryCtrl>::new();
                                  SupervisoryCtrl::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SystemCmdBool {

                              pub header: std_msgs::msg::Header,
pub enable: bool,
pub ignore_overrides: bool,
pub clear_override: bool,
pub command: bool,

                          }

                          impl WrappedTypesupport for SystemCmdBool { 

            type CStruct = pacmod3_msgs__msg__SystemCmdBool; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__SystemCmdBool() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__SystemCmdBool {

                unsafe { pacmod3_msgs__msg__SystemCmdBool__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__SystemCmdBool) -> () {

                unsafe { pacmod3_msgs__msg__SystemCmdBool__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SystemCmdBool {
  SystemCmdBool {
header: std_msgs::msg::Header::from_native(&msg.header),
enable: msg.enable,
ignore_overrides: msg.ignore_overrides,
clear_override: msg.clear_override,
command: msg.command,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.enable = self.enable;
msg.ignore_overrides = self.ignore_overrides;
msg.clear_override = self.clear_override;
msg.command = self.command;
}



        }


                          
                          impl Default for SystemCmdBool {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SystemCmdBool>::new();
                                  SystemCmdBool::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SystemCmdFloat {

                              pub header: std_msgs::msg::Header,
pub enable: bool,
pub ignore_overrides: bool,
pub clear_override: bool,
pub command: f64,

                          }

                          impl WrappedTypesupport for SystemCmdFloat { 

            type CStruct = pacmod3_msgs__msg__SystemCmdFloat; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__SystemCmdFloat() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__SystemCmdFloat {

                unsafe { pacmod3_msgs__msg__SystemCmdFloat__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__SystemCmdFloat) -> () {

                unsafe { pacmod3_msgs__msg__SystemCmdFloat__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SystemCmdFloat {
  SystemCmdFloat {
header: std_msgs::msg::Header::from_native(&msg.header),
enable: msg.enable,
ignore_overrides: msg.ignore_overrides,
clear_override: msg.clear_override,
command: msg.command,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.enable = self.enable;
msg.ignore_overrides = self.ignore_overrides;
msg.clear_override = self.clear_override;
msg.command = self.command;
}



        }


                          
                          impl Default for SystemCmdFloat {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SystemCmdFloat>::new();
                                  SystemCmdFloat::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SystemCmdInt {

                              pub header: std_msgs::msg::Header,
pub enable: bool,
pub ignore_overrides: bool,
pub clear_override: bool,
pub command: u16,

                          }

                          impl WrappedTypesupport for SystemCmdInt { 

            type CStruct = pacmod3_msgs__msg__SystemCmdInt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__SystemCmdInt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__SystemCmdInt {

                unsafe { pacmod3_msgs__msg__SystemCmdInt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__SystemCmdInt) -> () {

                unsafe { pacmod3_msgs__msg__SystemCmdInt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SystemCmdInt {
  SystemCmdInt {
header: std_msgs::msg::Header::from_native(&msg.header),
enable: msg.enable,
ignore_overrides: msg.ignore_overrides,
clear_override: msg.clear_override,
command: msg.command,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.enable = self.enable;
msg.ignore_overrides = self.ignore_overrides;
msg.clear_override = self.clear_override;
msg.command = self.command;
}



        }


                          
                          impl Default for SystemCmdInt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SystemCmdInt>::new();
                                  SystemCmdInt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SystemCmdLimitRpt {

                              pub header: std_msgs::msg::Header,
pub sys_cmd_limit: f64,
pub limited_sys_cmd: f64,

                          }

                          impl WrappedTypesupport for SystemCmdLimitRpt { 

            type CStruct = pacmod3_msgs__msg__SystemCmdLimitRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__SystemCmdLimitRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__SystemCmdLimitRpt {

                unsafe { pacmod3_msgs__msg__SystemCmdLimitRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__SystemCmdLimitRpt) -> () {

                unsafe { pacmod3_msgs__msg__SystemCmdLimitRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SystemCmdLimitRpt {
  SystemCmdLimitRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
sys_cmd_limit: msg.sys_cmd_limit,
limited_sys_cmd: msg.limited_sys_cmd,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.sys_cmd_limit = self.sys_cmd_limit;
msg.limited_sys_cmd = self.limited_sys_cmd;
}



        }


                          
                          impl Default for SystemCmdLimitRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SystemCmdLimitRpt>::new();
                                  SystemCmdLimitRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SystemRptBool {

                              pub header: std_msgs::msg::Header,
pub enabled: bool,
pub override_active: bool,
pub command_output_fault: bool,
pub input_output_fault: bool,
pub output_reported_fault: bool,
pub pacmod_fault: bool,
pub vehicle_fault: bool,
pub command_timeout: bool,
pub manual_input: bool,
pub command: bool,
pub output: bool,

                          }

                          impl WrappedTypesupport for SystemRptBool { 

            type CStruct = pacmod3_msgs__msg__SystemRptBool; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__SystemRptBool() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__SystemRptBool {

                unsafe { pacmod3_msgs__msg__SystemRptBool__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__SystemRptBool) -> () {

                unsafe { pacmod3_msgs__msg__SystemRptBool__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SystemRptBool {
  SystemRptBool {
header: std_msgs::msg::Header::from_native(&msg.header),
enabled: msg.enabled,
override_active: msg.override_active,
command_output_fault: msg.command_output_fault,
input_output_fault: msg.input_output_fault,
output_reported_fault: msg.output_reported_fault,
pacmod_fault: msg.pacmod_fault,
vehicle_fault: msg.vehicle_fault,
command_timeout: msg.command_timeout,
manual_input: msg.manual_input,
command: msg.command,
output: msg.output,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.enabled = self.enabled;
msg.override_active = self.override_active;
msg.command_output_fault = self.command_output_fault;
msg.input_output_fault = self.input_output_fault;
msg.output_reported_fault = self.output_reported_fault;
msg.pacmod_fault = self.pacmod_fault;
msg.vehicle_fault = self.vehicle_fault;
msg.command_timeout = self.command_timeout;
msg.manual_input = self.manual_input;
msg.command = self.command;
msg.output = self.output;
}



        }


                          
                          impl Default for SystemRptBool {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SystemRptBool>::new();
                                  SystemRptBool::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SystemRptFloat {

                              pub header: std_msgs::msg::Header,
pub enabled: bool,
pub override_active: bool,
pub command_output_fault: bool,
pub input_output_fault: bool,
pub output_reported_fault: bool,
pub pacmod_fault: bool,
pub vehicle_fault: bool,
pub command_timeout: bool,
pub manual_input: f64,
pub command: f64,
pub output: f64,

                          }

                          impl WrappedTypesupport for SystemRptFloat { 

            type CStruct = pacmod3_msgs__msg__SystemRptFloat; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__SystemRptFloat() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__SystemRptFloat {

                unsafe { pacmod3_msgs__msg__SystemRptFloat__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__SystemRptFloat) -> () {

                unsafe { pacmod3_msgs__msg__SystemRptFloat__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SystemRptFloat {
  SystemRptFloat {
header: std_msgs::msg::Header::from_native(&msg.header),
enabled: msg.enabled,
override_active: msg.override_active,
command_output_fault: msg.command_output_fault,
input_output_fault: msg.input_output_fault,
output_reported_fault: msg.output_reported_fault,
pacmod_fault: msg.pacmod_fault,
vehicle_fault: msg.vehicle_fault,
command_timeout: msg.command_timeout,
manual_input: msg.manual_input,
command: msg.command,
output: msg.output,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.enabled = self.enabled;
msg.override_active = self.override_active;
msg.command_output_fault = self.command_output_fault;
msg.input_output_fault = self.input_output_fault;
msg.output_reported_fault = self.output_reported_fault;
msg.pacmod_fault = self.pacmod_fault;
msg.vehicle_fault = self.vehicle_fault;
msg.command_timeout = self.command_timeout;
msg.manual_input = self.manual_input;
msg.command = self.command;
msg.output = self.output;
}



        }


                          
                          impl Default for SystemRptFloat {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SystemRptFloat>::new();
                                  SystemRptFloat::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SystemRptInt {

                              pub header: std_msgs::msg::Header,
pub enabled: bool,
pub override_active: bool,
pub command_output_fault: bool,
pub input_output_fault: bool,
pub output_reported_fault: bool,
pub pacmod_fault: bool,
pub vehicle_fault: bool,
pub command_timeout: bool,
pub manual_input: u16,
pub command: u16,
pub output: u16,

                          }

                          impl WrappedTypesupport for SystemRptInt { 

            type CStruct = pacmod3_msgs__msg__SystemRptInt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__SystemRptInt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__SystemRptInt {

                unsafe { pacmod3_msgs__msg__SystemRptInt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__SystemRptInt) -> () {

                unsafe { pacmod3_msgs__msg__SystemRptInt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SystemRptInt {
  SystemRptInt {
header: std_msgs::msg::Header::from_native(&msg.header),
enabled: msg.enabled,
override_active: msg.override_active,
command_output_fault: msg.command_output_fault,
input_output_fault: msg.input_output_fault,
output_reported_fault: msg.output_reported_fault,
pacmod_fault: msg.pacmod_fault,
vehicle_fault: msg.vehicle_fault,
command_timeout: msg.command_timeout,
manual_input: msg.manual_input,
command: msg.command,
output: msg.output,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.enabled = self.enabled;
msg.override_active = self.override_active;
msg.command_output_fault = self.command_output_fault;
msg.input_output_fault = self.input_output_fault;
msg.output_reported_fault = self.output_reported_fault;
msg.pacmod_fault = self.pacmod_fault;
msg.vehicle_fault = self.vehicle_fault;
msg.command_timeout = self.command_timeout;
msg.manual_input = self.manual_input;
msg.command = self.command;
msg.output = self.output;
}



        }


                          
                          impl Default for SystemRptInt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SystemRptInt>::new();
                                  SystemRptInt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct TirePressureRpt {

                              pub header: std_msgs::msg::Header,
pub front_left_tire_pressure: u8,
pub front_right_tire_pressure: u8,
pub rear_left_tire_pressure: u8,
pub rear_right_tire_pressure: u8,

                          }

                          impl WrappedTypesupport for TirePressureRpt { 

            type CStruct = pacmod3_msgs__msg__TirePressureRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__TirePressureRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__TirePressureRpt {

                unsafe { pacmod3_msgs__msg__TirePressureRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__TirePressureRpt) -> () {

                unsafe { pacmod3_msgs__msg__TirePressureRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> TirePressureRpt {
  TirePressureRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
front_left_tire_pressure: msg.front_left_tire_pressure,
front_right_tire_pressure: msg.front_right_tire_pressure,
rear_left_tire_pressure: msg.rear_left_tire_pressure,
rear_right_tire_pressure: msg.rear_right_tire_pressure,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.front_left_tire_pressure = self.front_left_tire_pressure;
msg.front_right_tire_pressure = self.front_right_tire_pressure;
msg.rear_left_tire_pressure = self.rear_left_tire_pressure;
msg.rear_right_tire_pressure = self.rear_right_tire_pressure;
}



        }


                          
                          impl Default for TirePressureRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<TirePressureRpt>::new();
                                  TirePressureRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct TurnAuxRpt {

                              pub header: std_msgs::msg::Header,
pub driver_blinker_bulb_on: bool,
pub passenger_blinker_bulb_on: bool,
pub driver_blinker_bulb_on_avail: bool,
pub passenger_blinker_bulb_on_avail: bool,

                          }

                          impl WrappedTypesupport for TurnAuxRpt { 

            type CStruct = pacmod3_msgs__msg__TurnAuxRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__TurnAuxRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__TurnAuxRpt {

                unsafe { pacmod3_msgs__msg__TurnAuxRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__TurnAuxRpt) -> () {

                unsafe { pacmod3_msgs__msg__TurnAuxRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> TurnAuxRpt {
  TurnAuxRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
driver_blinker_bulb_on: msg.driver_blinker_bulb_on,
passenger_blinker_bulb_on: msg.passenger_blinker_bulb_on,
driver_blinker_bulb_on_avail: msg.driver_blinker_bulb_on_avail,
passenger_blinker_bulb_on_avail: msg.passenger_blinker_bulb_on_avail,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.driver_blinker_bulb_on = self.driver_blinker_bulb_on;
msg.passenger_blinker_bulb_on = self.passenger_blinker_bulb_on;
msg.driver_blinker_bulb_on_avail = self.driver_blinker_bulb_on_avail;
msg.passenger_blinker_bulb_on_avail = self.passenger_blinker_bulb_on_avail;
}



        }


                          
                          impl Default for TurnAuxRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<TurnAuxRpt>::new();
                                  TurnAuxRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct VehicleDynamicsRpt {

                              pub header: std_msgs::msg::Header,
pub veh_g_forces: f64,

                          }

                          impl WrappedTypesupport for VehicleDynamicsRpt { 

            type CStruct = pacmod3_msgs__msg__VehicleDynamicsRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__VehicleDynamicsRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__VehicleDynamicsRpt {

                unsafe { pacmod3_msgs__msg__VehicleDynamicsRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__VehicleDynamicsRpt) -> () {

                unsafe { pacmod3_msgs__msg__VehicleDynamicsRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> VehicleDynamicsRpt {
  VehicleDynamicsRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
veh_g_forces: msg.veh_g_forces,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.veh_g_forces = self.veh_g_forces;
}



        }


                          
                          impl Default for VehicleDynamicsRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<VehicleDynamicsRpt>::new();
                                  VehicleDynamicsRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct VehicleSpeedRpt {

                              pub header: std_msgs::msg::Header,
pub vehicle_speed: f64,
pub vehicle_speed_valid: bool,

                          }

                          impl WrappedTypesupport for VehicleSpeedRpt { 

            type CStruct = pacmod3_msgs__msg__VehicleSpeedRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__VehicleSpeedRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__VehicleSpeedRpt {

                unsafe { pacmod3_msgs__msg__VehicleSpeedRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__VehicleSpeedRpt) -> () {

                unsafe { pacmod3_msgs__msg__VehicleSpeedRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> VehicleSpeedRpt {
  VehicleSpeedRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
vehicle_speed: msg.vehicle_speed,
vehicle_speed_valid: msg.vehicle_speed_valid,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.vehicle_speed = self.vehicle_speed;
msg.vehicle_speed_valid = self.vehicle_speed_valid;
}



        }


                          
                          impl Default for VehicleSpeedRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<VehicleSpeedRpt>::new();
                                  VehicleSpeedRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct VinRpt {

                              pub header: std_msgs::msg::Header,
pub mfg_code: std::string::String,
pub mfg: std::string::String,
pub model_year_code: std::string::String,
pub model_year: u32,
pub serial: u32,

                          }

                          impl WrappedTypesupport for VinRpt { 

            type CStruct = pacmod3_msgs__msg__VinRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__VinRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__VinRpt {

                unsafe { pacmod3_msgs__msg__VinRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__VinRpt) -> () {

                unsafe { pacmod3_msgs__msg__VinRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> VinRpt {
  VinRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
mfg_code: msg.mfg_code.to_str().to_owned(),
mfg: msg.mfg.to_str().to_owned(),
model_year_code: msg.model_year_code.to_str().to_owned(),
model_year: msg.model_year,
serial: msg.serial,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.mfg_code.assign(&self.mfg_code);
msg.mfg.assign(&self.mfg);
msg.model_year_code.assign(&self.model_year_code);
msg.model_year = self.model_year;
msg.serial = self.serial;
}



        }


                          
                          impl Default for VinRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<VinRpt>::new();
                                  VinRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct WatchdogRpt {

                              pub header: std_msgs::msg::Header,
pub global_enabled_flag: bool,
pub global_override_active: bool,
pub global_command_timeout_error: bool,
pub global_pacmod_subsystem_timeout: bool,
pub global_vehicle_can_timeout: bool,
pub global_pacmod_system_fault_active: bool,
pub global_config_fault_active: bool,
pub global_timeout: bool,
pub accel_enabled: bool,
pub accel_override_active: bool,
pub accel_command_output_fault: bool,
pub accel_input_output_fault: bool,
pub accel_output_reported_fault: bool,
pub accel_pacmod_fault: bool,
pub accel_vehicle_fault: bool,
pub accel_timeout: bool,
pub brake_enabled: bool,
pub brake_override_active: bool,
pub brake_command_output_fault: bool,
pub brake_input_output_fault: bool,
pub brake_output_reported_fault: bool,
pub brake_pacmod_fault: bool,
pub brake_vehicle_fault: bool,
pub brake_timeout: bool,
pub shift_enabled: bool,
pub shift_override_active: bool,
pub shift_command_output_fault: bool,
pub shift_input_output_fault: bool,
pub shift_output_reported_fault: bool,
pub shift_pacmod_fault: bool,
pub shift_vehicle_fault: bool,
pub shift_timeout: bool,
pub steer_enabled: bool,
pub steer_override_active: bool,
pub steer_command_output_fault: bool,
pub steer_input_output_fault: bool,
pub steer_output_reported_fault: bool,
pub steer_pacmod_fault: bool,
pub steer_vehicle_fault: bool,
pub steer_timeout: bool,
pub mod1_config_fault: bool,
pub mod1_can_timeout: bool,
pub mod1_counter_fault: bool,
pub mod2_config_fault: bool,
pub mod2_can_timeout: bool,
pub mod2_counter_fault: bool,
pub mod3_config_fault: bool,
pub mod3_can_timeout: bool,
pub mod3_counter_fault: bool,
pub mini1_rpt_timeout: bool,
pub mini1_config_fault: bool,
pub mini1_can_timeout: bool,
pub mini1_counter_fault: bool,
pub mini2_rpt_timeout: bool,
pub mini2_config_fault: bool,
pub mini2_can_timeout: bool,
pub mini2_counter_fault: bool,
pub mini3_rpt_timeout: bool,
pub mini3_config_fault: bool,
pub mini3_can_timeout: bool,
pub mini3_counter_fault: bool,
pub mod_system_present_fault: bool,
pub mini_system_present_fault: bool,
pub global_internal_power_supply_fault: bool,

                          }

                          impl WrappedTypesupport for WatchdogRpt { 

            type CStruct = pacmod3_msgs__msg__WatchdogRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__WatchdogRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__WatchdogRpt {

                unsafe { pacmod3_msgs__msg__WatchdogRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__WatchdogRpt) -> () {

                unsafe { pacmod3_msgs__msg__WatchdogRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> WatchdogRpt {
  WatchdogRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
global_enabled_flag: msg.global_enabled_flag,
global_override_active: msg.global_override_active,
global_command_timeout_error: msg.global_command_timeout_error,
global_pacmod_subsystem_timeout: msg.global_pacmod_subsystem_timeout,
global_vehicle_can_timeout: msg.global_vehicle_can_timeout,
global_pacmod_system_fault_active: msg.global_pacmod_system_fault_active,
global_config_fault_active: msg.global_config_fault_active,
global_timeout: msg.global_timeout,
accel_enabled: msg.accel_enabled,
accel_override_active: msg.accel_override_active,
accel_command_output_fault: msg.accel_command_output_fault,
accel_input_output_fault: msg.accel_input_output_fault,
accel_output_reported_fault: msg.accel_output_reported_fault,
accel_pacmod_fault: msg.accel_pacmod_fault,
accel_vehicle_fault: msg.accel_vehicle_fault,
accel_timeout: msg.accel_timeout,
brake_enabled: msg.brake_enabled,
brake_override_active: msg.brake_override_active,
brake_command_output_fault: msg.brake_command_output_fault,
brake_input_output_fault: msg.brake_input_output_fault,
brake_output_reported_fault: msg.brake_output_reported_fault,
brake_pacmod_fault: msg.brake_pacmod_fault,
brake_vehicle_fault: msg.brake_vehicle_fault,
brake_timeout: msg.brake_timeout,
shift_enabled: msg.shift_enabled,
shift_override_active: msg.shift_override_active,
shift_command_output_fault: msg.shift_command_output_fault,
shift_input_output_fault: msg.shift_input_output_fault,
shift_output_reported_fault: msg.shift_output_reported_fault,
shift_pacmod_fault: msg.shift_pacmod_fault,
shift_vehicle_fault: msg.shift_vehicle_fault,
shift_timeout: msg.shift_timeout,
steer_enabled: msg.steer_enabled,
steer_override_active: msg.steer_override_active,
steer_command_output_fault: msg.steer_command_output_fault,
steer_input_output_fault: msg.steer_input_output_fault,
steer_output_reported_fault: msg.steer_output_reported_fault,
steer_pacmod_fault: msg.steer_pacmod_fault,
steer_vehicle_fault: msg.steer_vehicle_fault,
steer_timeout: msg.steer_timeout,
mod1_config_fault: msg.mod1_config_fault,
mod1_can_timeout: msg.mod1_can_timeout,
mod1_counter_fault: msg.mod1_counter_fault,
mod2_config_fault: msg.mod2_config_fault,
mod2_can_timeout: msg.mod2_can_timeout,
mod2_counter_fault: msg.mod2_counter_fault,
mod3_config_fault: msg.mod3_config_fault,
mod3_can_timeout: msg.mod3_can_timeout,
mod3_counter_fault: msg.mod3_counter_fault,
mini1_rpt_timeout: msg.mini1_rpt_timeout,
mini1_config_fault: msg.mini1_config_fault,
mini1_can_timeout: msg.mini1_can_timeout,
mini1_counter_fault: msg.mini1_counter_fault,
mini2_rpt_timeout: msg.mini2_rpt_timeout,
mini2_config_fault: msg.mini2_config_fault,
mini2_can_timeout: msg.mini2_can_timeout,
mini2_counter_fault: msg.mini2_counter_fault,
mini3_rpt_timeout: msg.mini3_rpt_timeout,
mini3_config_fault: msg.mini3_config_fault,
mini3_can_timeout: msg.mini3_can_timeout,
mini3_counter_fault: msg.mini3_counter_fault,
mod_system_present_fault: msg.mod_system_present_fault,
mini_system_present_fault: msg.mini_system_present_fault,
global_internal_power_supply_fault: msg.global_internal_power_supply_fault,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.global_enabled_flag = self.global_enabled_flag;
msg.global_override_active = self.global_override_active;
msg.global_command_timeout_error = self.global_command_timeout_error;
msg.global_pacmod_subsystem_timeout = self.global_pacmod_subsystem_timeout;
msg.global_vehicle_can_timeout = self.global_vehicle_can_timeout;
msg.global_pacmod_system_fault_active = self.global_pacmod_system_fault_active;
msg.global_config_fault_active = self.global_config_fault_active;
msg.global_timeout = self.global_timeout;
msg.accel_enabled = self.accel_enabled;
msg.accel_override_active = self.accel_override_active;
msg.accel_command_output_fault = self.accel_command_output_fault;
msg.accel_input_output_fault = self.accel_input_output_fault;
msg.accel_output_reported_fault = self.accel_output_reported_fault;
msg.accel_pacmod_fault = self.accel_pacmod_fault;
msg.accel_vehicle_fault = self.accel_vehicle_fault;
msg.accel_timeout = self.accel_timeout;
msg.brake_enabled = self.brake_enabled;
msg.brake_override_active = self.brake_override_active;
msg.brake_command_output_fault = self.brake_command_output_fault;
msg.brake_input_output_fault = self.brake_input_output_fault;
msg.brake_output_reported_fault = self.brake_output_reported_fault;
msg.brake_pacmod_fault = self.brake_pacmod_fault;
msg.brake_vehicle_fault = self.brake_vehicle_fault;
msg.brake_timeout = self.brake_timeout;
msg.shift_enabled = self.shift_enabled;
msg.shift_override_active = self.shift_override_active;
msg.shift_command_output_fault = self.shift_command_output_fault;
msg.shift_input_output_fault = self.shift_input_output_fault;
msg.shift_output_reported_fault = self.shift_output_reported_fault;
msg.shift_pacmod_fault = self.shift_pacmod_fault;
msg.shift_vehicle_fault = self.shift_vehicle_fault;
msg.shift_timeout = self.shift_timeout;
msg.steer_enabled = self.steer_enabled;
msg.steer_override_active = self.steer_override_active;
msg.steer_command_output_fault = self.steer_command_output_fault;
msg.steer_input_output_fault = self.steer_input_output_fault;
msg.steer_output_reported_fault = self.steer_output_reported_fault;
msg.steer_pacmod_fault = self.steer_pacmod_fault;
msg.steer_vehicle_fault = self.steer_vehicle_fault;
msg.steer_timeout = self.steer_timeout;
msg.mod1_config_fault = self.mod1_config_fault;
msg.mod1_can_timeout = self.mod1_can_timeout;
msg.mod1_counter_fault = self.mod1_counter_fault;
msg.mod2_config_fault = self.mod2_config_fault;
msg.mod2_can_timeout = self.mod2_can_timeout;
msg.mod2_counter_fault = self.mod2_counter_fault;
msg.mod3_config_fault = self.mod3_config_fault;
msg.mod3_can_timeout = self.mod3_can_timeout;
msg.mod3_counter_fault = self.mod3_counter_fault;
msg.mini1_rpt_timeout = self.mini1_rpt_timeout;
msg.mini1_config_fault = self.mini1_config_fault;
msg.mini1_can_timeout = self.mini1_can_timeout;
msg.mini1_counter_fault = self.mini1_counter_fault;
msg.mini2_rpt_timeout = self.mini2_rpt_timeout;
msg.mini2_config_fault = self.mini2_config_fault;
msg.mini2_can_timeout = self.mini2_can_timeout;
msg.mini2_counter_fault = self.mini2_counter_fault;
msg.mini3_rpt_timeout = self.mini3_rpt_timeout;
msg.mini3_config_fault = self.mini3_config_fault;
msg.mini3_can_timeout = self.mini3_can_timeout;
msg.mini3_counter_fault = self.mini3_counter_fault;
msg.mod_system_present_fault = self.mod_system_present_fault;
msg.mini_system_present_fault = self.mini_system_present_fault;
msg.global_internal_power_supply_fault = self.global_internal_power_supply_fault;
}



        }


                          
                          impl Default for WatchdogRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<WatchdogRpt>::new();
                                  WatchdogRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct WatchdogRpt2 {

                              pub header: std_msgs::msg::Header,
pub accel_rpt_timeout: bool,
pub brake_rpt_timeout: bool,
pub brake_deccel_rpt_timeout: bool,
pub cabin_climate_rpt_timeout: bool,
pub cabin_fan_speed_rpt_timeout: bool,
pub cabin_temp_rpt_timeout: bool,
pub cruise_control_rpt_timeout: bool,
pub dash_left_rpt_timeout: bool,
pub dash_right_rpt_timeout: bool,
pub engine_brake_rpt_timeout: bool,
pub hazard_lights_rpt_timeout: bool,
pub headlight_rpt_timeout: bool,
pub horn_rpt_timeout: bool,
pub marker_lamp_rpt_timeout: bool,
pub media_controls_rpt_timeout: bool,
pub parking_brake_rpt_timeout: bool,
pub rear_pass_door_rpt_timeout: bool,
pub shift_rpt_timeout: bool,
pub sprayer_rpt_timeout: bool,
pub steering_rpt_timeout: bool,
pub turn_rpt_timeout: bool,
pub wiper_rpt_timeout: bool,
pub mod1_sanity_fault: bool,
pub mod2_sanity_fault: bool,
pub mod3_sanity_fault: bool,
pub mini1_sanity_fault: bool,
pub mini2_sanity_fault: bool,
pub mini3_sanity_fault: bool,
pub mod1_component_rpt_timeout: bool,
pub mod2_component_rpt_timeout: bool,
pub mod3_component_rpt_timeout: bool,
pub mini1_component_rpt_timeout: bool,
pub mini2_component_rpt_timeout: bool,
pub mini3_component_rpt_timeout: bool,
pub mod1_system_present_fault: bool,
pub mod2_system_present_fault: bool,
pub mod3_system_present_fault: bool,
pub mini1_system_present_fault: bool,
pub mini2_system_present_fault: bool,
pub mini3_system_present_fault: bool,
pub drive_mode_invalid: bool,

                          }

                          impl WrappedTypesupport for WatchdogRpt2 { 

            type CStruct = pacmod3_msgs__msg__WatchdogRpt2; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__WatchdogRpt2() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__WatchdogRpt2 {

                unsafe { pacmod3_msgs__msg__WatchdogRpt2__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__WatchdogRpt2) -> () {

                unsafe { pacmod3_msgs__msg__WatchdogRpt2__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> WatchdogRpt2 {
  WatchdogRpt2 {
header: std_msgs::msg::Header::from_native(&msg.header),
accel_rpt_timeout: msg.accel_rpt_timeout,
brake_rpt_timeout: msg.brake_rpt_timeout,
brake_deccel_rpt_timeout: msg.brake_deccel_rpt_timeout,
cabin_climate_rpt_timeout: msg.cabin_climate_rpt_timeout,
cabin_fan_speed_rpt_timeout: msg.cabin_fan_speed_rpt_timeout,
cabin_temp_rpt_timeout: msg.cabin_temp_rpt_timeout,
cruise_control_rpt_timeout: msg.cruise_control_rpt_timeout,
dash_left_rpt_timeout: msg.dash_left_rpt_timeout,
dash_right_rpt_timeout: msg.dash_right_rpt_timeout,
engine_brake_rpt_timeout: msg.engine_brake_rpt_timeout,
hazard_lights_rpt_timeout: msg.hazard_lights_rpt_timeout,
headlight_rpt_timeout: msg.headlight_rpt_timeout,
horn_rpt_timeout: msg.horn_rpt_timeout,
marker_lamp_rpt_timeout: msg.marker_lamp_rpt_timeout,
media_controls_rpt_timeout: msg.media_controls_rpt_timeout,
parking_brake_rpt_timeout: msg.parking_brake_rpt_timeout,
rear_pass_door_rpt_timeout: msg.rear_pass_door_rpt_timeout,
shift_rpt_timeout: msg.shift_rpt_timeout,
sprayer_rpt_timeout: msg.sprayer_rpt_timeout,
steering_rpt_timeout: msg.steering_rpt_timeout,
turn_rpt_timeout: msg.turn_rpt_timeout,
wiper_rpt_timeout: msg.wiper_rpt_timeout,
mod1_sanity_fault: msg.mod1_sanity_fault,
mod2_sanity_fault: msg.mod2_sanity_fault,
mod3_sanity_fault: msg.mod3_sanity_fault,
mini1_sanity_fault: msg.mini1_sanity_fault,
mini2_sanity_fault: msg.mini2_sanity_fault,
mini3_sanity_fault: msg.mini3_sanity_fault,
mod1_component_rpt_timeout: msg.mod1_component_rpt_timeout,
mod2_component_rpt_timeout: msg.mod2_component_rpt_timeout,
mod3_component_rpt_timeout: msg.mod3_component_rpt_timeout,
mini1_component_rpt_timeout: msg.mini1_component_rpt_timeout,
mini2_component_rpt_timeout: msg.mini2_component_rpt_timeout,
mini3_component_rpt_timeout: msg.mini3_component_rpt_timeout,
mod1_system_present_fault: msg.mod1_system_present_fault,
mod2_system_present_fault: msg.mod2_system_present_fault,
mod3_system_present_fault: msg.mod3_system_present_fault,
mini1_system_present_fault: msg.mini1_system_present_fault,
mini2_system_present_fault: msg.mini2_system_present_fault,
mini3_system_present_fault: msg.mini3_system_present_fault,
drive_mode_invalid: msg.drive_mode_invalid,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.accel_rpt_timeout = self.accel_rpt_timeout;
msg.brake_rpt_timeout = self.brake_rpt_timeout;
msg.brake_deccel_rpt_timeout = self.brake_deccel_rpt_timeout;
msg.cabin_climate_rpt_timeout = self.cabin_climate_rpt_timeout;
msg.cabin_fan_speed_rpt_timeout = self.cabin_fan_speed_rpt_timeout;
msg.cabin_temp_rpt_timeout = self.cabin_temp_rpt_timeout;
msg.cruise_control_rpt_timeout = self.cruise_control_rpt_timeout;
msg.dash_left_rpt_timeout = self.dash_left_rpt_timeout;
msg.dash_right_rpt_timeout = self.dash_right_rpt_timeout;
msg.engine_brake_rpt_timeout = self.engine_brake_rpt_timeout;
msg.hazard_lights_rpt_timeout = self.hazard_lights_rpt_timeout;
msg.headlight_rpt_timeout = self.headlight_rpt_timeout;
msg.horn_rpt_timeout = self.horn_rpt_timeout;
msg.marker_lamp_rpt_timeout = self.marker_lamp_rpt_timeout;
msg.media_controls_rpt_timeout = self.media_controls_rpt_timeout;
msg.parking_brake_rpt_timeout = self.parking_brake_rpt_timeout;
msg.rear_pass_door_rpt_timeout = self.rear_pass_door_rpt_timeout;
msg.shift_rpt_timeout = self.shift_rpt_timeout;
msg.sprayer_rpt_timeout = self.sprayer_rpt_timeout;
msg.steering_rpt_timeout = self.steering_rpt_timeout;
msg.turn_rpt_timeout = self.turn_rpt_timeout;
msg.wiper_rpt_timeout = self.wiper_rpt_timeout;
msg.mod1_sanity_fault = self.mod1_sanity_fault;
msg.mod2_sanity_fault = self.mod2_sanity_fault;
msg.mod3_sanity_fault = self.mod3_sanity_fault;
msg.mini1_sanity_fault = self.mini1_sanity_fault;
msg.mini2_sanity_fault = self.mini2_sanity_fault;
msg.mini3_sanity_fault = self.mini3_sanity_fault;
msg.mod1_component_rpt_timeout = self.mod1_component_rpt_timeout;
msg.mod2_component_rpt_timeout = self.mod2_component_rpt_timeout;
msg.mod3_component_rpt_timeout = self.mod3_component_rpt_timeout;
msg.mini1_component_rpt_timeout = self.mini1_component_rpt_timeout;
msg.mini2_component_rpt_timeout = self.mini2_component_rpt_timeout;
msg.mini3_component_rpt_timeout = self.mini3_component_rpt_timeout;
msg.mod1_system_present_fault = self.mod1_system_present_fault;
msg.mod2_system_present_fault = self.mod2_system_present_fault;
msg.mod3_system_present_fault = self.mod3_system_present_fault;
msg.mini1_system_present_fault = self.mini1_system_present_fault;
msg.mini2_system_present_fault = self.mini2_system_present_fault;
msg.mini3_system_present_fault = self.mini3_system_present_fault;
msg.drive_mode_invalid = self.drive_mode_invalid;
}



        }


                          
                          impl Default for WatchdogRpt2 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<WatchdogRpt2>::new();
                                  WatchdogRpt2::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct WheelSpeedRpt {

                              pub header: std_msgs::msg::Header,
pub front_left_wheel_speed: f64,
pub front_right_wheel_speed: f64,
pub rear_left_wheel_speed: f64,
pub rear_right_wheel_speed: f64,

                          }

                          impl WrappedTypesupport for WheelSpeedRpt { 

            type CStruct = pacmod3_msgs__msg__WheelSpeedRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__WheelSpeedRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__WheelSpeedRpt {

                unsafe { pacmod3_msgs__msg__WheelSpeedRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__WheelSpeedRpt) -> () {

                unsafe { pacmod3_msgs__msg__WheelSpeedRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> WheelSpeedRpt {
  WheelSpeedRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
front_left_wheel_speed: msg.front_left_wheel_speed,
front_right_wheel_speed: msg.front_right_wheel_speed,
rear_left_wheel_speed: msg.rear_left_wheel_speed,
rear_right_wheel_speed: msg.rear_right_wheel_speed,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.front_left_wheel_speed = self.front_left_wheel_speed;
msg.front_right_wheel_speed = self.front_right_wheel_speed;
msg.rear_left_wheel_speed = self.rear_left_wheel_speed;
msg.rear_right_wheel_speed = self.rear_right_wheel_speed;
}



        }


                          
                          impl Default for WheelSpeedRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<WheelSpeedRpt>::new();
                                  WheelSpeedRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct WiperAuxRpt {

                              pub header: std_msgs::msg::Header,
pub front_wiping: bool,
pub front_spraying: bool,
pub rear_wiping: bool,
pub rear_spraying: bool,
pub spray_near_empty: bool,
pub spray_empty: bool,
pub front_wiping_avail: bool,
pub front_spraying_avail: bool,
pub rear_wiping_avail: bool,
pub rear_spraying_avail: bool,
pub spray_near_empty_avail: bool,
pub spray_empty_avail: bool,

                          }

                          impl WrappedTypesupport for WiperAuxRpt { 

            type CStruct = pacmod3_msgs__msg__WiperAuxRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__WiperAuxRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__WiperAuxRpt {

                unsafe { pacmod3_msgs__msg__WiperAuxRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__WiperAuxRpt) -> () {

                unsafe { pacmod3_msgs__msg__WiperAuxRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> WiperAuxRpt {
  WiperAuxRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
front_wiping: msg.front_wiping,
front_spraying: msg.front_spraying,
rear_wiping: msg.rear_wiping,
rear_spraying: msg.rear_spraying,
spray_near_empty: msg.spray_near_empty,
spray_empty: msg.spray_empty,
front_wiping_avail: msg.front_wiping_avail,
front_spraying_avail: msg.front_spraying_avail,
rear_wiping_avail: msg.rear_wiping_avail,
rear_spraying_avail: msg.rear_spraying_avail,
spray_near_empty_avail: msg.spray_near_empty_avail,
spray_empty_avail: msg.spray_empty_avail,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.front_wiping = self.front_wiping;
msg.front_spraying = self.front_spraying;
msg.rear_wiping = self.rear_wiping;
msg.rear_spraying = self.rear_spraying;
msg.spray_near_empty = self.spray_near_empty;
msg.spray_empty = self.spray_empty;
msg.front_wiping_avail = self.front_wiping_avail;
msg.front_spraying_avail = self.front_spraying_avail;
msg.rear_wiping_avail = self.rear_wiping_avail;
msg.rear_spraying_avail = self.rear_spraying_avail;
msg.spray_near_empty_avail = self.spray_near_empty_avail;
msg.spray_empty_avail = self.spray_empty_avail;
}



        }


                          
                          impl Default for WiperAuxRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<WiperAuxRpt>::new();
                                  WiperAuxRpt::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct YawRateRpt {

                              pub header: std_msgs::msg::Header,
pub yaw_rate: f64,

                          }

                          impl WrappedTypesupport for YawRateRpt { 

            type CStruct = pacmod3_msgs__msg__YawRateRpt; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pacmod3_msgs__msg__YawRateRpt() }
            }

            fn create_msg() -> *mut pacmod3_msgs__msg__YawRateRpt {

                unsafe { pacmod3_msgs__msg__YawRateRpt__create() }

            }

            fn destroy_msg(msg: *mut pacmod3_msgs__msg__YawRateRpt) -> () {

                unsafe { pacmod3_msgs__msg__YawRateRpt__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> YawRateRpt {
  YawRateRpt {
header: std_msgs::msg::Header::from_native(&msg.header),
yaw_rate: msg.yaw_rate,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.yaw_rate = self.yaw_rate;
}



        }


                          
                          impl Default for YawRateRpt {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<YawRateRpt>::new();
                                  YawRateRpt::from_native(&msg_native)
                              }
                          }
             


                      }
