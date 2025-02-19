use hoover3_types::{
    db_schema::{DatabaseColumnType, ModelDefinition, ModelFieldDefinition},
    identifier::DatabaseIdentifier,
};
use proc_macro2::TokenStream;

/// Macro extracts fields and their configuration, then adds derives and charybdis attributes.
/// It also adds the ModelDefinition to the inventory.
pub fn model( item: TokenStream) -> TokenStream {
    let mut model_def = parse_model(item.clone());

    // Parse the original struct
    let item_struct = syn::parse2::<syn::ItemStruct>(item).expect("parse model struct");
    let item_struct = generate_new_item_struct(&model_def, &item_struct);

    let charybdis_code = prettyplease::unparse(&syn::parse_quote!(#item_struct));
    model_def.charybdis_code = charybdis_code;
    let inventory_submit = generate_inventory_submit(&model_def);

    quote::quote! {
        #item_struct
        #inventory_submit
    }
}

/// Generates a new struct item with charybdis attributes.
fn generate_new_item_struct(
    model_def: &ModelDefinition,
    item_struct: &syn::ItemStruct,
) -> syn::ItemStruct {
    let mut item_struct = item_struct.clone();
    let table_name = model_def.table_name.clone();
    if let Err(e) = DatabaseIdentifier::new(&table_name) {
        panic!("invalid model/table name: {}", e);
    }
    let table_name_id = syn::Ident::new(&table_name, proc_macro2::Span::call_site());
    // Get partition and clustering keys
    let partition_keys: Vec<syn::Ident> = model_def
        .fields
        .iter()
        .filter(|f| f.partition_key)
        .map(|f| syn::Ident::new(&f.name, proc_macro2::Span::call_site()))
        .collect();

    let clustering_keys: Vec<syn::Ident> = model_def
        .fields
        .iter()
        .filter(|f| f.clustering_key)
        .map(|f| syn::Ident::new(&f.name, proc_macro2::Span::call_site()))
        .collect();

    // Add charybdis_model attribute
    let charybdis_attr = syn::parse_quote! {
        #[::charybdis::macros::charybdis_model(
            table_name = #table_name_id,
            partition_keys = [#(#partition_keys),*],
            clustering_keys = [#(#clustering_keys),*],
            global_secondary_indexes = [],
            local_secondary_indexes = [],
            static_columns = []
        )]
    };

    item_struct.attrs.push(charybdis_attr);
    item_struct.attrs.push(syn::parse_quote! {
        #[derive(Debug, Clone, Hash, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize, ::hoover3_macro::Hoover3_Macro_Model_Helper)]
    });

    // Add charybdis column type attributes to fields
    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        for field in fields.named.iter_mut() {
            let (column_type, nullable) = get_field_type(&field.ty);
            let column_type_str = column_type.to_scylla_type().expect("unsupported type");
            let column_type = syn::Ident::new(&column_type_str, proc_macro2::Span::call_site());

            // let charybdis_field_attr = syn::parse_quote! {
            //     #[charybdis(column_type = #column_type_str)]
            // };
            let charybdis_field_type = if nullable {
                syn::parse_quote! {
                    Option<::charybdis::types::#column_type>
                }
            } else {
                syn::parse_quote! {
                    ::charybdis::types::#column_type
                }
            };
            field.ty = charybdis_field_type;
            // field.attrs.push(charybdis_field_attr);
            // field.ty = charybdis_field_type;
        }
    }
    // for each field, remove any #[model(...)] attributes
    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        for field in fields.named.iter_mut() {
            field.attrs.retain(|attr| !attr.path().is_ident("model"));
        }
    }

    item_struct
}

