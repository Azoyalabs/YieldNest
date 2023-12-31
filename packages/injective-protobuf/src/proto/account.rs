// This file is generated by rust-protobuf 2.28.0. Do not edit
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
//! Generated file from `injective/types/v1beta1/account.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default,Debug)]
pub struct EthAccount {
    // message fields
    pub base_account: ::protobuf::SingularPtrField<super::auth::BaseAccount>,
    pub code_hash: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EthAccount {
    fn default() -> &'a EthAccount {
        <EthAccount as ::protobuf::Message>::default_instance()
    }
}

impl EthAccount {
    pub fn new() -> EthAccount {
        ::std::default::Default::default()
    }

    // .cosmos.auth.v1beta1.BaseAccount base_account = 1;


    pub fn get_base_account(&self) -> &super::auth::BaseAccount {
        self.base_account.as_ref().unwrap_or_else(|| <super::auth::BaseAccount as ::protobuf::Message>::default_instance())
    }
    pub fn clear_base_account(&mut self) {
        self.base_account.clear();
    }

    pub fn has_base_account(&self) -> bool {
        self.base_account.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_account(&mut self, v: super::auth::BaseAccount) {
        self.base_account = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_base_account(&mut self) -> &mut super::auth::BaseAccount {
        if self.base_account.is_none() {
            self.base_account.set_default();
        }
        self.base_account.as_mut().unwrap()
    }

    // Take field
    pub fn take_base_account(&mut self) -> super::auth::BaseAccount {
        self.base_account.take().unwrap_or_else(|| super::auth::BaseAccount::new())
    }

    // bytes code_hash = 2;


    pub fn get_code_hash(&self) -> &[u8] {
        &self.code_hash
    }
    pub fn clear_code_hash(&mut self) {
        self.code_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_code_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.code_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_code_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.code_hash
    }

    // Take field
    pub fn take_code_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.code_hash, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for EthAccount {
    fn is_initialized(&self) -> bool {
        for v in &self.base_account {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.base_account)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.code_hash)?;
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
        if let Some(ref v) = self.base_account.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.code_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.code_hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.base_account.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.code_hash.is_empty() {
            os.write_bytes(2, &self.code_hash)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> EthAccount {
        EthAccount::new()
    }

    fn default_instance() -> &'static EthAccount {
        static instance: ::protobuf::rt::LazyV2<EthAccount> = ::protobuf::rt::LazyV2::INIT;
        instance.get(EthAccount::new)
    }
}

impl ::protobuf::Clear for EthAccount {
    fn clear(&mut self) {
        self.base_account.clear();
        self.code_hash.clear();
        self.unknown_fields.clear();
    }
}

impl ::protobuf::reflect::ProtobufValue for EthAccount {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}
