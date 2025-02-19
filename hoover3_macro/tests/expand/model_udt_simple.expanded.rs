use hoover3_macro::udt_model;
use hoover3_types::db_schema::Timestamp;
use serde::Serialize;
/// Documentation
pub struct SimpleModelUdt {
    /// Some Field
    pub id: ::charybdis::types::Text,
    /// Other Field
    pub another_field: Option<::charybdis::types::Int>,
    /// Timestamp field
    pub created_at: ::charybdis::types::Timestamp,
}
#[automatically_derived]
impl ::core::fmt::Debug for SimpleModelUdt {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "SimpleModelUdt",
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
impl ::core::clone::Clone for SimpleModelUdt {
    #[inline]
    fn clone(&self) -> SimpleModelUdt {
        SimpleModelUdt {
            id: ::core::clone::Clone::clone(&self.id),
            another_field: ::core::clone::Clone::clone(&self.another_field),
            created_at: ::core::clone::Clone::clone(&self.created_at),
        }
    }
}
#[automatically_derived]
impl ::core::hash::Hash for SimpleModelUdt {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.id, state);
        ::core::hash::Hash::hash(&self.another_field, state);
        ::core::hash::Hash::hash(&self.created_at, state)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for SimpleModelUdt {}
#[automatically_derived]
impl ::core::cmp::PartialEq for SimpleModelUdt {
    #[inline]
    fn eq(&self, other: &SimpleModelUdt) -> bool {
        self.id == other.id && self.another_field == other.another_field
            && self.created_at == other.created_at
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for SimpleModelUdt {
    #[inline]
    fn partial_cmp(
        &self,
        other: &SimpleModelUdt,
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SimpleModelUdt {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "SimpleModelUdt",
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
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SimpleModelUdt {
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
                marker: _serde::__private::PhantomData<SimpleModelUdt>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SimpleModelUdt;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SimpleModelUdt",
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
                        ::charybdis::types::Text,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SimpleModelUdt with 3 elements",
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
                                    &"struct SimpleModelUdt with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        ::charybdis::types::Timestamp,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct SimpleModelUdt with 3 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SimpleModelUdt {
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
                        ::charybdis::types::Text,
                    > = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<
                        Option<::charybdis::types::Int>,
                    > = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<
                        ::charybdis::types::Timestamp,
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
                                        ::charybdis::types::Text,
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
                                        ::charybdis::types::Timestamp,
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
                    _serde::__private::Ok(SimpleModelUdt {
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
                "SimpleModelUdt",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SimpleModelUdt>,
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
> scylla::_macro_internal::DeserializeValue<'lifetime, 'lifetime_> for SimpleModelUdt {
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
                        <::charybdis::types::Text as scylla::_macro_internal::DeserializeValue<
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
                        <::charybdis::types::Timestamp as scylla::_macro_internal::DeserializeValue<
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
                        <::charybdis::types::Text as scylla::_macro_internal::DeserializeValue<
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
                        <::charybdis::types::Timestamp as scylla::_macro_internal::DeserializeValue<
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
impl ::scylla::_macro_internal::SerializeValue for SimpleModelUdt {
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
                    match <::charybdis::types::Text as ::scylla::_macro_internal::SerializeValue>::serialize(
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
                    match <::charybdis::types::Timestamp as ::scylla::_macro_internal::SerializeValue>::serialize(
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
            ::hoover3_types::db_schema::UdtModelDefinitionStatic {
                udt_name: "simple_model_udt",
                model_name: "SimpleModelUdt",
                docstring: "Documentation",
                charybdis_code: "/// Documentation\n#[::charybdis::macros::charybdis_udt_model(type_name = simple_model_udt)]\n#[derive(\n    Debug,\n    Clone,\n    Hash,\n    PartialEq,\n    PartialOrd,\n    ::serde::Serialize,\n    ::serde::Deserialize\n)]\npub struct SimpleModelUdt {\n    /// Some Field\n    pub id: ::charybdis::types::Text,\n    /// Other Field\n    pub another_field: Option<::charybdis::types::Int>,\n    /// Timestamp field\n    pub created_at: ::charybdis::types::Timestamp,\n}\n",
                fields: &[
                    ::hoover3_types::db_schema::ModelFieldDefinitionStatic {
                        name: "id",
                        field_type: ::hoover3_types::db_schema::DatabaseColumnType::String,
                        docstring: "Some Field",
                        clustering_key: false,
                        partition_key: false,
                        search_store: false,
                        search_index: false,
                        search_facet: false,
                        nullable: false,
                    },
                    ::hoover3_types::db_schema::ModelFieldDefinitionStatic {
                        name: "another_field",
                        field_type: ::hoover3_types::db_schema::DatabaseColumnType::Int32,
                        docstring: "Other Field",
                        clustering_key: false,
                        partition_key: false,
                        search_store: false,
                        search_index: false,
                        search_facet: false,
                        nullable: true,
                    },
                    ::hoover3_types::db_schema::ModelFieldDefinitionStatic {
                        name: "created_at",
                        field_type: ::hoover3_types::db_schema::DatabaseColumnType::Timestamp,
                        docstring: "Timestamp field",
                        clustering_key: false,
                        partition_key: false,
                        search_store: false,
                        search_index: false,
                        search_facet: false,
                        nullable: false,
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
