  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Duration {

                              pub sec: i32,
pub nanosec: u32,

                          }

                          impl WrappedTypesupport for Duration { 

            type CStruct = builtin_interfaces__msg__Duration; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__builtin_interfaces__msg__Duration() }
            }

            fn create_msg() -> *mut builtin_interfaces__msg__Duration {

                unsafe { builtin_interfaces__msg__Duration__create() }

            }

            fn destroy_msg(msg: *mut builtin_interfaces__msg__Duration) -> () {

                unsafe { builtin_interfaces__msg__Duration__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Duration {
  Duration {
sec: msg.sec,
nanosec: msg.nanosec,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.sec = self.sec;
msg.nanosec = self.nanosec;
}



        }


                          
                          impl Default for Duration {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Duration>::new();
                                  Duration::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Time {

                              pub sec: i32,
pub nanosec: u32,

                          }

                          impl WrappedTypesupport for Time { 

            type CStruct = builtin_interfaces__msg__Time; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__builtin_interfaces__msg__Time() }
            }

            fn create_msg() -> *mut builtin_interfaces__msg__Time {

                unsafe { builtin_interfaces__msg__Time__create() }

            }

            fn destroy_msg(msg: *mut builtin_interfaces__msg__Time) -> () {

                unsafe { builtin_interfaces__msg__Time__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Time {
  Time {
sec: msg.sec,
nanosec: msg.nanosec,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.sec = self.sec;
msg.nanosec = self.nanosec;
}



        }


                          
                          impl Default for Time {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Time>::new();
                                  Time::from_native(&msg_native)
                              }
                          }
             


                      }
