  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Packet {

                              pub header: std_msgs::msg::Header,
pub data: Vec<u8>,
pub b_o_s: i32,
pub e_o_s: i32,
pub granulepos: i64,
pub packetno: i64,

                          }

                          impl WrappedTypesupport for Packet { 

            type CStruct = theora_image_transport__msg__Packet; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__theora_image_transport__msg__Packet() }
            }

            fn create_msg() -> *mut theora_image_transport__msg__Packet {

                unsafe { theora_image_transport__msg__Packet__create() }

            }

            fn destroy_msg(msg: *mut theora_image_transport__msg__Packet) -> () {

                unsafe { theora_image_transport__msg__Packet__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Packet {
  Packet {
header: std_msgs::msg::Header::from_native(&msg.header),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
b_o_s: msg.b_o_s,
e_o_s: msg.e_o_s,
granulepos: msg.granulepos,
packetno: msg.packetno,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.data.update(&self.data);
msg.b_o_s = self.b_o_s;
msg.e_o_s = self.e_o_s;
msg.granulepos = self.granulepos;
msg.packetno = self.packetno;
}



        }


                          
                          impl Default for Packet {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Packet>::new();
                                  Packet::from_native(&msg_native)
                              }
                          }
             


                      }
