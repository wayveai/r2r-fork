  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Gid {

                              pub data: Vec<u8>,

                          }

                          impl WrappedTypesupport for Gid { 

            type CStruct = rmw_dds_common__msg__Gid; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rmw_dds_common__msg__Gid() }
            }

            fn create_msg() -> *mut rmw_dds_common__msg__Gid {

                unsafe { rmw_dds_common__msg__Gid__create() }

            }

            fn destroy_msg(msg: *mut rmw_dds_common__msg__Gid) -> () {

                unsafe { rmw_dds_common__msg__Gid__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Gid {
  Gid {
// is_upper_bound_: false
// member.array_size_ : 24
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {assert_eq!(self.data.len(), 24, "Field {} is fixed size of {}!", "data", 24);
msg.data.copy_from_slice(&self.data[..24]);
}



        }


                          
                          impl Default for Gid {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Gid>::new();
                                  Gid::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct NodeEntitiesInfo {

                              pub node_namespace: std::string::String,
pub node_name: std::string::String,
pub reader_gid_seq: Vec<rmw_dds_common::msg::Gid>,
pub writer_gid_seq: Vec<rmw_dds_common::msg::Gid>,

                          }

                          impl WrappedTypesupport for NodeEntitiesInfo { 

            type CStruct = rmw_dds_common__msg__NodeEntitiesInfo; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rmw_dds_common__msg__NodeEntitiesInfo() }
            }

            fn create_msg() -> *mut rmw_dds_common__msg__NodeEntitiesInfo {

                unsafe { rmw_dds_common__msg__NodeEntitiesInfo__create() }

            }

            fn destroy_msg(msg: *mut rmw_dds_common__msg__NodeEntitiesInfo) -> () {

                unsafe { rmw_dds_common__msg__NodeEntitiesInfo__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> NodeEntitiesInfo {
  NodeEntitiesInfo {
node_namespace: msg.node_namespace.to_str().to_owned(),
node_name: msg.node_name.to_str().to_owned(),
// is_upper_bound_: false
// member.array_size_ : 0
reader_gid_seq : {
let mut temp = Vec::with_capacity(msg.reader_gid_seq.size);
let slice = unsafe { std::slice::from_raw_parts(msg.reader_gid_seq.data, msg.reader_gid_seq.size)};
for s in slice { temp.push(rmw_dds_common::msg::Gid::from_native(s)); }
temp },
// is_upper_bound_: false
// member.array_size_ : 0
writer_gid_seq : {
let mut temp = Vec::with_capacity(msg.writer_gid_seq.size);
let slice = unsafe { std::slice::from_raw_parts(msg.writer_gid_seq.data, msg.writer_gid_seq.size)};
for s in slice { temp.push(rmw_dds_common::msg::Gid::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.node_namespace.assign(&self.node_namespace);
msg.node_name.assign(&self.node_name);
unsafe { rmw_dds_common__msg__Gid__Sequence__fini(&mut msg.reader_gid_seq) };
unsafe { rmw_dds_common__msg__Gid__Sequence__init(&mut msg.reader_gid_seq, self.reader_gid_seq.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.reader_gid_seq.data, msg.reader_gid_seq.size)};
for (t,s) in slice.iter_mut().zip(&self.reader_gid_seq) { s.copy_to_native(t);}
unsafe { rmw_dds_common__msg__Gid__Sequence__fini(&mut msg.writer_gid_seq) };
unsafe { rmw_dds_common__msg__Gid__Sequence__init(&mut msg.writer_gid_seq, self.writer_gid_seq.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.writer_gid_seq.data, msg.writer_gid_seq.size)};
for (t,s) in slice.iter_mut().zip(&self.writer_gid_seq) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for NodeEntitiesInfo {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<NodeEntitiesInfo>::new();
                                  NodeEntitiesInfo::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct ParticipantEntitiesInfo {

                              pub gid: rmw_dds_common::msg::Gid,
pub node_entities_info_seq: Vec<rmw_dds_common::msg::NodeEntitiesInfo>,

                          }

                          impl WrappedTypesupport for ParticipantEntitiesInfo { 

            type CStruct = rmw_dds_common__msg__ParticipantEntitiesInfo; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rmw_dds_common__msg__ParticipantEntitiesInfo() }
            }

            fn create_msg() -> *mut rmw_dds_common__msg__ParticipantEntitiesInfo {

                unsafe { rmw_dds_common__msg__ParticipantEntitiesInfo__create() }

            }

            fn destroy_msg(msg: *mut rmw_dds_common__msg__ParticipantEntitiesInfo) -> () {

                unsafe { rmw_dds_common__msg__ParticipantEntitiesInfo__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> ParticipantEntitiesInfo {
  ParticipantEntitiesInfo {
gid: rmw_dds_common::msg::Gid::from_native(&msg.gid),
// is_upper_bound_: false
// member.array_size_ : 0
node_entities_info_seq : {
let mut temp = Vec::with_capacity(msg.node_entities_info_seq.size);
let slice = unsafe { std::slice::from_raw_parts(msg.node_entities_info_seq.data, msg.node_entities_info_seq.size)};
for s in slice { temp.push(rmw_dds_common::msg::NodeEntitiesInfo::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.gid.copy_to_native(&mut msg.gid);
unsafe { rmw_dds_common__msg__NodeEntitiesInfo__Sequence__fini(&mut msg.node_entities_info_seq) };
unsafe { rmw_dds_common__msg__NodeEntitiesInfo__Sequence__init(&mut msg.node_entities_info_seq, self.node_entities_info_seq.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.node_entities_info_seq.data, msg.node_entities_info_seq.size)};
for (t,s) in slice.iter_mut().zip(&self.node_entities_info_seq) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for ParticipantEntitiesInfo {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<ParticipantEntitiesInfo>::new();
                                  ParticipantEntitiesInfo::from_native(&msg_native)
                              }
                          }
             


                      }
