// This file is generated by rust-protobuf 2.22.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `event.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum WorkspaceEvent {
    CreateWorkspace = 0,
    GetCurWorkspace = 1,
    GetWorkspace = 2,
    CreateApp = 101,
    GetApp = 102,
    CreateView = 201,
    ReadView = 202,
    UpdateView = 203,
}

impl ::protobuf::ProtobufEnum for WorkspaceEvent {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<WorkspaceEvent> {
        match value {
            0 => ::std::option::Option::Some(WorkspaceEvent::CreateWorkspace),
            1 => ::std::option::Option::Some(WorkspaceEvent::GetCurWorkspace),
            2 => ::std::option::Option::Some(WorkspaceEvent::GetWorkspace),
            101 => ::std::option::Option::Some(WorkspaceEvent::CreateApp),
            102 => ::std::option::Option::Some(WorkspaceEvent::GetApp),
            201 => ::std::option::Option::Some(WorkspaceEvent::CreateView),
            202 => ::std::option::Option::Some(WorkspaceEvent::ReadView),
            203 => ::std::option::Option::Some(WorkspaceEvent::UpdateView),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [WorkspaceEvent] = &[
            WorkspaceEvent::CreateWorkspace,
            WorkspaceEvent::GetCurWorkspace,
            WorkspaceEvent::GetWorkspace,
            WorkspaceEvent::CreateApp,
            WorkspaceEvent::GetApp,
            WorkspaceEvent::CreateView,
            WorkspaceEvent::ReadView,
            WorkspaceEvent::UpdateView,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<WorkspaceEvent>("WorkspaceEvent", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for WorkspaceEvent {
}

impl ::std::default::Default for WorkspaceEvent {
    fn default() -> Self {
        WorkspaceEvent::CreateWorkspace
    }
}

impl ::protobuf::reflect::ProtobufValue for WorkspaceEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bevent.proto*\x98\x01\n\x0eWorkspaceEvent\x12\x13\n\x0fCreateWorksp\
    ace\x10\0\x12\x13\n\x0fGetCurWorkspace\x10\x01\x12\x10\n\x0cGetWorkspace\
    \x10\x02\x12\r\n\tCreateApp\x10e\x12\n\n\x06GetApp\x10f\x12\x0f\n\nCreat\
    eView\x10\xc9\x01\x12\r\n\x08ReadView\x10\xca\x01\x12\x0f\n\nUpdateView\
    \x10\xcb\x01J\xf2\x02\n\x06\x12\x04\0\0\x0b\x01\n\x08\n\x01\x0c\x12\x03\
    \0\0\x12\n\n\n\x02\x05\0\x12\x04\x02\0\x0b\x01\n\n\n\x03\x05\0\x01\x12\
    \x03\x02\x05\x13\n\x0b\n\x04\x05\0\x02\0\x12\x03\x03\x04\x18\n\x0c\n\x05\
    \x05\0\x02\0\x01\x12\x03\x03\x04\x13\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\
    \x03\x16\x17\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x04\x04\x18\n\x0c\n\x05\
    \x05\0\x02\x01\x01\x12\x03\x04\x04\x13\n\x0c\n\x05\x05\0\x02\x01\x02\x12\
    \x03\x04\x16\x17\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x05\x04\x15\n\x0c\n\
    \x05\x05\0\x02\x02\x01\x12\x03\x05\x04\x10\n\x0c\n\x05\x05\0\x02\x02\x02\
    \x12\x03\x05\x13\x14\n\x0b\n\x04\x05\0\x02\x03\x12\x03\x06\x04\x14\n\x0c\
    \n\x05\x05\0\x02\x03\x01\x12\x03\x06\x04\r\n\x0c\n\x05\x05\0\x02\x03\x02\
    \x12\x03\x06\x10\x13\n\x0b\n\x04\x05\0\x02\x04\x12\x03\x07\x04\x11\n\x0c\
    \n\x05\x05\0\x02\x04\x01\x12\x03\x07\x04\n\n\x0c\n\x05\x05\0\x02\x04\x02\
    \x12\x03\x07\r\x10\n\x0b\n\x04\x05\0\x02\x05\x12\x03\x08\x04\x15\n\x0c\n\
    \x05\x05\0\x02\x05\x01\x12\x03\x08\x04\x0e\n\x0c\n\x05\x05\0\x02\x05\x02\
    \x12\x03\x08\x11\x14\n\x0b\n\x04\x05\0\x02\x06\x12\x03\t\x04\x13\n\x0c\n\
    \x05\x05\0\x02\x06\x01\x12\x03\t\x04\x0c\n\x0c\n\x05\x05\0\x02\x06\x02\
    \x12\x03\t\x0f\x12\n\x0b\n\x04\x05\0\x02\x07\x12\x03\n\x04\x15\n\x0c\n\
    \x05\x05\0\x02\x07\x01\x12\x03\n\x04\x0e\n\x0c\n\x05\x05\0\x02\x07\x02\
    \x12\x03\n\x11\x14b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