/// Generates inventory::submit! call for model definition.
fn generate_inventory_submit(model_def: &ModelDefinition) -> TokenStream {
    let table_name = model_def.table_name.clone();
    let model_name = model_def.model_name.clone();
    let docstring = model_def.docstring.clone();
    let charybdis_code = model_def.charybdis_code.clone();
    let fields = model_def
        .fields
        .iter()
        .map(|f| {
            let name = f.name.clone();
            let field_type = f.field_type.clone();
            let docstring = f.docstring.clone();
            let field_type_str = match field_type {
                DatabaseColumnType::String => "String",
                DatabaseColumnType::Int8 => "Int8",
                DatabaseColumnType::Int16 => "Int16",
                DatabaseColumnType::Int32 => "Int32",
                DatabaseColumnType::Int64 => "Int64",
                DatabaseColumnType::Float => "Float",
                DatabaseColumnType::Double => "Double",
                DatabaseColumnType::Boolean => "Boolean",
                DatabaseColumnType::Timestamp => "Timestamp",
                _ => {
                    panic!("unsupported field type: {:?}", field_type);
                }
            };
            let field_type_str = format!(
                "::hoover3_types::db_schema::DatabaseColumnType::{}",
                field_type_str
            );
            let field_type = syn::parse_str::<syn::Type>(&field_type_str).unwrap();
            let clustering_key = f.clustering_key;
            let partition_key = f.partition_key;
            let search_store = f.search_store;
            let search_index = f.search_index;
            let search_facet = f.search_facet;
            let nullable = f.nullable;
            quote::quote! {
                ::hoover3_types::db_schema::ModelFieldDefinitionStatic {
                    name: #name,
                    field_type: #field_type,
                    docstring: #docstring,
                    clustering_key: #clustering_key,
                    partition_key: #partition_key,
                    search_store: #search_store,
                    search_index: #search_index,
                    search_facet: #search_facet,
                    nullable: #nullable,
                }
            }
        })
        .collect::<Vec<_>>();

    quote::quote! {
        ::hoover3_types::inventory::submit!{::hoover3_types::db_schema::ModelDefinitionStatic {
            table_name: #table_name,
            model_name: #model_name,
            docstring: #docstring,
            charybdis_code: #charybdis_code,
            fields: &[#(#fields),*],
        }}
    }
}

/// Darling helper struct: parses field attributes.
#[derive(darling::FromMeta)]
struct ModelFieldAttr {
    primary: Option<PrimaryAttr>,
    search: Option<SearchAttr>,
}

/// Darling helper struct: parses field primary key attributes.
#[derive(darling::FromMeta)]
struct PrimaryAttr {
    #[darling(default)]
    partition: bool,
    #[darling(default)]
    clustering: bool,
}

#[derive(darling::FromMeta)]
struct SearchAttr {
    #[darling(default)]
    store: bool,
    #[darling(default)]
    index: bool,
    #[darling(default)]
    facet: bool,
}

/// Parses ModelDefinition instance from struct code.
fn parse_model(item: TokenStream) -> ModelDefinition {
    let item_struct = syn::parse2::<syn::ItemStruct>(item).expect("parse model struct");
    let model_name = item_struct.ident.to_string();

    // Extract struct docstring
    let struct_docstring = item_struct
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .map(|attr| {
            if let syn::Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) = &nv.value
                {
                    lit_str.value()
                } else {
                    panic!("invalid doc attribute")
                }
            } else {
                panic!("invalid doc attribute")
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    if struct_docstring.is_empty() {
        panic!("model {} is missing documentation", model_name);
    }

    let table_name = {
        use convert_case::{Case, Casing};
        model_name.to_case(Case::Snake)
    };

    let syn::Fields::Named(syn::FieldsNamed { named: fields, .. }) = item_struct.fields else {
        panic!("model fields are not named");
    };
    let fields: Vec<_> = fields
        .iter()
        .map(|field| {
            let syn::Field {
                attrs,
                vis: syn::Visibility::Public(_),
                ident,
                ty,
                ..
            } = field
            else {
                panic!("field is not public");
            };

            // Extract field docstring
            let field_docstring = attrs
                .iter()
                .filter(|attr| attr.path().is_ident("doc"))
                .map(|attr| {
                    if let syn::Meta::NameValue(nv) = &attr.meta {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }) = &nv.value
                        {
                            lit_str.value()
                        } else {
                            panic!("invalid doc attribute")
                        }
                    } else {
                        panic!("invalid doc attribute")
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");

            let field_name = ident.as_ref().unwrap().to_string();
            if field_docstring.is_empty() {
                panic!("field {} is missing documentation", field_name);
            }

            let mut partition_key = false;
            let mut clustering_key = false;
            let mut search_store = false;
            let mut search_index = false;
            let mut search_facet = false;

            // Get field type and nullable status
            let (field_type, nullable) = get_field_type(ty);

            // Parse field attributes
            for attr in attrs {
                if attr.path().is_ident("model") {
                    let model_attr: ModelFieldAttr = darling::FromMeta::from_meta(&attr.meta)
                        .expect("failed to parse model attribute");
                    if let Some(primary) = model_attr.primary {
                        partition_key = primary.partition;
                        clustering_key = primary.clustering;
                    }
                    if let Some(search) = model_attr.search {
                        search_facet = search.facet;
                        search_index = search.index || search_facet;
                        search_store = search.store || search_index;
                    }
                }
            }

            ModelFieldDefinition {
                name: field_name,
                docstring: field_docstring.trim().to_string(),
                field_type: field_type,
                clustering_key,
                partition_key,
                search_store,
                search_index,
                search_facet,
                nullable,
            }
        })
        .collect();

    ModelDefinition {
        table_name,
        model_name,
        docstring: struct_docstring.trim().to_string(),
        fields,
        charybdis_code: "".to_string(),
    }
}

fn get_field_type(ty: &syn::Type) -> (DatabaseColumnType, bool) {
    match ty {
        syn::Type::Path(syn::TypePath { path, .. }) => {
            let segments = &path.segments;
            let last_segment = segments.last().unwrap();

            // Check if it's an Option type
            if last_segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                        let (inner_type, _) = get_field_type(inner_type);
                        return (inner_type, true);
                    }
                }
                panic!("invalid Option type");
            }

            // Handle non-Option types
            let type_name = last_segment.ident.to_string();
            let column_type = match type_name.as_str() {
                "String" => DatabaseColumnType::String,
                "i8" => DatabaseColumnType::Int8,
                "i16" => DatabaseColumnType::Int16,
                "i32" => DatabaseColumnType::Int32,
                "i64" => DatabaseColumnType::Int64,
                "f32" => DatabaseColumnType::Float,
                "f64" => DatabaseColumnType::Double,
                "bool" => DatabaseColumnType::Boolean,
                "Timestamp" => DatabaseColumnType::Timestamp,
                _ => panic!("unsupported field type: {:?}", ty),
            };
            (column_type, false)
        }
        _ => panic!("unsupported field type: {:?}", ty),
    }
}

