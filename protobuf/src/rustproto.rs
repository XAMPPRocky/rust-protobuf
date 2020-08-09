// This file is generated by rust-protobuf 3.0.0-pre. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `rustproto.proto`

/// Extension fields
pub mod exts {

    pub const expose_oneof_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17001, phantom: ::std::marker::PhantomData };

    pub const expose_fields_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const generate_getter_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17005, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const serde_derive_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17030, phantom: ::std::marker::PhantomData };

    pub const serde_derive_cfg_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeString> = crate::ext::ExtFieldOptional { field_number: 17031, phantom: ::std::marker::PhantomData };

    pub const lite_runtime_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17035, phantom: ::std::marker::PhantomData };

    pub const expose_oneof: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17001, phantom: ::std::marker::PhantomData };

    pub const expose_fields: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const generate_getter: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17005, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const serde_derive: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17030, phantom: ::std::marker::PhantomData };

    pub const serde_derive_cfg: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeString> = crate::ext::ExtFieldOptional { field_number: 17031, phantom: ::std::marker::PhantomData };

    pub const expose_fields_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const generate_getter_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17005, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0frustproto.proto\x12\trustproto\x1a\x20google/protobuf/descriptor.p\
    roto:H\n\x10expose_oneof_all\x18\xe9\x84\x01\x20\x01(\x08\x12\x1c.google\
    .protobuf.FileOptionsR\x0eexposeOneofAll:J\n\x11expose_fields_all\x18\
    \xeb\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x0fexpose\
    FieldsAll:T\n\x16generate_accessors_all\x18\xec\x84\x01\x20\x01(\x08\x12\
    \x1c.google.protobuf.FileOptionsR\x14generateAccessorsAll:N\n\x13generat\
    e_getter_all\x18\xed\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOp\
    tionsR\x11generateGetterAll:b\n\x1ecarllerche_bytes_for_bytes_all\x18\
    \xf3\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x1acarlle\
    rcheBytesForBytesAll:d\n\x1fcarllerche_bytes_for_string_all\x18\xf4\x84\
    \x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x1bcarllercheByte\
    sForStringAll:H\n\x10serde_derive_all\x18\x86\x85\x01\x20\x01(\x08\x12\
    \x1c.google.protobuf.FileOptionsR\x0eserdeDeriveAll:O\n\x14serde_derive_\
    cfg_all\x18\x87\x85\x01\x20\x01(\t\x12\x1c.google.protobuf.FileOptionsR\
    \x11serdeDeriveCfgAll:H\n\x10lite_runtime_all\x18\x8b\x85\x01\x20\x01(\
    \x08\x12\x1c.google.protobuf.FileOptionsR\x0eliteRuntimeAll:D\n\x0cexpos\
    e_oneof\x18\xe9\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOpti\
    onsR\x0bexposeOneof:F\n\rexpose_fields\x18\xeb\x84\x01\x20\x01(\x08\x12\
    \x1f.google.protobuf.MessageOptionsR\x0cexposeFields:P\n\x12generate_acc\
    essors\x18\xec\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptio\
    nsR\x11generateAccessors:J\n\x0fgenerate_getter\x18\xed\x84\x01\x20\x01(\
    \x08\x12\x1f.google.protobuf.MessageOptionsR\x0egenerateGetter:^\n\x1aca\
    rllerche_bytes_for_bytes\x18\xf3\x84\x01\x20\x01(\x08\x12\x1f.google.pro\
    tobuf.MessageOptionsR\x17carllercheBytesForBytes:`\n\x1bcarllerche_bytes\
    _for_string\x18\xf4\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.Message\
    OptionsR\x18carllercheBytesForString:D\n\x0cserde_derive\x18\x86\x85\x01\
    \x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x0bserdeDerive:K\n\
    \x10serde_derive_cfg\x18\x87\x85\x01\x20\x01(\t\x12\x1f.google.protobuf.\
    MessageOptionsR\x0eserdeDeriveCfg:O\n\x13expose_fields_field\x18\xeb\x84\
    \x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x11exposeFieldsF\
    ield:Y\n\x18generate_accessors_field\x18\xec\x84\x01\x20\x01(\x08\x12\
    \x1d.google.protobuf.FieldOptionsR\x16generateAccessorsField:S\n\x15gene\
    rate_getter_field\x18\xed\x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.F\
    ieldOptionsR\x13generateGetterField:g\n\x20carllerche_bytes_for_bytes_fi\
    eld\x18\xf3\x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\
    \x1ccarllercheBytesForBytesField:i\n!carllerche_bytes_for_string_field\
    \x18\xf4\x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x1dc\
    arllercheBytesForStringFieldJ\x8e\x18\n\x06\x12\x04\0\0A\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\0*\n\xe5\x01\n\x01\x02\
    \x12\x03\n\0\x122^\x20see\x20https://github.com/gogo/protobuf/blob/maste\
    r/gogoproto/gogo.proto\n\x20for\x20the\x20original\x20idea\n2{\x20Genera\
    ted\x20files\x20can\x20be\x20customized\x20using\x20this\x20proto\n\x20o\
    r\x20using\x20`Customize`\x20struct\x20when\x20codegen\x20is\x20invoked\
    \x20programmatically.\n\n\t\n\x01\x07\x12\x04\x0c\0!\x01\n7\n\x02\x07\0\
    \x12\x03\x0e\x04+\x1a,\x20When\x20true,\x20oneof\x20field\x20is\x20gener\
    ated\x20public\n\n\n\n\x03\x07\0\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\0\
    \x04\x12\x03\x0e\x04\x0c\n\n\n\x03\x07\0\x05\x12\x03\x0e\r\x11\n\n\n\x03\
    \x07\0\x01\x12\x03\x0e\x12\"\n\n\n\x03\x07\0\x03\x12\x03\x0e%*\nI\n\x02\
    \x07\x01\x12\x03\x10\x04,\x1a>\x20When\x20true\x20all\x20fields\x20are\
    \x20public,\x20and\x20not\x20accessors\x20generated\n\n\n\n\x03\x07\x01\
    \x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x01\x04\x12\x03\x10\x04\x0c\n\n\n\
    \x03\x07\x01\x05\x12\x03\x10\r\x11\n\n\n\x03\x07\x01\x01\x12\x03\x10\x12\
    #\n\n\n\x03\x07\x01\x03\x12\x03\x10&+\nP\n\x02\x07\x02\x12\x03\x12\x041\
    \x1aE\x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20acces\
    sors\x20are\x20not\x20generated\n\n\n\n\x03\x07\x02\x02\x12\x03\x0c\x07\
    \"\n\n\n\x03\x07\x02\x04\x12\x03\x12\x04\x0c\n\n\n\x03\x07\x02\x05\x12\
    \x03\x12\r\x11\n\n\n\x03\x07\x02\x01\x12\x03\x12\x12(\n\n\n\x03\x07\x02\
    \x03\x12\x03\x12+0\nL\n\x02\x07\x03\x12\x03\x14\x04.\x1aA\x20When\x20fal\
    se,\x20`get_`\x20is\x20not\x20generated\x20even\x20if\x20`syntax\x20=\
    \x20\"proto2\"`\n\n\n\n\x03\x07\x03\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\
    \x03\x04\x12\x03\x14\x04\x0c\n\n\n\x03\x07\x03\x05\x12\x03\x14\r\x11\n\n\
    \n\x03\x07\x03\x01\x12\x03\x14\x12%\n\n\n\x03\x07\x03\x03\x12\x03\x14(-\
    \n2\n\x02\x07\x04\x12\x03\x16\x049\x1a'\x20Use\x20`bytes::Bytes`\x20for\
    \x20`bytes`\x20fields\n\n\n\n\x03\x07\x04\x02\x12\x03\x0c\x07\"\n\n\n\
    \x03\x07\x04\x04\x12\x03\x16\x04\x0c\n\n\n\x03\x07\x04\x05\x12\x03\x16\r\
    \x11\n\n\n\x03\x07\x04\x01\x12\x03\x16\x120\n\n\n\x03\x07\x04\x03\x12\
    \x03\x1638\n3\n\x02\x07\x05\x12\x03\x18\x04:\x1a(\x20Use\x20`bytes::Byte\
    s`\x20for\x20`string`\x20fields\n\n\n\n\x03\x07\x05\x02\x12\x03\x0c\x07\
    \"\n\n\n\x03\x07\x05\x04\x12\x03\x18\x04\x0c\n\n\n\x03\x07\x05\x05\x12\
    \x03\x18\r\x11\n\n\n\x03\x07\x05\x01\x12\x03\x18\x121\n\n\n\x03\x07\x05\
    \x03\x12\x03\x1849\nJ\n\x02\x07\x06\x12\x03\x1b\x04+\x1a?\x20Use\x20`ser\
    de_derive`\x20to\x20implement\x20`Serialize`\x20and\x20`Deserialize`\n\n\
    \n\n\x03\x07\x06\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x06\x04\x12\x03\x1b\
    \x04\x0c\n\n\n\x03\x07\x06\x05\x12\x03\x1b\r\x11\n\n\n\x03\x07\x06\x01\
    \x12\x03\x1b\x12\"\n\n\n\x03\x07\x06\x03\x12\x03\x1b%*\n3\n\x02\x07\x07\
    \x12\x03\x1d\x041\x1a(\x20Guard\x20serde\x20annotations\x20with\x20cfg\
    \x20attr.\n\n\n\n\x03\x07\x07\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x07\
    \x04\x12\x03\x1d\x04\x0c\n\n\n\x03\x07\x07\x05\x12\x03\x1d\r\x13\n\n\n\
    \x03\x07\x07\x01\x12\x03\x1d\x14(\n\n\n\x03\x07\x07\x03\x12\x03\x1d+0\nN\
    \n\x02\x07\x08\x12\x03\x20\x04+\x1aC\x20When\x20true,\x20will\x20only\
    \x20generate\x20codes\x20that\x20works\x20with\x20lite\x20runtime.\n\n\n\
    \n\x03\x07\x08\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x08\x04\x12\x03\x20\
    \x04\x0c\n\n\n\x03\x07\x08\x05\x12\x03\x20\r\x11\n\n\n\x03\x07\x08\x01\
    \x12\x03\x20\x12\"\n\n\n\x03\x07\x08\x03\x12\x03\x20%*\n\t\n\x01\x07\x12\
    \x04#\04\x01\n7\n\x02\x07\t\x12\x03%\x04'\x1a,\x20When\x20true,\x20oneof\
    \x20field\x20is\x20generated\x20public\n\n\n\n\x03\x07\t\x02\x12\x03#\
    \x07%\n\n\n\x03\x07\t\x04\x12\x03%\x04\x0c\n\n\n\x03\x07\t\x05\x12\x03%\
    \r\x11\n\n\n\x03\x07\t\x01\x12\x03%\x12\x1e\n\n\n\x03\x07\t\x03\x12\x03%\
    !&\nI\n\x02\x07\n\x12\x03'\x04(\x1a>\x20When\x20true\x20all\x20fields\
    \x20are\x20public,\x20and\x20not\x20accessors\x20generated\n\n\n\n\x03\
    \x07\n\x02\x12\x03#\x07%\n\n\n\x03\x07\n\x04\x12\x03'\x04\x0c\n\n\n\x03\
    \x07\n\x05\x12\x03'\r\x11\n\n\n\x03\x07\n\x01\x12\x03'\x12\x1f\n\n\n\x03\
    \x07\n\x03\x12\x03'\"'\nP\n\x02\x07\x0b\x12\x03)\x04-\x1aE\x20When\x20fa\
    lse,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20accessors\x20are\x20not\
    \x20generated\n\n\n\n\x03\x07\x0b\x02\x12\x03#\x07%\n\n\n\x03\x07\x0b\
    \x04\x12\x03)\x04\x0c\n\n\n\x03\x07\x0b\x05\x12\x03)\r\x11\n\n\n\x03\x07\
    \x0b\x01\x12\x03)\x12$\n\n\n\x03\x07\x0b\x03\x12\x03)',\nL\n\x02\x07\x0c\
    \x12\x03+\x04*\x1aA\x20When\x20false,\x20`get_`\x20is\x20not\x20generate\
    d\x20even\x20if\x20`syntax\x20=\x20\"proto2\"`\n\n\n\n\x03\x07\x0c\x02\
    \x12\x03#\x07%\n\n\n\x03\x07\x0c\x04\x12\x03+\x04\x0c\n\n\n\x03\x07\x0c\
    \x05\x12\x03+\r\x11\n\n\n\x03\x07\x0c\x01\x12\x03+\x12!\n\n\n\x03\x07\
    \x0c\x03\x12\x03+$)\n2\n\x02\x07\r\x12\x03-\x045\x1a'\x20Use\x20`bytes::\
    Bytes`\x20for\x20`bytes`\x20fields\n\n\n\n\x03\x07\r\x02\x12\x03#\x07%\n\
    \n\n\x03\x07\r\x04\x12\x03-\x04\x0c\n\n\n\x03\x07\r\x05\x12\x03-\r\x11\n\
    \n\n\x03\x07\r\x01\x12\x03-\x12,\n\n\n\x03\x07\r\x03\x12\x03-/4\n3\n\x02\
    \x07\x0e\x12\x03/\x046\x1a(\x20Use\x20`bytes::Bytes`\x20for\x20`string`\
    \x20fields\n\n\n\n\x03\x07\x0e\x02\x12\x03#\x07%\n\n\n\x03\x07\x0e\x04\
    \x12\x03/\x04\x0c\n\n\n\x03\x07\x0e\x05\x12\x03/\r\x11\n\n\n\x03\x07\x0e\
    \x01\x12\x03/\x12-\n\n\n\x03\x07\x0e\x03\x12\x03/05\nJ\n\x02\x07\x0f\x12\
    \x031\x04'\x1a?\x20Use\x20`serde_derive`\x20to\x20implement\x20`Serializ\
    e`\x20and\x20`Deserialize`\n\n\n\n\x03\x07\x0f\x02\x12\x03#\x07%\n\n\n\
    \x03\x07\x0f\x04\x12\x031\x04\x0c\n\n\n\x03\x07\x0f\x05\x12\x031\r\x11\n\
    \n\n\x03\x07\x0f\x01\x12\x031\x12\x1e\n\n\n\x03\x07\x0f\x03\x12\x031!&\n\
    3\n\x02\x07\x10\x12\x033\x04-\x1a(\x20Guard\x20serde\x20annotations\x20w\
    ith\x20cfg\x20attr.\n\n\n\n\x03\x07\x10\x02\x12\x03#\x07%\n\n\n\x03\x07\
    \x10\x04\x12\x033\x04\x0c\n\n\n\x03\x07\x10\x05\x12\x033\r\x13\n\n\n\x03\
    \x07\x10\x01\x12\x033\x14$\n\n\n\x03\x07\x10\x03\x12\x033',\n\t\n\x01\
    \x07\x12\x046\0A\x01\nI\n\x02\x07\x11\x12\x038\x04.\x1a>\x20When\x20true\
    \x20all\x20fields\x20are\x20public,\x20and\x20not\x20accessors\x20genera\
    ted\n\n\n\n\x03\x07\x11\x02\x12\x036\x07#\n\n\n\x03\x07\x11\x04\x12\x038\
    \x04\x0c\n\n\n\x03\x07\x11\x05\x12\x038\r\x11\n\n\n\x03\x07\x11\x01\x12\
    \x038\x12%\n\n\n\x03\x07\x11\x03\x12\x038(-\nP\n\x02\x07\x12\x12\x03:\
    \x043\x1aE\x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20\
    accessors\x20are\x20not\x20generated\n\n\n\n\x03\x07\x12\x02\x12\x036\
    \x07#\n\n\n\x03\x07\x12\x04\x12\x03:\x04\x0c\n\n\n\x03\x07\x12\x05\x12\
    \x03:\r\x11\n\n\n\x03\x07\x12\x01\x12\x03:\x12*\n\n\n\x03\x07\x12\x03\
    \x12\x03:-2\nL\n\x02\x07\x13\x12\x03<\x040\x1aA\x20When\x20false,\x20`ge\
    t_`\x20is\x20not\x20generated\x20even\x20if\x20`syntax\x20=\x20\"proto2\
    \"`\n\n\n\n\x03\x07\x13\x02\x12\x036\x07#\n\n\n\x03\x07\x13\x04\x12\x03<\
    \x04\x0c\n\n\n\x03\x07\x13\x05\x12\x03<\r\x11\n\n\n\x03\x07\x13\x01\x12\
    \x03<\x12'\n\n\n\x03\x07\x13\x03\x12\x03<*/\n2\n\x02\x07\x14\x12\x03>\
    \x04;\x1a'\x20Use\x20`bytes::Bytes`\x20for\x20`bytes`\x20fields\n\n\n\n\
    \x03\x07\x14\x02\x12\x036\x07#\n\n\n\x03\x07\x14\x04\x12\x03>\x04\x0c\n\
    \n\n\x03\x07\x14\x05\x12\x03>\r\x11\n\n\n\x03\x07\x14\x01\x12\x03>\x122\
    \n\n\n\x03\x07\x14\x03\x12\x03>5:\n3\n\x02\x07\x15\x12\x03@\x04<\x1a(\
    \x20Use\x20`bytes::Bytes`\x20for\x20`string`\x20fields\n\n\n\n\x03\x07\
    \x15\x02\x12\x036\x07#\n\n\n\x03\x07\x15\x04\x12\x03@\x04\x0c\n\n\n\x03\
    \x07\x15\x05\x12\x03@\r\x11\n\n\n\x03\x07\x15\x01\x12\x03@\x123\n\n\n\
    \x03\x07\x15\x03\x12\x03@6;\
";

/// `FileDescriptorProto` object which was a source for this generated file
pub fn file_descriptor_proto() -> &'static crate::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: crate::rt::LazyV2<crate::descriptor::FileDescriptorProto> = crate::rt::LazyV2::INIT;
    file_descriptor_proto_lazy.get(|| {
        crate::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static crate::reflect::FileDescriptor {
    static file_descriptor_lazy: crate::rt::LazyV2<crate::reflect::FileDescriptor> = crate::rt::LazyV2::INIT;
    file_descriptor_lazy.get(|| {
        let mut deps = ::std::vec::Vec::new();
        deps.push(crate::descriptor::file_descriptor());
        let mut messages = ::std::vec::Vec::new();
        let mut enums = ::std::vec::Vec::new();
        crate::reflect::FileDescriptor::new_generated(
            file_descriptor_proto(),
            deps,
            messages,
            enums,
        )
    })
}
