use hoover3_macro::{model, udt_model};
use hoover3_types::db_schema::Timestamp;
use serde::Serialize;
#[allow(non_camel_case_types)]
/// Documentation
pub struct simple_model_udt {
    /// Some Field
    pub id: ::hoover3_database::charybdis::types::Text,
    /// Other Field
    pub another_field: Option<::charybdis::types::Int>,
    /// Timestamp field
    pub created_at: ::hoover3_database::charybdis::types::Timestamp,
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::fmt::Debug for simple_model_udt {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "simple_model_udt",
            "id",
            &self.id,
            "another_field",
            &self.another_field,
            "created_at",
            &&self.created_at,
        )
    }
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for simple_model_udt {
    #[inline]
    fn clone(&self) -> simple_model_udt {
        simple_model_udt {
            id: ::core::clone::Clone::clone(&self.id),
            another_field: ::core::clone::Clone::clone(&self.another_field),
            created_at: ::core::clone::Clone::clone(&self.created_at),
        }
    }
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::hash::Hash for simple_model_udt {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.id, state);
        ::core::hash::Hash::hash(&self.another_field, state);
        ::core::hash::Hash::hash(&self.created_at, state)
    }
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::marker::StructuralPartialEq for simple_model_udt {}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for simple_model_udt {
    #[inline]
    fn eq(&self, other: &simple_model_udt) -> bool {
        self.id == other.id && self.another_field == other.another_field
            && self.created_at == other.created_at
    }
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialOrd for simple_model_udt {
    #[inline]
    fn partial_cmp(
        &self,
        other: &simple_model_udt,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&self.id, &other.id) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                match ::core::cmp::PartialOrd::partial_cmp(
                    &self.another_field,
                    &other.another_field,
                ) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(
                            &self.created_at,
                            &other.created_at,
                        )
                    }
                    cmp => cmp,
                }
            }
            cmp => cmp,
        }
    }
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for simple_model_udt {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "simple_model_udt",
                false as usize + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "id",
                &self.id,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "another_field",
                &self.another_field,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "created_at",
                &self.created_at,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for simple_model_udt {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "id" => _serde::__private::Ok(__Field::__field0),
                        "another_field" => _serde::__private::Ok(__Field::__field1),
                        "created_at" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"id" => _serde::__private::Ok(__Field::__field0),
                        b"another_field" => _serde::__private::Ok(__Field::__field1),
                        b"created_at" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<simple_model_udt>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = simple_model_udt;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct simple_model_udt",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        ::hoover3_database::charybdis::types::Text,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct simple_model_udt with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        Option<::charybdis::types::Int>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct simple_model_udt with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        ::hoover3_database::charybdis::types::Timestamp,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct simple_model_udt with 3 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(simple_model_udt {
                        id: __field0,
                        another_field: __field1,
                        created_at: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<
                        ::hoover3_database::charybdis::types::Text,
                    > = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<
                        Option<::charybdis::types::Int>,
                    > = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<
                        ::hoover3_database::charybdis::types::Timestamp,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        ::hoover3_database::charybdis::types::Text,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "another_field",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<::charybdis::types::Int>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "created_at",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        ::hoover3_database::charybdis::types::Timestamp,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("id")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("another_field")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("created_at")?
                        }
                    };
                    _serde::__private::Ok(simple_model_udt {
                        id: __field0,
                        another_field: __field1,
                        created_at: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &[
                "id",
                "another_field",
                "created_at",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "simple_model_udt",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<simple_model_udt>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl<
    'lifetime,
    'lifetime_,
> scylla::_macro_internal::DeserializeValue<'lifetime, 'lifetime_> for simple_model_udt {
    fn type_check(
        typ: &scylla::_macro_internal::ColumnType,
    ) -> ::std::result::Result<(), scylla::_macro_internal::TypeCheckError> {
        let cql_fields = match typ {
            scylla::_macro_internal::ColumnType::UserDefinedType { field_types, .. } => {
                field_types
            }
            other => {
                return ::std::result::Result::Err(
                    scylla::_macro_internal::mk_value_typck_err::<
                        Self,
                    >(
                        &other,
                        scylla::_macro_internal::DeserUdtTypeCheckErrorKind::NotUdt,
                    ),
                );
            }
        };
        let mut remaining_required_cql_fields: ::std::primitive::usize = 3;
        let mut visited_id = false;
        let mut visited_another_field = false;
        let mut visited_created_at = false;
        for (cql_field_name, cql_field_typ) in cql_fields {
            match std::ops::Deref::deref(cql_field_name) {
                "id" => {
                    if !visited_id {
                        <::hoover3_database::charybdis::types::Text as scylla::_macro_internal::DeserializeValue<
                            'lifetime,
                            'lifetime_,
                        >>::type_check(cql_field_typ)
                            .map_err(|err| scylla::_macro_internal::mk_value_typck_err::<
                                Self,
                            >(
                                typ,
                                scylla::_macro_internal::DeserUdtTypeCheckErrorKind::FieldTypeCheckFailed {
                                    field_name: <_ as ::std::clone::Clone>::clone(
                                            cql_field_name,
                                        )
                                        .into_owned(),
                                    err,
                                },
                            ))?;
                        visited_id = true;
                        remaining_required_cql_fields -= 1;
                    } else {
                        return ::std::result::Result::Err(
                            scylla::_macro_internal::mk_value_typck_err::<
                                Self,
                            >(
                                typ,
                                scylla::_macro_internal::DeserUdtTypeCheckErrorKind::DuplicatedField {
                                    field_name: <_ as ::std::borrow::ToOwned>::to_owned("id"),
                                },
                            ),
                        )
                    }
                }
                "another_field" => {
                    if !visited_another_field {
                        <Option<
                            ::charybdis::types::Int,
                        > as scylla::_macro_internal::DeserializeValue<
                            'lifetime,
                            'lifetime_,
                        >>::type_check(cql_field_typ)
                            .map_err(|err| scylla::_macro_internal::mk_value_typck_err::<
                                Self,
                            >(
                                typ,
                                scylla::_macro_internal::DeserUdtTypeCheckErrorKind::FieldTypeCheckFailed {
                                    field_name: <_ as ::std::clone::Clone>::clone(
                                            cql_field_name,
                                        )
                                        .into_owned(),
                                    err,
                                },
                            ))?;
                        visited_another_field = true;
                        remaining_required_cql_fields -= 1;
                    } else {
                        return ::std::result::Result::Err(
                            scylla::_macro_internal::mk_value_typck_err::<
                                Self,
                            >(
                                typ,
                                scylla::_macro_internal::DeserUdtTypeCheckErrorKind::DuplicatedField {
                                    field_name: <_ as ::std::borrow::ToOwned>::to_owned(
                                        "another_field",
                                    ),
                                },
                            ),
                        )
                    }
                }
                "created_at" => {
                    if !visited_created_at {
                        <::hoover3_database::charybdis::types::Timestamp as scylla::_macro_internal::DeserializeValue<
                            'lifetime,
                            'lifetime_,
                        >>::type_check(cql_field_typ)
                            .map_err(|err| scylla::_macro_internal::mk_value_typck_err::<
                                Self,
                            >(
                                typ,
                                scylla::_macro_internal::DeserUdtTypeCheckErrorKind::FieldTypeCheckFailed {
                                    field_name: <_ as ::std::clone::Clone>::clone(
                                            cql_field_name,
                                        )
                                        .into_owned(),
                                    err,
                                },
                            ))?;
                        visited_created_at = true;
                        remaining_required_cql_fields -= 1;
                    } else {
                        return ::std::result::Result::Err(
                            scylla::_macro_internal::mk_value_typck_err::<
                                Self,
                            >(
                                typ,
                                scylla::_macro_internal::DeserUdtTypeCheckErrorKind::DuplicatedField {
                                    field_name: <_ as ::std::borrow::ToOwned>::to_owned(
                                        "created_at",
                                    ),
                                },
                            ),
                        )
                    }
                }
                unknown => {}
            }
        }
        if remaining_required_cql_fields > 0 {
            let mut missing_fields = ::std::vec::Vec::<
                &'static str,
            >::with_capacity(remaining_required_cql_fields);
            {
                if !visited_id {
                    missing_fields.push("id");
                }
            }
            {
                if !visited_another_field {
                    missing_fields.push("another_field");
                }
            }
            {
                if !visited_created_at {
                    missing_fields.push("created_at");
                }
            }
            return ::std::result::Result::Err(
                scylla::_macro_internal::mk_value_typck_err::<
                    Self,
                >(
                    typ,
                    scylla::_macro_internal::DeserUdtTypeCheckErrorKind::ValuesMissingForUdtFields {
                        field_names: missing_fields,
                    },
                ),
            );
        }
        ::std::result::Result::Ok(())
    }
    fn deserialize(
        typ: &'lifetime_ scylla::_macro_internal::ColumnType<'lifetime_>,
        v: ::std::option::Option<scylla::_macro_internal::FrameSlice<'lifetime>>,
    ) -> ::std::result::Result<Self, scylla::_macro_internal::DeserializationError> {
        let cql_field_iter = <scylla::_macro_internal::UdtIterator<
            'lifetime,
            'lifetime_,
        > as scylla::_macro_internal::DeserializeValue<
            'lifetime,
            'lifetime_,
        >>::deserialize(typ, v)
            .map_err(
                scylla::_macro_internal::value_deser_error_replace_rust_name::<Self>,
            )?;
        let mut f_id = ::std::option::Option::None;
        let mut f_another_field = ::std::option::Option::None;
        let mut f_created_at = ::std::option::Option::None;
        for item in cql_field_iter {
            let ((cql_field_name, cql_field_typ), value_res) = item;
            let value = value_res
                .map_err(|err| scylla::_macro_internal::mk_value_deser_err::<
                    Self,
                >(
                    typ,
                    scylla::_macro_internal::UdtDeserializationErrorKind::FieldDeserializationFailed {
                        field_name: ::std::clone::Clone::clone(cql_field_name)
                            .into_owned(),
                        err,
                    },
                ))?;
            match std::ops::Deref::deref(cql_field_name) {
                "id" => {
                    if !f_id.is_none() {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "duplicated field {0} - type check should have prevented this!",
                                    "f_id",
                                ),
                            );
                        }
                    }
                    let value = value.flatten();
                    f_id = ::std::option::Option::Some(
                        <::hoover3_database::charybdis::types::Text as scylla::_macro_internal::DeserializeValue<
                            'lifetime,
                            'lifetime_,
                        >>::deserialize(cql_field_typ, value)
                            .map_err(|err| scylla::_macro_internal::mk_value_deser_err::<
                                Self,
                            >(
                                typ,
                                scylla::_macro_internal::UdtDeserializationErrorKind::FieldDeserializationFailed {
                                    field_name: "id".to_owned(),
                                    err,
                                },
                            ))?,
                    );
                }
                "another_field" => {
                    if !f_another_field.is_none() {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "duplicated field {0} - type check should have prevented this!",
                                    "f_another_field",
                                ),
                            );
                        }
                    }
                    let value = value.flatten();
                    f_another_field = ::std::option::Option::Some(
                        <Option<
                            ::charybdis::types::Int,
                        > as scylla::_macro_internal::DeserializeValue<
                            'lifetime,
                            'lifetime_,
                        >>::deserialize(cql_field_typ, value)
                            .map_err(|err| scylla::_macro_internal::mk_value_deser_err::<
                                Self,
                            >(
                                typ,
                                scylla::_macro_internal::UdtDeserializationErrorKind::FieldDeserializationFailed {
                                    field_name: "another_field".to_owned(),
                                    err,
                                },
                            ))?,
                    );
                }
                "created_at" => {
                    if !f_created_at.is_none() {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "duplicated field {0} - type check should have prevented this!",
                                    "f_created_at",
                                ),
                            );
                        }
                    }
                    let value = value.flatten();
                    f_created_at = ::std::option::Option::Some(
                        <::hoover3_database::charybdis::types::Timestamp as scylla::_macro_internal::DeserializeValue<
                            'lifetime,
                            'lifetime_,
                        >>::deserialize(cql_field_typ, value)
                            .map_err(|err| scylla::_macro_internal::mk_value_deser_err::<
                                Self,
                            >(
                                typ,
                                scylla::_macro_internal::UdtDeserializationErrorKind::FieldDeserializationFailed {
                                    field_name: "created_at".to_owned(),
                                    err,
                                },
                            ))?,
                    );
                }
                unknown => {}
            }
        }
        ::std::result::Result::Ok(Self {
            id: f_id
                .unwrap_or_else(|| {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "field {0} missing in UDT - type check should have prevented this!",
                            "id",
                        ),
                    );
                }),
            another_field: f_another_field
                .unwrap_or_else(|| {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "field {0} missing in UDT - type check should have prevented this!",
                            "another_field",
                        ),
                    );
                }),
            created_at: f_created_at
                .unwrap_or_else(|| {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "field {0} missing in UDT - type check should have prevented this!",
                            "created_at",
                        ),
                    );
                }),
        })
    }
}
#[automatically_derived]
impl ::scylla::_macro_internal::SerializeValue for simple_model_udt {
    fn serialize<'b>(
        &self,
        typ: &::scylla::_macro_internal::ColumnType,
        writer: ::scylla::_macro_internal::CellWriter<'b>,
    ) -> ::std::result::Result<
        ::scylla::_macro_internal::WrittenCellProof<'b>,
        ::scylla::_macro_internal::SerializationError,
    > {
        let mk_typck_err = |
            kind: ::scylla::_macro_internal::UdtTypeCheckErrorKind,
        | -> ::scylla::_macro_internal::SerializationError {
            ::scylla::_macro_internal::SerializationError::new(::scylla::_macro_internal::BuiltinTypeTypeCheckError {
                rust_name: ::std::any::type_name::<Self>(),
                got: <_ as ::std::clone::Clone>::clone(typ).into_owned(),
                kind: ::scylla::_macro_internal::BuiltinTypeTypeCheckErrorKind::UdtError(
                    kind,
                ),
            })
        };
        let mk_ser_err = |
            kind: ::scylla::_macro_internal::UdtSerializationErrorKind,
        | -> ::scylla::_macro_internal::SerializationError {
            ::scylla::_macro_internal::SerializationError::new(::scylla::_macro_internal::BuiltinTypeSerializationError {
                rust_name: ::std::any::type_name::<Self>(),
                got: <_ as ::std::clone::Clone>::clone(typ).into_owned(),
                kind: ::scylla::_macro_internal::BuiltinTypeSerializationErrorKind::UdtError(
                    kind,
                ),
            })
        };
        let (type_name, keyspace, field_types) = match typ {
            ::scylla::_macro_internal::ColumnType::UserDefinedType {
                type_name,
                keyspace,
                field_types,
                ..
            } => (type_name, keyspace, field_types),
            _ => {
                return ::std::result::Result::Err(
                    mk_typck_err(
                        ::scylla::_macro_internal::UdtTypeCheckErrorKind::NotUdt,
                    ),
                );
            }
        };
        let mut visited_flag_id = false;
        let mut visited_flag_another_field = false;
        let mut visited_flag_created_at = false;
        let mut remaining_count = 3usize;
        let mut skipped_fields = 0;
        let mut builder = ::scylla::_macro_internal::CellWriter::into_value_builder(
            writer,
        );
        for (field_name, field_type) in field_types {
            match ::std::ops::Deref::deref(field_name) {
                "id" => {
                    while skipped_fields > 0 {
                        let sub_builder = ::scylla::_macro_internal::CellValueBuilder::make_sub_writer(
                            &mut builder,
                        );
                        sub_builder.set_null();
                        skipped_fields -= 1;
                    }
                    let sub_builder = ::scylla::_macro_internal::CellValueBuilder::make_sub_writer(
                        &mut builder,
                    );
                    match <::hoover3_database::charybdis::types::Text as ::scylla::_macro_internal::SerializeValue>::serialize(
                        &self.id,
                        field_type,
                        sub_builder,
                    ) {
                        ::std::result::Result::Ok(_proof) => {}
                        ::std::result::Result::Err(err) => {
                            return ::std::result::Result::Err(
                                mk_ser_err(::scylla::_macro_internal::UdtSerializationErrorKind::FieldSerializationFailed {
                                    field_name: <_ as ::std::clone::Clone>::clone(field_name)
                                        .into_owned(),
                                    err,
                                }),
                            );
                        }
                    }
                    if !visited_flag_id {
                        visited_flag_id = true;
                        remaining_count -= 1;
                    }
                }
                "another_field" => {
                    while skipped_fields > 0 {
                        let sub_builder = ::scylla::_macro_internal::CellValueBuilder::make_sub_writer(
                            &mut builder,
                        );
                        sub_builder.set_null();
                        skipped_fields -= 1;
                    }
                    let sub_builder = ::scylla::_macro_internal::CellValueBuilder::make_sub_writer(
                        &mut builder,
                    );
                    match <Option<
                        ::charybdis::types::Int,
                    > as ::scylla::_macro_internal::SerializeValue>::serialize(
                        &self.another_field,
                        field_type,
                        sub_builder,
                    ) {
                        ::std::result::Result::Ok(_proof) => {}
                        ::std::result::Result::Err(err) => {
                            return ::std::result::Result::Err(
                                mk_ser_err(::scylla::_macro_internal::UdtSerializationErrorKind::FieldSerializationFailed {
                                    field_name: <_ as ::std::clone::Clone>::clone(field_name)
                                        .into_owned(),
                                    err,
                                }),
                            );
                        }
                    }
                    if !visited_flag_another_field {
                        visited_flag_another_field = true;
                        remaining_count -= 1;
                    }
                }
                "created_at" => {
                    while skipped_fields > 0 {
                        let sub_builder = ::scylla::_macro_internal::CellValueBuilder::make_sub_writer(
                            &mut builder,
                        );
                        sub_builder.set_null();
                        skipped_fields -= 1;
                    }
                    let sub_builder = ::scylla::_macro_internal::CellValueBuilder::make_sub_writer(
                        &mut builder,
                    );
                    match <::hoover3_database::charybdis::types::Timestamp as ::scylla::_macro_internal::SerializeValue>::serialize(
                        &self.created_at,
                        field_type,
                        sub_builder,
                    ) {
                        ::std::result::Result::Ok(_proof) => {}
                        ::std::result::Result::Err(err) => {
                            return ::std::result::Result::Err(
                                mk_ser_err(::scylla::_macro_internal::UdtSerializationErrorKind::FieldSerializationFailed {
                                    field_name: <_ as ::std::clone::Clone>::clone(field_name)
                                        .into_owned(),
                                    err,
                                }),
                            );
                        }
                    }
                    if !visited_flag_created_at {
                        visited_flag_created_at = true;
                        remaining_count -= 1;
                    }
                }
                _ => skipped_fields += 1,
            }
        }
        if remaining_count > 0 {
            if !visited_flag_id && !false {
                return ::std::result::Result::Err(
                    mk_typck_err(::scylla::_macro_internal::UdtTypeCheckErrorKind::ValueMissingForUdtField {
                        field_name: <_ as ::std::string::ToString>::to_string("id"),
                    }),
                );
            }
            if !visited_flag_another_field && !false {
                return ::std::result::Result::Err(
                    mk_typck_err(::scylla::_macro_internal::UdtTypeCheckErrorKind::ValueMissingForUdtField {
                        field_name: <_ as ::std::string::ToString>::to_string(
                            "another_field",
                        ),
                    }),
                );
            }
            if !visited_flag_created_at && !false {
                return ::std::result::Result::Err(
                    mk_typck_err(::scylla::_macro_internal::UdtTypeCheckErrorKind::ValueMissingForUdtField {
                        field_name: <_ as ::std::string::ToString>::to_string(
                            "created_at",
                        ),
                    }),
                );
            }
        }
        let proof = ::scylla::_macro_internal::CellValueBuilder::finish(builder)
            .map_err(|_| {
                ::scylla::_macro_internal::SerializationError::new(::scylla::_macro_internal::BuiltinTypeSerializationError {
                    rust_name: ::std::any::type_name::<Self>(),
                    got: <_ as ::std::clone::Clone>::clone(typ).into_owned(),
                    kind: ::scylla::_macro_internal::BuiltinTypeSerializationErrorKind::SizeOverflow,
                }) as ::scylla::_macro_internal::SerializationError
            })?;
        ::std::result::Result::Ok(proof)
    }
}
#[allow(non_upper_case_globals)]
const _: () = {
    static __INVENTORY: ::inventory::Node = ::inventory::Node {
        value: &{
            ::hoover3_database::models::collection::UdtModelDefinitionStatic {
                udt_name: "simple_model_udt",
                model_name: "simple_model_udt",
                docstring: "Documentation",
                charybdis_code: "/// Documentation\n#[::charybdis::macros::charybdis_udt_model(type_name = simple_model_udt)]\n#[derive(\n    Debug,\n    Clone,\n    Hash,\n    PartialEq,\n    PartialOrd,\n    ::serde::Serialize,\n    ::serde::Deserialize\n)]\npub struct simple_model_udt {\n    /// Some Field\n    pub id: ::hoover3_database::charybdis::types::Text,\n    /// Other Field\n    pub another_field: Option<::charybdis::types::Int>,\n    /// Timestamp field\n    pub created_at: ::hoover3_database::charybdis::types::Timestamp,\n}\n",
                fields: &[
                    ::hoover3_database::models::collection::ModelFieldDefinitionStatic {
                        name: "id",
                        field_type: ::hoover3_types::db_schema::DatabaseColumnType::String,
                        docstring: "Some Field",
                        clustering_key: false,
                        partition_key: false,
                        search_store: false,
                        search_index: false,
                        search_facet: false,
                        nullable: false,
                        field_type_original: "String",
                    },
                    ::hoover3_database::models::collection::ModelFieldDefinitionStatic {
                        name: "another_field",
                        field_type: ::hoover3_types::db_schema::DatabaseColumnType::Int32,
                        docstring: "Other Field",
                        clustering_key: false,
                        partition_key: false,
                        search_store: false,
                        search_index: false,
                        search_facet: false,
                        nullable: true,
                        field_type_original: "i32",
                    },
                    ::hoover3_database::models::collection::ModelFieldDefinitionStatic {
                        name: "created_at",
                        field_type: ::hoover3_types::db_schema::DatabaseColumnType::Timestamp,
                        docstring: "Timestamp field",
                        clustering_key: false,
                        partition_key: false,
                        search_store: false,
                        search_index: false,
                        search_facet: false,
                        nullable: false,
                        field_type_original: "Timestamp",
                    },
                ],
            }
        },
        next: ::inventory::core::cell::UnsafeCell::new(
            ::inventory::core::option::Option::None,
        ),
    };
    #[link_section = ".text.startup"]
    unsafe extern "C" fn __ctor() {
        unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
    }
    #[used]
    #[link_section = ".init_array"]
    static __CTOR: unsafe extern "C" fn() = __ctor;
};
/// Documentation
pub struct SimpleModelUdtWithTable {
    /// Some Field
    pub id: ::hoover3_database::charybdis::types::Text,
    /// Other Field
    pub another_field: Option<simple_model_udt>,
    /// The Field
    pub the_field: simple_model_udt,
}
#[automatically_derived]
impl ::core::fmt::Debug for SimpleModelUdtWithTable {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "SimpleModelUdtWithTable",
            "id",
            &self.id,
            "another_field",
            &self.another_field,
            "the_field",
            &&self.the_field,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for SimpleModelUdtWithTable {
    #[inline]
    fn clone(&self) -> SimpleModelUdtWithTable {
        SimpleModelUdtWithTable {
            id: ::core::clone::Clone::clone(&self.id),
            another_field: ::core::clone::Clone::clone(&self.another_field),
            the_field: ::core::clone::Clone::clone(&self.the_field),
        }
    }
}
#[automatically_derived]
impl ::core::hash::Hash for SimpleModelUdtWithTable {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.id, state);
        ::core::hash::Hash::hash(&self.another_field, state);
        ::core::hash::Hash::hash(&self.the_field, state)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for SimpleModelUdtWithTable {}
#[automatically_derived]
impl ::core::cmp::PartialEq for SimpleModelUdtWithTable {
    #[inline]
    fn eq(&self, other: &SimpleModelUdtWithTable) -> bool {
        self.id == other.id && self.another_field == other.another_field
            && self.the_field == other.the_field
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for SimpleModelUdtWithTable {
    #[inline]
    fn partial_cmp(
        &self,
        other: &SimpleModelUdtWithTable,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&self.id, &other.id) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                match ::core::cmp::PartialOrd::partial_cmp(
                    &self.another_field,
                    &other.another_field,
                ) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(
                            &self.the_field,
                            &other.the_field,
                        )
                    }
                    cmp => cmp,
                }
            }
            cmp => cmp,
        }
    }
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SimpleModelUdtWithTable {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "SimpleModelUdtWithTable",
                false as usize + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "id",
                &self.id,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "another_field",
                &self.another_field,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "the_field",
                &self.the_field,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SimpleModelUdtWithTable {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "id" => _serde::__private::Ok(__Field::__field0),
                        "another_field" => _serde::__private::Ok(__Field::__field1),
                        "the_field" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"id" => _serde::__private::Ok(__Field::__field0),
                        b"another_field" => _serde::__private::Ok(__Field::__field1),
                        b"the_field" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SimpleModelUdtWithTable>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SimpleModelUdtWithTable;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SimpleModelUdtWithTable",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        ::hoover3_database::charybdis::types::Text,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SimpleModelUdtWithTable with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        Option<simple_model_udt>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SimpleModelUdtWithTable with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        simple_model_udt,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct SimpleModelUdtWithTable with 3 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SimpleModelUdtWithTable {
                        id: __field0,
                        another_field: __field1,
                        the_field: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<
                        ::hoover3_database::charybdis::types::Text,
                    > = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<
                        Option<simple_model_udt>,
                    > = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<simple_model_udt> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        ::hoover3_database::charybdis::types::Text,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "another_field",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<simple_model_udt>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "the_field",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        simple_model_udt,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("id")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("another_field")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("the_field")?
                        }
                    };
                    _serde::__private::Ok(SimpleModelUdtWithTable {
                        id: __field0,
                        another_field: __field1,
                        the_field: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &[
                "id",
                "another_field",
                "the_field",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SimpleModelUdtWithTable",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SimpleModelUdtWithTable>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl<
    'lifetime,
    'lifetime_,
> scylla::_macro_internal::DeserializeRow<'lifetime, 'lifetime_>
for SimpleModelUdtWithTable {
    fn type_check(
        specs: &[scylla::_macro_internal::ColumnSpec],
    ) -> ::std::result::Result<(), scylla::_macro_internal::TypeCheckError> {
        let mut remaining_required_fields: ::std::primitive::usize = 3usize;
        let mut visited_id = false;
        let mut visited_another_field = false;
        let mut visited_the_field = false;
        let column_types_iter = || {
            specs.iter().map(|spec| ::std::clone::Clone::clone(spec.typ()).into_owned())
        };
        for (column_index, spec) in specs.iter().enumerate() {
            match spec.name() {
                "id" => {
                    if !visited_id {
                        <::hoover3_database::charybdis::types::Text as scylla::_macro_internal::DeserializeValue<
                            'lifetime,
                            'lifetime_,
                        >>::type_check(spec.typ())
                            .map_err(|err| {
                                scylla::_macro_internal::mk_row_typck_err::<
                                    Self,
                                >(
                                    column_types_iter(),
                                    scylla::_macro_internal::DeserBuiltinRowTypeCheckErrorKind::ColumnTypeCheckFailed {
                                        column_index,
                                        column_name: <_ as ::std::borrow::ToOwned>::to_owned("id"),
                                        err,
                                    },
                                )
                            })?;
                        visited_id = true;
                        remaining_required_fields -= 1;
                    } else {
                        return ::std::result::Result::Err(
                            scylla::_macro_internal::mk_row_typck_err::<
                                Self,
                            >(
                                column_types_iter(),
                                scylla::_macro_internal::DeserBuiltinRowTypeCheckErrorKind::DuplicatedColumn {
                                    column_index,
                                    column_name: "id",
                                },
                            ),
                        )
                    }
                }
                "another_field" => {
                    if !visited_another_field {
                        <Option<
                            simple_model_udt,
                        > as scylla::_macro_internal::DeserializeValue<
                            'lifetime,
                            'lifetime_,
                        >>::type_check(spec.typ())
                            .map_err(|err| {
                                scylla::_macro_internal::mk_row_typck_err::<
                                    Self,
                                >(
                                    column_types_iter(),
                                    scylla::_macro_internal::DeserBuiltinRowTypeCheckErrorKind::ColumnTypeCheckFailed {
                                        column_index,
                                        column_name: <_ as ::std::borrow::ToOwned>::to_owned(
                                            "another_field",
                                        ),
                                        err,
                                    },
                                )
                            })?;
                        visited_another_field = true;
                        remaining_required_fields -= 1;
                    } else {
                        return ::std::result::Result::Err(
                            scylla::_macro_internal::mk_row_typck_err::<
                                Self,
                            >(
                                column_types_iter(),
                                scylla::_macro_internal::DeserBuiltinRowTypeCheckErrorKind::DuplicatedColumn {
                                    column_index,
                                    column_name: "another_field",
                                },
                            ),
                        )
                    }
                }
                "the_field" => {
                    if !visited_the_field {
                        <simple_model_udt as scylla::_macro_internal::DeserializeValue<
                            'lifetime,
                            'lifetime_,
                        >>::type_check(spec.typ())
                            .map_err(|err| {
                                scylla::_macro_internal::mk_row_typck_err::<
                                    Self,
                                >(
                                    column_types_iter(),
                                    scylla::_macro_internal::DeserBuiltinRowTypeCheckErrorKind::ColumnTypeCheckFailed {
                                        column_index,
                                        column_name: <_ as ::std::borrow::ToOwned>::to_owned(
                                            "the_field",
                                        ),
                                        err,
                                    },
                                )
                            })?;
                        visited_the_field = true;
                        remaining_required_fields -= 1;
                    } else {
                        return ::std::result::Result::Err(
                            scylla::_macro_internal::mk_row_typck_err::<
                                Self,
                            >(
                                column_types_iter(),
                                scylla::_macro_internal::DeserBuiltinRowTypeCheckErrorKind::DuplicatedColumn {
                                    column_index,
                                    column_name: "the_field",
                                },
                            ),
                        )
                    }
                }
                _unknown => {
                    return ::std::result::Result::Err(
                        scylla::_macro_internal::mk_row_typck_err::<
                            Self,
                        >(
                            column_types_iter(),
                            scylla::_macro_internal::DeserBuiltinRowTypeCheckErrorKind::ColumnWithUnknownName {
                                column_index,
                                column_name: <_ as ::std::borrow::ToOwned>::to_owned(
                                    spec.name(),
                                ),
                            },
                        ),
                    );
                }
            }
        }
        if remaining_required_fields > 0 {
            let mut missing_fields = ::std::vec::Vec::<
                &'static str,
            >::with_capacity(remaining_required_fields);
            {
                if !visited_id {
                    missing_fields.push("id");
                }
            }
            {
                if !visited_another_field {
                    missing_fields.push("another_field");
                }
            }
            {
                if !visited_the_field {
                    missing_fields.push("the_field");
                }
            }
            return ::std::result::Result::Err(
                scylla::_macro_internal::mk_row_typck_err::<
                    Self,
                >(
                    column_types_iter(),
                    scylla::_macro_internal::DeserBuiltinRowTypeCheckErrorKind::ValuesMissingForColumns {
                        column_names: missing_fields,
                    },
                ),
            );
        }
        ::std::result::Result::Ok(())
    }
    fn deserialize(
        #[allow(unused_mut)]
        mut row: scylla::_macro_internal::ColumnIterator<'lifetime, 'lifetime_>,
    ) -> ::std::result::Result<Self, scylla::_macro_internal::DeserializationError> {
        let mut f_id = ::std::option::Option::None;
        let mut f_another_field = ::std::option::Option::None;
        let mut f_the_field = ::std::option::Option::None;
        for col in row {
            let col = col
                .map_err(
                    scylla::_macro_internal::row_deser_error_replace_rust_name::<Self>,
                )?;
            match col.spec.name() {
                "id" => {
                    if !f_id.is_none() {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "duplicated column {0} - type check should have prevented this!",
                                    "f_id",
                                ),
                            );
                        }
                    }
                    f_id = ::std::option::Option::Some(
                        <::hoover3_database::charybdis::types::Text as scylla::_macro_internal::DeserializeValue<
                            'lifetime,
                            'lifetime_,
                        >>::deserialize(col.spec.typ(), col.slice)
                            .map_err(|err| {
                                scylla::_macro_internal::mk_row_deser_err::<
                                    Self,
                                >(scylla::_macro_internal::BuiltinRowDeserializationErrorKind::ColumnDeserializationFailed {
                                    column_index: 0usize,
                                    column_name: <_ as std::borrow::ToOwned>::to_owned(
                                        col.spec.name(),
                                    ),
                                    err,
                                })
                            })?,
                    );
                }
                "another_field" => {
                    if !f_another_field.is_none() {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "duplicated column {0} - type check should have prevented this!",
                                    "f_another_field",
                                ),
                            );
                        }
                    }
                    f_another_field = ::std::option::Option::Some(
                        <Option<
                            simple_model_udt,
                        > as scylla::_macro_internal::DeserializeValue<
                            'lifetime,
                            'lifetime_,
                        >>::deserialize(col.spec.typ(), col.slice)
                            .map_err(|err| {
                                scylla::_macro_internal::mk_row_deser_err::<
                                    Self,
                                >(scylla::_macro_internal::BuiltinRowDeserializationErrorKind::ColumnDeserializationFailed {
                                    column_index: 1usize,
                                    column_name: <_ as std::borrow::ToOwned>::to_owned(
                                        col.spec.name(),
                                    ),
                                    err,
                                })
                            })?,
                    );
                }
                "the_field" => {
                    if !f_the_field.is_none() {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "duplicated column {0} - type check should have prevented this!",
                                    "f_the_field",
                                ),
                            );
                        }
                    }
                    f_the_field = ::std::option::Option::Some(
                        <simple_model_udt as scylla::_macro_internal::DeserializeValue<
                            'lifetime,
                            'lifetime_,
                        >>::deserialize(col.spec.typ(), col.slice)
                            .map_err(|err| {
                                scylla::_macro_internal::mk_row_deser_err::<
                                    Self,
                                >(scylla::_macro_internal::BuiltinRowDeserializationErrorKind::ColumnDeserializationFailed {
                                    column_index: 2usize,
                                    column_name: <_ as std::borrow::ToOwned>::to_owned(
                                        col.spec.name(),
                                    ),
                                    err,
                                })
                            })?,
                    );
                }
                unknown => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("Typecheck should have prevented this scenario! Unknown column name: {0}",
                            unknown,),
                        ),
                    );
                }
            }
        }
        Ok(Self {
            id: f_id
                .unwrap_or_else(|| {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "column {0} missing in DB row - type check should have prevented this!",
                            "id",
                        ),
                    );
                }),
            another_field: f_another_field
                .unwrap_or_else(|| {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "column {0} missing in DB row - type check should have prevented this!",
                            "another_field",
                        ),
                    );
                }),
            the_field: f_the_field
                .unwrap_or_else(|| {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "column {0} missing in DB row - type check should have prevented this!",
                            "the_field",
                        ),
                    );
                }),
        })
    }
}
#[automatically_derived]
impl ::scylla::_macro_internal::SerializeRow for SimpleModelUdtWithTable {
    fn serialize<'b>(
        &self,
        ctx: &::scylla::_macro_internal::RowSerializationContext,
        writer: &mut ::scylla::_macro_internal::RowWriter<'b>,
    ) -> ::std::result::Result<(), ::scylla::_macro_internal::SerializationError> {
        let mk_typck_err = |
            kind: ::scylla::_macro_internal::BuiltinRowTypeCheckErrorKind,
        | -> ::scylla::_macro_internal::SerializationError {
            ::scylla::_macro_internal::SerializationError::new(::scylla::_macro_internal::BuiltinRowTypeCheckError {
                rust_name: ::std::any::type_name::<Self>(),
                kind,
            })
        };
        let mk_ser_err = |
            kind: ::scylla::_macro_internal::BuiltinRowSerializationErrorKind,
        | -> ::scylla::_macro_internal::SerializationError {
            ::scylla::_macro_internal::SerializationError::new(::scylla::_macro_internal::BuiltinRowSerializationError {
                rust_name: ::std::any::type_name::<Self>(),
                kind,
            })
        };
        let mut visited_flag_id = false;
        let mut visited_flag_another_field = false;
        let mut visited_flag_the_field = false;
        let mut remaining_count = 3usize;
        for spec in ctx.columns() {
            match spec.name() {
                "id" => {
                    let sub_writer = ::scylla::_macro_internal::RowWriter::make_cell_writer(
                        writer,
                    );
                    match <::hoover3_database::charybdis::types::Text as ::scylla::_macro_internal::SerializeValue>::serialize(
                        &self.id,
                        spec.typ(),
                        sub_writer,
                    ) {
                        ::std::result::Result::Ok(_proof) => {}
                        ::std::result::Result::Err(err) => {
                            return ::std::result::Result::Err(
                                mk_ser_err(::scylla::_macro_internal::BuiltinRowSerializationErrorKind::ColumnSerializationFailed {
                                    name: <_ as ::std::borrow::ToOwned>::to_owned(spec.name()),
                                    err,
                                }),
                            );
                        }
                    }
                    if !visited_flag_id {
                        visited_flag_id = true;
                        remaining_count -= 1;
                    }
                }
                "another_field" => {
                    let sub_writer = ::scylla::_macro_internal::RowWriter::make_cell_writer(
                        writer,
                    );
                    match <Option<
                        simple_model_udt,
                    > as ::scylla::_macro_internal::SerializeValue>::serialize(
                        &self.another_field,
                        spec.typ(),
                        sub_writer,
                    ) {
                        ::std::result::Result::Ok(_proof) => {}
                        ::std::result::Result::Err(err) => {
                            return ::std::result::Result::Err(
                                mk_ser_err(::scylla::_macro_internal::BuiltinRowSerializationErrorKind::ColumnSerializationFailed {
                                    name: <_ as ::std::borrow::ToOwned>::to_owned(spec.name()),
                                    err,
                                }),
                            );
                        }
                    }
                    if !visited_flag_another_field {
                        visited_flag_another_field = true;
                        remaining_count -= 1;
                    }
                }
                "the_field" => {
                    let sub_writer = ::scylla::_macro_internal::RowWriter::make_cell_writer(
                        writer,
                    );
                    match <simple_model_udt as ::scylla::_macro_internal::SerializeValue>::serialize(
                        &self.the_field,
                        spec.typ(),
                        sub_writer,
                    ) {
                        ::std::result::Result::Ok(_proof) => {}
                        ::std::result::Result::Err(err) => {
                            return ::std::result::Result::Err(
                                mk_ser_err(::scylla::_macro_internal::BuiltinRowSerializationErrorKind::ColumnSerializationFailed {
                                    name: <_ as ::std::borrow::ToOwned>::to_owned(spec.name()),
                                    err,
                                }),
                            );
                        }
                    }
                    if !visited_flag_the_field {
                        visited_flag_the_field = true;
                        remaining_count -= 1;
                    }
                }
                _ => {
                    return ::std::result::Result::Err(
                        mk_typck_err(::scylla::_macro_internal::BuiltinRowTypeCheckErrorKind::NoColumnWithName {
                            name: <_ as ::std::borrow::ToOwned>::to_owned(spec.name()),
                        }),
                    );
                }
            }
        }
        if remaining_count > 0 {
            if !visited_flag_id {
                return ::std::result::Result::Err(
                    mk_typck_err(::scylla::_macro_internal::BuiltinRowTypeCheckErrorKind::ValueMissingForColumn {
                        name: <_ as ::std::string::ToString>::to_string("id"),
                    }),
                );
            }
            if !visited_flag_another_field {
                return ::std::result::Result::Err(
                    mk_typck_err(::scylla::_macro_internal::BuiltinRowTypeCheckErrorKind::ValueMissingForColumn {
                        name: <_ as ::std::string::ToString>::to_string("another_field"),
                    }),
                );
            }
            if !visited_flag_the_field {
                return ::std::result::Result::Err(
                    mk_typck_err(::scylla::_macro_internal::BuiltinRowTypeCheckErrorKind::ValueMissingForColumn {
                        name: <_ as ::std::string::ToString>::to_string("the_field"),
                    }),
                );
            }
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        ::std::result::Result::Ok(())
    }
    #[inline]
    fn is_empty(&self) -> bool {
        false
    }
}
impl SimpleModelUdtWithTable {
    pub fn find_by_id<'a>(
        id: ::hoover3_database::charybdis::types::Text,
    ) -> charybdis::query::CharybdisQuery<
        'a,
        (::hoover3_database::charybdis::types::Text,),
        Self,
        charybdis::query::ModelRow,
    > {
        <SimpleModelUdtWithTable as charybdis::operations::Find>::find_first(
            "SELECT id, another_field, the_field FROM simple_model_udt_with_table WHERE id = ?",
            (id,),
        )
    }
    pub fn find_first_by_id<'a>(
        id: ::hoover3_database::charybdis::types::Text,
    ) -> charybdis::query::CharybdisQuery<
        'a,
        (::hoover3_database::charybdis::types::Text,),
        Self,
        charybdis::query::ModelRow,
    > {
        <SimpleModelUdtWithTable as charybdis::operations::Find>::find_first(
            "SELECT id, another_field, the_field FROM simple_model_udt_with_table WHERE id = ? LIMIT 1",
            (id,),
        )
    }
    pub fn maybe_find_first_by_id<'a>(
        id: ::hoover3_database::charybdis::types::Text,
    ) -> charybdis::query::CharybdisQuery<
        'a,
        (::hoover3_database::charybdis::types::Text,),
        Self,
        charybdis::query::OptionalModelRow,
    > {
        <SimpleModelUdtWithTable as charybdis::operations::Find>::maybe_find_first(
            "SELECT id, another_field, the_field FROM simple_model_udt_with_table WHERE id = ? LIMIT 1",
            (id,),
        )
    }
    pub fn delete_by_id<'a>(
        id: ::hoover3_database::charybdis::types::Text,
    ) -> charybdis::query::CharybdisQuery<
        'a,
        (::hoover3_database::charybdis::types::Text,),
        Self,
        charybdis::query::ModelMutation,
    > {
        charybdis::query::CharybdisQuery::new(
            "DELETE FROM simple_model_udt_with_table WHERE id = ?",
            charybdis::query::QueryValue::Owned((id,)),
        )
    }
}
impl charybdis::model::BaseModel for SimpleModelUdtWithTable {
    type PrimaryKey = (::hoover3_database::charybdis::types::Text,);
    type PartitionKey = (::hoover3_database::charybdis::types::Text,);
    const DB_MODEL_NAME: &'static str = "simple_model_udt_with_table";
    const FIND_ALL_QUERY: &'static str = "SELECT id, another_field, the_field FROM simple_model_udt_with_table";
    const FIND_BY_PRIMARY_KEY_QUERY: &'static str = "SELECT id, another_field, the_field FROM simple_model_udt_with_table WHERE id = ?";
    const FIND_BY_PARTITION_KEY_QUERY: &'static str = "SELECT id, another_field, the_field FROM simple_model_udt_with_table WHERE id = ?";
    const FIND_FIRST_BY_PARTITION_KEY_QUERY: &'static str = "SELECT id, another_field, the_field FROM simple_model_udt_with_table WHERE id = ? LIMIT 1";
    fn primary_key_values(&self) -> Self::PrimaryKey {
        return (self.id.clone(),);
    }
    fn partition_key_values(&self) -> Self::PartitionKey {
        return (self.id.clone(),);
    }
}
impl charybdis::model::Model for SimpleModelUdtWithTable {
    const INSERT_QUERY: &'static str = "INSERT INTO simple_model_udt_with_table (id, another_field, the_field) VALUES (:id, :another_field, :the_field)";
    const INSERT_IF_NOT_EXIST_QUERY: &'static str = "INSERT INTO simple_model_udt_with_table (id, another_field, the_field) VALUES (:id, :another_field, :the_field) IF NOT EXISTS";
    const UPDATE_QUERY: &'static str = "UPDATE simple_model_udt_with_table SET another_field = :another_field, the_field = :the_field WHERE id = :id";
    const DELETE_QUERY: &'static str = "DELETE FROM simple_model_udt_with_table WHERE id = ?";
    const DELETE_BY_PARTITION_KEY_QUERY: &'static str = "DELETE FROM simple_model_udt_with_table WHERE id = ?";
}
pub(crate) use find_simple_model_udt_with_table_query;
pub(crate) use find_simple_model_udt_with_table;
pub(crate) use find_first_simple_model_udt_with_table;
pub(crate) use update_simple_model_udt_with_table_query;
pub(crate) use partial_simple_model_udt_with_table;
pub(crate) use delete_simple_model_udt_with_table_query;
pub(crate) use delete_simple_model_udt_with_table;
#[allow(non_upper_case_globals)]
const _: () = {
    static __INVENTORY: ::inventory::Node = ::inventory::Node {
        value: &{
            hoover3_database::models::collection::ModelDefinitionStatic {
                table_name: "simple_model_udt_with_table",
                model_name: "SimpleModelUdtWithTable",
                docstring: "Documentation",
                charybdis_code: "/// Documentation\n#[::charybdis::macros::charybdis_model(\n    table_name = simple_model_udt_with_table,\n    partition_keys = [id],\n    clustering_keys = [],\n    global_secondary_indexes = [],\n    local_secondary_indexes = [],\n    static_columns = []\n)]\n#[derive(\n    Debug,\n    Clone,\n    Hash,\n    PartialEq,\n    PartialOrd,\n    ::serde::Serialize,\n    ::serde::Deserialize\n)]\npub struct SimpleModelUdtWithTable {\n    /// Some Field\n    pub id: ::hoover3_database::charybdis::types::Text,\n    /// Other Field\n    pub another_field: Option<simple_model_udt>,\n    /// The Field\n    pub the_field: simple_model_udt,\n}\n",
                fields: &[
                    ::hoover3_database::models::collection::ModelFieldDefinitionStatic {
                        name: "id",
                        field_type: ::hoover3_types::db_schema::DatabaseColumnType::String,
                        docstring: "Some Field",
                        clustering_key: false,
                        partition_key: true,
                        search_store: false,
                        search_index: false,
                        search_facet: false,
                        nullable: false,
                        field_type_original: "String",
                    },
                    ::hoover3_database::models::collection::ModelFieldDefinitionStatic {
                        name: "another_field",
                        field_type: ::hoover3_types::db_schema::DatabaseColumnType::UnspecifiedType,
                        docstring: "Other Field",
                        clustering_key: false,
                        partition_key: false,
                        search_store: false,
                        search_index: false,
                        search_facet: false,
                        nullable: true,
                        field_type_original: "simple_model_udt",
                    },
                    ::hoover3_database::models::collection::ModelFieldDefinitionStatic {
                        name: "the_field",
                        field_type: ::hoover3_types::db_schema::DatabaseColumnType::UnspecifiedType,
                        docstring: "The Field",
                        clustering_key: false,
                        partition_key: false,
                        search_store: false,
                        search_index: false,
                        search_facet: false,
                        nullable: false,
                        field_type_original: "simple_model_udt",
                    },
                ],
            }
        },
        next: ::inventory::core::cell::UnsafeCell::new(
            ::inventory::core::option::Option::None,
        ),
    };
    #[link_section = ".text.startup"]
    unsafe extern "C" fn __ctor() {
        unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
    }
    #[used]
    #[link_section = ".init_array"]
    static __CTOR: unsafe extern "C" fn() = __ctor;
};
impl ::charybdis::callbacks::Callbacks for SimpleModelUdtWithTable {
    /// Error type for the callbacks - we always use anyhow.
    type Error = ::anyhow::Error;
    /// Extension type for the callbacks - see [DatabaseExtraCallbacks].
    type Extension = ::hoover3_database::models::collection::DatabaseExtraCallbacks;
    /// Callback calls the `insert` method on the `DatabaseExtraCallbacks` instance.
    async fn after_insert(
        &mut self,
        _session: &::charybdis::scylla::CachingSession,
        extension: &::hoover3_database::models::collection::DatabaseExtraCallbacks,
    ) -> ::anyhow::Result<()> {
        extension.insert(&[self.clone()]).await
    }
    /// Callback calls the `delete` method on the `DatabaseExtraCallbacks` instance.
    async fn after_delete(
        &mut self,
        _session: &::charybdis::scylla::CachingSession,
        extension: &::hoover3_database::models::collection::DatabaseExtraCallbacks,
    ) -> ::anyhow::Result<()> {
        extension.delete(&[self.clone()]).await
    }
}
impl SimpleModelUdtWithTable {
    /// Compute a stable hash of a row's primary key, and concatenate it with table name.
    pub fn row_pk_hash(&self) -> String {
        use ::charybdis::model::BaseModel;
        ::hoover3_database::models::collection::row_pk_hash::<
            SimpleModelUdtWithTable,
        >(&self.primary_key_values())
    }
    /// Get a JSON representation of a row's primary key.
    pub fn row_pk_json(&self) -> ::anyhow::Result<::serde_json::Value> {
        use ::charybdis::model::BaseModel;
        Ok(::serde_json::to_value(&self.primary_key_values())?)
    }
}