#[test]
fn test_parse_model() {
    let item = quote::quote! {
        /// This is a test model
        struct MyModel {
            /// Doc One
            #[model(primary(partition))]
            pub pk: String,
            /// Doc Two
            #[model(primary(clustering))]
            pub ck: i64,
            /// Doc Three
            #[model(search(index))]
            pub field1: i32,
            #[doc = "Doc Four"]
            #[model(search(store))]
            pub field2: Option<i8>,
            #[doc = "Doc Five"]
            #[model(search(facet))]
            pub field3: hoover3_types::db_schema::Timestamp,
            #[doc = "Doc Six"]
            #[model(primary(clustering), search(store))]
            pub field4: i16,
        }
    };
    let model_def = parse_model(item);

    assert_eq!(
        model_def,
        ModelDefinition {
            table_name: "my_model".to_string(),
            model_name: "MyModel".to_string(),
            docstring: "This is a test model".to_string(),
            charybdis_code: "".to_string(),
            fields: vec![
                ModelFieldDefinition {
                    name: "pk".to_string(),
                    field_type: DatabaseColumnType::String,
                    clustering_key: false,
                    partition_key: true,
                    search_store: false,
                    search_index: false,
                    search_facet: false,
                    nullable: false,
                    docstring: "Doc One".to_string(),
                },
                ModelFieldDefinition {
                    name: "ck".to_string(),
                    field_type: DatabaseColumnType::Int64,
                    clustering_key: true,
                    partition_key: false,
                    search_store: false,
                    search_index: false,
                    search_facet: false,
                    nullable: false,
                    docstring: "Doc Two".to_string(),
                },
                ModelFieldDefinition {
                    name: "field1".to_string(),
                    field_type: DatabaseColumnType::Int32,
                    clustering_key: false,
                    partition_key: false,
                    search_store: true,
                    search_index: true,
                    search_facet: false,
                    nullable: false,
                    docstring: "Doc Three".to_string(),
                },
                ModelFieldDefinition {
                    name: "field2".to_string(),
                    field_type: DatabaseColumnType::Int8,
                    clustering_key: false,
                    partition_key: false,
                    search_store: true,
                    search_index: false,
                    search_facet: false,
                    nullable: true,
                    docstring: "Doc Four".to_string(),
                },
                ModelFieldDefinition {
                    name: "field3".to_string(),
                    field_type: DatabaseColumnType::Timestamp,
                    clustering_key: false,
                    partition_key: false,
                    search_store: true,
                    search_index: true,
                    search_facet: true,
                    nullable: false,
                    docstring: "Doc Five".to_string(),
                },
                ModelFieldDefinition {
                    name: "field4".to_string(),
                    field_type: DatabaseColumnType::Int16,
                    clustering_key: true,
                    partition_key: false,
                    search_store: true,
                    search_index: false,
                    search_facet: false,
                    nullable: false,
                    docstring: "Doc Six".to_string(),
                },
            ],
        }
    );
}

