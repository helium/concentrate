// This file is generated by rust-protobuf 2.6.2. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct LongFiRxPacket {
    // message fields
    pub crc_check: bool,
    pub timestamp: u64,
    pub rssi: f32,
    pub snr: f32,
    pub oui: u32,
    pub device_id: u32,
    pub mac: u32,
    pub payload: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a LongFiRxPacket {
    fn default() -> &'a LongFiRxPacket {
        <LongFiRxPacket as ::protobuf::Message>::default_instance()
    }
}

impl LongFiRxPacket {
    pub fn new() -> LongFiRxPacket {
        ::std::default::Default::default()
    }

    // bool crc_check = 1;


    pub fn get_crc_check(&self) -> bool {
        self.crc_check
    }
    pub fn clear_crc_check(&mut self) {
        self.crc_check = false;
    }

    // Param is passed by value, moved
    pub fn set_crc_check(&mut self, v: bool) {
        self.crc_check = v;
    }

    // uint64 timestamp = 2;


    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = v;
    }

    // float rssi = 3;


    pub fn get_rssi(&self) -> f32 {
        self.rssi
    }
    pub fn clear_rssi(&mut self) {
        self.rssi = 0.;
    }

    // Param is passed by value, moved
    pub fn set_rssi(&mut self, v: f32) {
        self.rssi = v;
    }

    // float snr = 4;


    pub fn get_snr(&self) -> f32 {
        self.snr
    }
    pub fn clear_snr(&mut self) {
        self.snr = 0.;
    }

    // Param is passed by value, moved
    pub fn set_snr(&mut self, v: f32) {
        self.snr = v;
    }

    // uint32 oui = 5;


    pub fn get_oui(&self) -> u32 {
        self.oui
    }
    pub fn clear_oui(&mut self) {
        self.oui = 0;
    }

    // Param is passed by value, moved
    pub fn set_oui(&mut self, v: u32) {
        self.oui = v;
    }

    // uint32 device_id = 6;


    pub fn get_device_id(&self) -> u32 {
        self.device_id
    }
    pub fn clear_device_id(&mut self) {
        self.device_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_device_id(&mut self, v: u32) {
        self.device_id = v;
    }

    // uint32 mac = 7;


    pub fn get_mac(&self) -> u32 {
        self.mac
    }
    pub fn clear_mac(&mut self) {
        self.mac = 0;
    }

    // Param is passed by value, moved
    pub fn set_mac(&mut self, v: u32) {
        self.mac = v;
    }

    // bytes payload = 8;


    pub fn get_payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.payload
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.payload, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for LongFiRxPacket {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.crc_check = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.rssi = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.snr = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.oui = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.device_id = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.mac = tmp;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.payload)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.crc_check != false {
            my_size += 2;
        }
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(2, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.rssi != 0. {
            my_size += 5;
        }
        if self.snr != 0. {
            my_size += 5;
        }
        if self.oui != 0 {
            my_size += ::protobuf::rt::value_size(5, self.oui, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.device_id != 0 {
            my_size += ::protobuf::rt::value_size(6, self.device_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.mac != 0 {
            my_size += ::protobuf::rt::value_size(7, self.mac, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.payload.is_empty() {
            my_size += ::protobuf::rt::bytes_size(8, &self.payload);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.crc_check != false {
            os.write_bool(1, self.crc_check)?;
        }
        if self.timestamp != 0 {
            os.write_uint64(2, self.timestamp)?;
        }
        if self.rssi != 0. {
            os.write_float(3, self.rssi)?;
        }
        if self.snr != 0. {
            os.write_float(4, self.snr)?;
        }
        if self.oui != 0 {
            os.write_uint32(5, self.oui)?;
        }
        if self.device_id != 0 {
            os.write_uint32(6, self.device_id)?;
        }
        if self.mac != 0 {
            os.write_uint32(7, self.mac)?;
        }
        if !self.payload.is_empty() {
            os.write_bytes(8, &self.payload)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> LongFiRxPacket {
        LongFiRxPacket::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "crc_check",
                    |m: &LongFiRxPacket| { &m.crc_check },
                    |m: &mut LongFiRxPacket| { &mut m.crc_check },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    |m: &LongFiRxPacket| { &m.timestamp },
                    |m: &mut LongFiRxPacket| { &mut m.timestamp },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "rssi",
                    |m: &LongFiRxPacket| { &m.rssi },
                    |m: &mut LongFiRxPacket| { &mut m.rssi },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "snr",
                    |m: &LongFiRxPacket| { &m.snr },
                    |m: &mut LongFiRxPacket| { &mut m.snr },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "oui",
                    |m: &LongFiRxPacket| { &m.oui },
                    |m: &mut LongFiRxPacket| { &mut m.oui },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "device_id",
                    |m: &LongFiRxPacket| { &m.device_id },
                    |m: &mut LongFiRxPacket| { &mut m.device_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "mac",
                    |m: &LongFiRxPacket| { &m.mac },
                    |m: &mut LongFiRxPacket| { &mut m.mac },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "payload",
                    |m: &LongFiRxPacket| { &m.payload },
                    |m: &mut LongFiRxPacket| { &mut m.payload },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LongFiRxPacket>(
                    "LongFiRxPacket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static LongFiRxPacket {
        static mut instance: ::protobuf::lazy::Lazy<LongFiRxPacket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LongFiRxPacket,
        };
        unsafe {
            instance.get(LongFiRxPacket::new)
        }
    }
}

impl ::protobuf::Clear for LongFiRxPacket {
    fn clear(&mut self) {
        self.crc_check = false;
        self.timestamp = 0;
        self.rssi = 0.;
        self.snr = 0.;
        self.oui = 0;
        self.device_id = 0;
        self.mac = 0;
        self.payload.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LongFiRxPacket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LongFiRxPacket {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LongFiTxUplinkPacket {
    // message fields
    pub disable_encoding: bool,
    pub disable_fragmentation: bool,
    pub oui: u32,
    pub device_id: u32,
    pub payload: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a LongFiTxUplinkPacket {
    fn default() -> &'a LongFiTxUplinkPacket {
        <LongFiTxUplinkPacket as ::protobuf::Message>::default_instance()
    }
}

impl LongFiTxUplinkPacket {
    pub fn new() -> LongFiTxUplinkPacket {
        ::std::default::Default::default()
    }

    // bool disable_encoding = 1;


    pub fn get_disable_encoding(&self) -> bool {
        self.disable_encoding
    }
    pub fn clear_disable_encoding(&mut self) {
        self.disable_encoding = false;
    }

    // Param is passed by value, moved
    pub fn set_disable_encoding(&mut self, v: bool) {
        self.disable_encoding = v;
    }

    // bool disable_fragmentation = 2;


    pub fn get_disable_fragmentation(&self) -> bool {
        self.disable_fragmentation
    }
    pub fn clear_disable_fragmentation(&mut self) {
        self.disable_fragmentation = false;
    }

    // Param is passed by value, moved
    pub fn set_disable_fragmentation(&mut self, v: bool) {
        self.disable_fragmentation = v;
    }

    // uint32 oui = 3;


    pub fn get_oui(&self) -> u32 {
        self.oui
    }
    pub fn clear_oui(&mut self) {
        self.oui = 0;
    }

    // Param is passed by value, moved
    pub fn set_oui(&mut self, v: u32) {
        self.oui = v;
    }

    // uint32 device_id = 4;


    pub fn get_device_id(&self) -> u32 {
        self.device_id
    }
    pub fn clear_device_id(&mut self) {
        self.device_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_device_id(&mut self, v: u32) {
        self.device_id = v;
    }

    // bytes payload = 5;


    pub fn get_payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.payload
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.payload, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for LongFiTxUplinkPacket {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.disable_encoding = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.disable_fragmentation = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.oui = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.device_id = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.payload)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.disable_encoding != false {
            my_size += 2;
        }
        if self.disable_fragmentation != false {
            my_size += 2;
        }
        if self.oui != 0 {
            my_size += ::protobuf::rt::value_size(3, self.oui, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.device_id != 0 {
            my_size += ::protobuf::rt::value_size(4, self.device_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.payload.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.payload);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.disable_encoding != false {
            os.write_bool(1, self.disable_encoding)?;
        }
        if self.disable_fragmentation != false {
            os.write_bool(2, self.disable_fragmentation)?;
        }
        if self.oui != 0 {
            os.write_uint32(3, self.oui)?;
        }
        if self.device_id != 0 {
            os.write_uint32(4, self.device_id)?;
        }
        if !self.payload.is_empty() {
            os.write_bytes(5, &self.payload)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> LongFiTxUplinkPacket {
        LongFiTxUplinkPacket::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "disable_encoding",
                    |m: &LongFiTxUplinkPacket| { &m.disable_encoding },
                    |m: &mut LongFiTxUplinkPacket| { &mut m.disable_encoding },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "disable_fragmentation",
                    |m: &LongFiTxUplinkPacket| { &m.disable_fragmentation },
                    |m: &mut LongFiTxUplinkPacket| { &mut m.disable_fragmentation },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "oui",
                    |m: &LongFiTxUplinkPacket| { &m.oui },
                    |m: &mut LongFiTxUplinkPacket| { &mut m.oui },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "device_id",
                    |m: &LongFiTxUplinkPacket| { &m.device_id },
                    |m: &mut LongFiTxUplinkPacket| { &mut m.device_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "payload",
                    |m: &LongFiTxUplinkPacket| { &m.payload },
                    |m: &mut LongFiTxUplinkPacket| { &mut m.payload },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LongFiTxUplinkPacket>(
                    "LongFiTxUplinkPacket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static LongFiTxUplinkPacket {
        static mut instance: ::protobuf::lazy::Lazy<LongFiTxUplinkPacket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LongFiTxUplinkPacket,
        };
        unsafe {
            instance.get(LongFiTxUplinkPacket::new)
        }
    }
}

impl ::protobuf::Clear for LongFiTxUplinkPacket {
    fn clear(&mut self) {
        self.disable_encoding = false;
        self.disable_fragmentation = false;
        self.oui = 0;
        self.device_id = 0;
        self.payload.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LongFiTxUplinkPacket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LongFiTxUplinkPacket {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10src/longfi.proto\x12\rhelium.longfi\"\xa1\x01\n\x0eLongFiRxPacket\
    \x12\x13\n\tcrc_check\x18\x01\x20\x01(\x08B\0\x12\x13\n\ttimestamp\x18\
    \x02\x20\x01(\x04B\0\x12\x0e\n\x04rssi\x18\x03\x20\x01(\x02B\0\x12\r\n\
    \x03snr\x18\x04\x20\x01(\x02B\0\x12\r\n\x03oui\x18\x05\x20\x01(\rB\0\x12\
    \x13\n\tdevice_id\x18\x06\x20\x01(\rB\0\x12\r\n\x03mac\x18\x07\x20\x01(\
    \rB\0\x12\x11\n\x07payload\x18\x08\x20\x01(\x0cB\0:\0\"\x8c\x01\n\x14Lon\
    gFiTxUplinkPacket\x12\x1a\n\x10disable_encoding\x18\x01\x20\x01(\x08B\0\
    \x12\x1f\n\x15disable_fragmentation\x18\x02\x20\x01(\x08B\0\x12\r\n\x03o\
    ui\x18\x03\x20\x01(\rB\0\x12\x13\n\tdevice_id\x18\x04\x20\x01(\rB\0\x12\
    \x11\n\x07payload\x18\x05\x20\x01(\x0cB\0:\0B\0b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
