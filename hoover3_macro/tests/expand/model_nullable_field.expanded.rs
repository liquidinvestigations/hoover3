use hoover3_macro::model;
use hoover3_types::db_schema::Timestamp;
use serde::Serialize;
/// Documentation
pub struct SimpleModel {
    /// Primary key field
    pub id: ::charybdis::types::Text,
    /// Nullable Field
    pub created_at: Option<::charybdis::types::Timestamp>,
}
#[automatically_derived]
impl ::core::fmt::Debug for SimpleModel {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "SimpleModel",
            "id",
            &self.id,
            "created_at",
            &&self.created_at,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for SimpleModel {
    #[inline]
    fn clone(&self) -> SimpleModel {
        SimpleModel {
            id: ::core::clone::Clone::clone(&self.id),
            created_at: ::core::clone::Clone::clone(&self.created_at),
        }
    }
}
#[automatically_derived]
impl ::core::hash::Hash for SimpleModel {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.id, state);
        ::core::hash::Hash::hash(&self.created_at, state)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for SimpleModel {}
#[automatically_derived]
impl ::core::cmp::PartialEq for SimpleModel {
    #[inline]
    fn eq(&self, other: &SimpleModel) -> bool {
        self.id == other.id && self.created_at == other.created_at
    }
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for SimpleModel {
    #[inline]
    fn partial_cmp(
        &self,
        other: &SimpleModel,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&self.id, &other.id) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                ::core::cmp::PartialOrd::partial_cmp(&self.created_at, &other.created_at)
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
    impl _serde::Serialize for SimpleModel {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "SimpleModel",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "id",
                &self.id,
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
    impl<'de> _serde::Deserialize<'de> for SimpleModel {
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
                        "created_at" => _serde::__private::Ok(__Field::__field1),
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
                        b"created_at" => _serde::__private::Ok(__Field::__field1),
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
                marker: _serde::__private::PhantomData<SimpleModel>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SimpleModel;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SimpleModel",
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
                                    &"struct SimpleModel with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        Option<::charybdis::types::Timestamp>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SimpleModel with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SimpleModel {
                        id: __field0,
                        created_at: __field1,
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
                        Option<::charybdis::types::Timestamp>,
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
                                            "created_at",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        Option<::charybdis::types::Timestamp>,
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
                            _serde::__private::de::missing_field("created_at")?
                        }
                    };
                    _serde::__private::Ok(SimpleModel {
                        id: __field0,
                        created_at: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["id", "created_at"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SimpleModel",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SimpleModel>,
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
> scylla::_macro_internal::DeserializeRow<'lifetime, 'lifetime_> for SimpleModel {
    fn type_check(
        specs: &[scylla::_macro_internal::ColumnSpec],
    ) -> ::std::result::Result<(), scylla::_macro_internal::TypeCheckError> {
        let mut remaining_required_fields: ::std::primitive::usize = 2usize;
        let mut visited_id = false;
        let mut visited_created_at = false;
        let column_types_iter = || {
            specs.iter().map(|spec| ::std::clone::Clone::clone(spec.typ()).into_owned())
        };
        for (column_index, spec) in specs.iter().enumerate() {
            match spec.name() {
                "id" => {
                    if !visited_id {
                        <::charybdis::types::Text as scylla::_macro_internal::DeserializeValue<
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
                "created_at" => {
                    if !visited_created_at {
                        <Option<
                            ::charybdis::types::Timestamp,
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
                                            "created_at",
                                        ),
                                        err,
                                    },
                                )
                            })?;
                        visited_created_at = true;
                        remaining_required_fields -= 1;
                    } else {
                        return ::std::result::Result::Err(
                            scylla::_macro_internal::mk_row_typck_err::<
                                Self,
                            >(
                                column_types_iter(),
                                scylla::_macro_internal::DeserBuiltinRowTypeCheckErrorKind::DuplicatedColumn {
                                    column_index,
                                    column_name: "created_at",
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
                if !visited_created_at {
                    missing_fields.push("created_at");
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
        let mut f_created_at = ::std::option::Option::None;
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
                        <::charybdis::types::Text as scylla::_macro_internal::DeserializeValue<
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
                "created_at" => {
                    if !f_created_at.is_none() {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "duplicated column {0} - type check should have prevented this!",
                                    "f_created_at",
                                ),
                            );
                        }
                    }
                    f_created_at = ::std::option::Option::Some(
                        <Option<
                            ::charybdis::types::Timestamp,
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
            created_at: f_created_at
                .unwrap_or_else(|| {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "column {0} missing in DB row - type check should have prevented this!",
                            "created_at",
                        ),
                    );
                }),
        })
    }
}
#[automatically_derived]
impl ::scylla::_macro_internal::SerializeRow for SimpleModel {
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
        let mut visited_flag_created_at = false;
        let mut remaining_count = 2usize;
        for spec in ctx.columns() {
            match spec.name() {
                "id" => {
                    let sub_writer = ::scylla::_macro_internal::RowWriter::make_cell_writer(
                        writer,
                    );
                    match <::charybdis::types::Text as ::scylla::_macro_internal::SerializeValue>::serialize(
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
                "created_at" => {
                    let sub_writer = ::scylla::_macro_internal::RowWriter::make_cell_writer(
                        writer,
                    );
                    match <Option<
                        ::charybdis::types::Timestamp,
                    > as ::scylla::_macro_internal::SerializeValue>::serialize(
                        &self.created_at,
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
                    if !visited_flag_created_at {
                        visited_flag_created_at = true;
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
            if !visited_flag_created_at {
                return ::std::result::Result::Err(
                    mk_typck_err(::scylla::_macro_internal::BuiltinRowTypeCheckErrorKind::ValueMissingForColumn {
                        name: <_ as ::std::string::ToString>::to_string("created_at"),
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
impl SimpleModel {
    pub fn find_by_id<'a>(
        id: ::charybdis::types::Text,
    ) -> charybdis::query::CharybdisQuery<
        'a,
        (::charybdis::types::Text,),
        Self,
        charybdis::query::ModelRow,
    > {
        <SimpleModel as charybdis::operations::Find>::find_first(
            "SELECT id, created_at FROM simple_model WHERE id = ?",
            (id,),
        )
    }
    pub fn find_first_by_id<'a>(
        id: ::charybdis::types::Text,
    ) -> charybdis::query::CharybdisQuery<
        'a,
        (::charybdis::types::Text,),
        Self,
        charybdis::query::ModelRow,
    > {
        <SimpleModel as charybdis::operations::Find>::find_first(
            "SELECT id, created_at FROM simple_model WHERE id = ? LIMIT 1",
            (id,),
        )
    }
    pub fn maybe_find_first_by_id<'a>(
        id: ::charybdis::types::Text,
    ) -> charybdis::query::CharybdisQuery<
        'a,
        (::charybdis::types::Text,),
        Self,
        charybdis::query::OptionalModelRow,
    > {
        <SimpleModel as charybdis::operations::Find>::maybe_find_first(
            "SELECT id, created_at FROM simple_model WHERE id = ? LIMIT 1",
            (id,),
        )
    }
    pub fn delete_by_id<'a>(
        id: ::charybdis::types::Text,
    ) -> charybdis::query::CharybdisQuery<
        'a,
        (::charybdis::types::Text,),
        Self,
        charybdis::query::ModelMutation,
    > {
        charybdis::query::CharybdisQuery::new(
            "DELETE FROM simple_model WHERE id = ?",
            charybdis::query::QueryValue::Owned((id,)),
        )
    }
}
impl charybdis::model::BaseModel for SimpleModel {
    type PrimaryKey = (::charybdis::types::Text,);
    type PartitionKey = (::charybdis::types::Text,);
    const DB_MODEL_NAME: &'static str = "simple_model";
    const FIND_ALL_QUERY: &'static str = "SELECT id, created_at FROM simple_model";
    const FIND_BY_PRIMARY_KEY_QUERY: &'static str = "SELECT id, created_at FROM simple_model WHERE id = ?";
    const FIND_BY_PARTITION_KEY_QUERY: &'static str = "SELECT id, created_at FROM simple_model WHERE id = ?";
    const FIND_FIRST_BY_PARTITION_KEY_QUERY: &'static str = "SELECT id, created_at FROM simple_model WHERE id = ? LIMIT 1";
    fn primary_key_values(&self) -> Self::PrimaryKey {
        return (self.id.clone(),);
    }
    fn partition_key_values(&self) -> Self::PartitionKey {
        return (self.id.clone(),);
    }
}
impl charybdis::model::Model for SimpleModel {
    const INSERT_QUERY: &'static str = "INSERT INTO simple_model (id, created_at) VALUES (:id, :created_at)";
    const INSERT_IF_NOT_EXIST_QUERY: &'static str = "INSERT INTO simple_model (id, created_at) VALUES (:id, :created_at) IF NOT EXISTS";
    const UPDATE_QUERY: &'static str = "UPDATE simple_model SET created_at = :created_at WHERE id = :id";
    const DELETE_QUERY: &'static str = "DELETE FROM simple_model WHERE id = ?";
    const DELETE_BY_PARTITION_KEY_QUERY: &'static str = "DELETE FROM simple_model WHERE id = ?";
}
pub(crate) use find_simple_model_query;
pub(crate) use find_simple_model;
pub(crate) use find_first_simple_model;
pub(crate) use update_simple_model_query;
pub(crate) use partial_simple_model;
pub(crate) use delete_simple_model_query;
pub(crate) use delete_simple_model;
#[allow(non_upper_case_globals)]
const _: () = {
    static __INVENTORY: ::inventory::Node = ::inventory::Node {
        value: &{
            ::hoover3_types::db_schema::ModelDefinitionStatic {
                table_name: "simple_model",
                model_name: "SimpleModel",
                docstring: "Documentation",
                charybdis_code: "/// Documentation\n#[::charybdis::macros::charybdis_model(\n    table_name = simple_model,\n    partition_keys = [id],\n    clustering_keys = [],\n    global_secondary_indexes = [],\n    local_secondary_indexes = [],\n    static_columns = []\n)]\n#[derive(\n    Debug,\n    Clone,\n    Hash,\n    PartialEq,\n    PartialOrd,\n    ::serde::Serialize,\n    ::serde::Deserialize\n)]\npub struct SimpleModel {\n    /// Primary key field\n    pub id: ::charybdis::types::Text,\n    /// Nullable Field\n    pub created_at: Option<::charybdis::types::Timestamp>,\n}\n",
                fields: &[
                    ::hoover3_types::db_schema::ModelFieldDefinitionStatic {
                        name: "id",
                        field_type: ::hoover3_types::db_schema::DatabaseColumnType::String,
                        docstring: "Primary key field",
                        clustering_key: false,
                        partition_key: true,
                        search_store: false,
                        search_index: false,
                        search_facet: false,
                        nullable: false,
                        field_type_original: "String",
                    },
                    ::hoover3_types::db_schema::ModelFieldDefinitionStatic {
                        name: "created_at",
                        field_type: ::hoover3_types::db_schema::DatabaseColumnType::Timestamp,
                        docstring: "Nullable Field",
                        clustering_key: false,
                        partition_key: false,
                        search_store: false,
                        search_index: false,
                        search_facet: false,
                        nullable: true,
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