#[test]
fn test_generate_new_item_struct() {
    use pretty_assertions::assert_eq;
    let input_struct = quote::quote! {
        /// Test model documentation
        pub struct SimpleModel {
            /// Primary key field
            #[model(primary(partition))]
            pub id: String,
            /// Other Field
            #[model(primary(clustering))]
            pub other_field: i64,
            /// Another field
            #[model(primary(partition))]
            pub another_field: i32,
            /// Timestamp field
            pub created_at: hoover3_types::db_schema::Timestamp,
        }
    };

    let model_def = parse_model(input_struct.clone());
    let input_struct =
        syn::parse2::<syn::ItemStruct>(input_struct.clone()).expect("parse model struct");
    let result = generate_new_item_struct(&model_def, &input_struct);
    let expected = quote::quote! {
        /// Test model documentation
        #[::charybdis::macros::charybdis_model(
            table_name = simple_model,
            partition_keys = [id, another_field],
            clustering_keys = [other_field],
            global_secondary_indexes = [],
            local_secondary_indexes = [],
            static_columns = []
        )]
        #[derive(
            Debug,
            Clone,
            Hash,
            PartialEq,
            PartialOrd,
            ::serde::Serialize,
            ::serde::Deserialize,
            ::hoover3_macro::Hoover3_Macro_Model_Helper
        )]
        pub struct SimpleModel {
            /// Primary key field
            pub id: ::charybdis::types::Text,
            /// Other Field
            pub other_field: ::charybdis::types::BigInt,
            /// Another field
            pub another_field: ::charybdis::types::Int,
            /// Timestamp field
            pub created_at: ::charybdis::types::Timestamp,
        }
    };

    let code_result = prettyplease::unparse(&syn::parse_quote!(#result));
    let code_expected = prettyplease::unparse(&syn::parse_quote!(#expected));

    println!("EXPECTED: {}", code_expected);
    println!("RESULT: {}", code_result);
    assert_eq!(code_expected, code_result);
}

#[test]
fn test_generate_inventory_submit() {
    use pretty_assertions::assert_eq;

    let model_def = ModelDefinition {
        table_name: "my_model".to_string(),
        model_name: "MyModel".to_string(),
        docstring: "This is a test model".to_string(),
        charybdis_code: "".to_string(),
        fields: vec![
            ModelFieldDefinition {
                name: "pk".to_string(),
                field_type: DatabaseColumnType::String,
                clustering_key: false,
                partition_key: true,
                search_store: false,
                search_index: false,
                search_facet: false,
                docstring: "a".to_string(),
                nullable: false,
            },
            ModelFieldDefinition {
                name: "ck".to_string(),
                field_type: DatabaseColumnType::Int64,
                clustering_key: true,
                partition_key: false,
                search_store: false,
                search_index: false,
                search_facet: false,
                docstring: "b".to_string(),
                nullable: false,
            },
        ],
    };
    let result = generate_inventory_submit(&model_def);

    let expected = quote::quote! {
        ::hoover3_types::inventory::submit!{::hoover3_types::db_schema::ModelDefinitionStatic {
            table_name: "my_model",
            model_name: "MyModel",
            docstring: "This is a test model",
            charybdis_code: "",
            fields: & [
                ::hoover3_types::db_schema::ModelFieldDefinitionStatic {
                    name: "pk",
                    field_type: ::hoover3_types::db_schema::DatabaseColumnType::String,
                    docstring: "a",
                    clustering_key: false,
                    partition_key: true,
                    search_store: false,
                    search_index: false,
                    search_facet: false,
                    nullable: false,
                },
                ::hoover3_types::db_schema::ModelFieldDefinitionStatic {
                    name: "ck",
                    field_type: ::hoover3_types::db_schema::DatabaseColumnType::Int64,
                    docstring: "b",
                    clustering_key: true,
                    partition_key: false,
                    search_store: false,
                    search_index: false,
                    search_facet: false,
                    nullable: false,
                }
            ],
        }}
    };

    let result_str = prettyplease::unparse(&syn::parse_quote!(#result));
    let expected_str = prettyplease::unparse(&syn::parse_quote!(#expected));
    println!("EXPECTED: {}", expected_str);
    println!("RESULT: {}", result_str);
    assert_eq!(result_str, expected_str);
}

#[test]
fn test_model_macro() {
    use pretty_assertions::assert_eq;
    let item = quote::quote! {
        /// This is a test model
        struct MyModel {
            /// Doc One
            #[model(primary(partition))]
            pub pk: String,
            /// Doc Two
            pub created_at: Option<hoover3_types::db_schema::Timestamp>,
        }
    };
    let result = model(item);
    let expected = quote::quote! {
        /// This is a test model
        #[::charybdis::macros::charybdis_model(
            table_name = my_model,
            partition_keys = [pk],
            clustering_keys = [],
            global_secondary_indexes = [],
            local_secondary_indexes = [],
            static_columns = []
        )]
        #[derive(Debug, Clone, Hash, PartialEq, PartialOrd, ::serde::Serialize, ::serde::Deserialize, ::hoover3_macro::Hoover3_Macro_Model_Helper)]
        struct MyModel {
            /// Doc One
            pub pk: ::charybdis::types::Text,
            /// Doc Two
            pub created_at: Option<::charybdis::types::Timestamp>,
        }

        ::hoover3_types::inventory::submit! {
            ::hoover3_types::db_schema::ModelDefinitionStatic { table_name : "my_model",
            model_name : "MyModel", docstring : "This is a test model", charybdis_code :
            "/// This is a test model\n#[::charybdis::macros::charybdis_model(\n    table_name = my_model,\n    partition_keys = [pk],\n    clustering_keys = [],\n    global_secondary_indexes = [],\n    local_secondary_indexes = [],\n    static_columns = []\n)]\n#[derive(\n    Debug,\n    Clone,\n    Hash,\n    PartialEq,\n    PartialOrd,\n    ::serde::Serialize,\n    ::serde::Deserialize,\n    ::hoover3_macro::Hoover3_Macro_Model_Helper\n)]\nstruct MyModel {\n    /// Doc One\n    pub pk: ::charybdis::types::Text,\n    /// Doc Two\n    pub created_at: Option<::charybdis::types::Timestamp>,\n}\n",
            fields : & [::hoover3_types::db_schema::ModelFieldDefinitionStatic { name : "pk",
            field_type : ::hoover3_types::db_schema::DatabaseColumnType::String, docstring :
            "Doc One", clustering_key : false, partition_key : true, search_store : false,
            search_index : false, search_facet : false, nullable : false, },
            ::hoover3_types::db_schema::ModelFieldDefinitionStatic { name : "created_at",
            field_type : ::hoover3_types::db_schema::DatabaseColumnType::Timestamp, docstring :
            "Doc Two", clustering_key : false, partition_key : false, search_store : false,
            search_index : false, search_facet : false, nullable : true, }], }
        }

    };
    let result_str = prettyplease::unparse(&syn::parse_quote!(#result));
    let expected_str = prettyplease::unparse(&syn::parse_quote!(#expected));
    println!("EXPECTED: {}", expected_str);
    println!("RESULT: {}", result_str);
    assert_eq!(result_str, expected_str);
}
