use std::collections::{BTreeMap, BTreeSet};

use hoover3_types::{
    db_schema::{
        DatabaseColumn, DatabaseColumnType, DatabaseTable, EdgeStoreImplementation, GraphEdgeId,
        GraphEdgeSchemaDynamic, GraphEdgeTypeDynamic, ModelDefinition, ModelFieldDefinition,
        ScyllaDatabaseSchema, UdtModelDefinition,
    },
    identifier::DatabaseIdentifier,
};

/// Static version of ModelDefinition - used for compile-time inventory.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub struct ModelDefinitionStatic {
    pub table_name: &'static str,
    pub model_name: &'static str,
    pub fields: &'static [ModelFieldDefinitionStatic],
    pub docstring: &'static str,
    pub charybdis_code: &'static str,
}

impl ModelDefinitionStatic {
    /// Convert a static model definition to a dynamic model definition.
    pub fn to_owned(&self) -> ModelDefinition {
        ModelDefinition {
            table_name: self.table_name.to_string(),
            model_name: self.model_name.to_string(),
            fields: self.fields.iter().map(|f| f.to_owned()).collect(),
            docstring: self.docstring.to_string(),
            charybdis_code: self.charybdis_code.to_string(),
        }
    }
}

/// Static version of UdtModelDefinition - used for compile-time inventory.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub struct UdtModelDefinitionStatic {
    pub udt_name: &'static str,
    pub model_name: &'static str,
    pub fields: &'static [ModelFieldDefinitionStatic],
    pub docstring: &'static str,
    pub charybdis_code: &'static str,
}

impl UdtModelDefinitionStatic {
    /// Convert a static UDT model definition to a dynamic UDT model definition.
    pub fn to_owned(&self) -> UdtModelDefinition {
        UdtModelDefinition {
            udt_name: self.udt_name.to_string(),
            model_name: self.model_name.to_string(),
            fields: self.fields.iter().map(|f| f.to_owned()).collect(),
            docstring: self.docstring.to_string(),
            charybdis_code: self.charybdis_code.to_string(),
        }
    }
}

/// Static version of ModelFieldDefinition - used for compile-time inventory.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub struct ModelFieldDefinitionStatic {
    pub name: &'static str,
    pub field_type: DatabaseColumnType,
    pub partition_key: bool,
    pub clustering_key: bool,
    pub search_store: bool,
    pub search_index: bool,
    pub search_facet: bool,
    pub docstring: &'static str,
    pub nullable: bool,
    pub field_type_original: &'static str,
}

impl ModelFieldDefinitionStatic {
    /// Convert a static model field definition to a dynamic model field definition.
    pub fn to_owned(&self) -> ModelFieldDefinition {
        ModelFieldDefinition {
            name: self.name.to_string(),
            field_type: self.field_type.clone(),
            partition_key: self.partition_key,
            clustering_key: self.clustering_key,
            search_store: self.search_store,
            search_index: self.search_index,
            search_facet: self.search_facet,
            docstring: self.docstring.to_string(),
            nullable: self.nullable,
            field_type_original: self.field_type_original.to_string(),
        }
    }
}

inventory::collect!(ModelDefinitionStatic);
inventory::collect!(UdtModelDefinitionStatic);

/// Get all Charybdis codes for all models and UDTs.
pub fn get_all_charybdis_codes() -> Vec<String> {
    let mut codes = Vec::new();
    for model in inventory::iter::<ModelDefinitionStatic> {
        codes.push(model.charybdis_code.to_string());
    }
    for udt in inventory::iter::<UdtModelDefinitionStatic> {
        codes.push(udt.charybdis_code.to_string());
    }
    codes
        .iter()
        .map(|c| {
            c.replace("::charybdis::macros::", "")
                .replace("::hoover3_database::charybdis::types::", "")
        })
        .collect()
}

/// Get the Scylla database schema from the inventory of models and UDTs.
pub fn get_scylla_schema_from_inventory() -> std::sync::Arc<ScyllaDatabaseSchema> {
    SCYLLA_CODE_SCHEMA.clone()
}

lazy_static::lazy_static! {
    static ref SCYLLA_CODE_SCHEMA: std::sync::Arc<ScyllaDatabaseSchema> = read_scylla_schema_from_inventory();
}

fn read_scylla_schema_from_inventory() -> std::sync::Arc<ScyllaDatabaseSchema> {
    tracing::info!("read_scylla_schema_from_inventory()");
    let mut table_names = BTreeSet::new();
    let mut model_names = BTreeSet::new();

    let mut udts = BTreeMap::new();
    for udt_static in inventory::iter::<UdtModelDefinitionStatic> {
        if udts.contains_key(&udt_static.udt_name)
            || table_names.contains(udt_static.udt_name)
            || model_names.contains(udt_static.model_name)
        {
            panic!(
                "UDT `{}` defined multiple times in inventory",
                udt_static.udt_name
            );
        }
        udts.insert(udt_static.udt_name, udt_static);
        table_names.insert(udt_static.udt_name.to_string());
        model_names.insert(udt_static.model_name.to_string());
    }
    let mut tables = BTreeMap::new();
    for model_static in inventory::iter::<ModelDefinitionStatic> {
        let table_name = DatabaseIdentifier::new(model_static.table_name).unwrap();
        if table_names.contains(&table_name.to_string()) {
            panic!("table {} collides with UDT of same name", table_name);
        }
        if tables.contains_key(&table_name) {
            panic!("table {} defined multiple times in inventory", table_name);
        }
        let mut table = DatabaseTable {
            name: table_name.clone(),
            columns: Vec::new(),
        };

        for field in model_static.fields {
            table.columns.push(DatabaseColumn {
                name: DatabaseIdentifier::new(field.name).unwrap(),
                _type: resolve_field_type(&field, &udts),
                primary: field.partition_key || field.clustering_key,
                field_definition: Some(field.to_owned()),
            });
        }
        tables.insert(table_name, table);
    }
    std::sync::Arc::new(ScyllaDatabaseSchema { tables })
}

fn resolve_field_type(
    static_field: &ModelFieldDefinitionStatic,
    udts: &BTreeMap<&str, &UdtModelDefinitionStatic>,
) -> DatabaseColumnType {
    match static_field.field_type.clone() {
        DatabaseColumnType::UnspecifiedType => {
            // Try to match against UDT types
            if let Some(udt) = udts.get(&static_field.field_type_original) {
                DatabaseColumnType::Object(
                    udt.fields
                        .iter()
                        .map(|f| (f.name.to_string(), Box::new(resolve_field_type(f, udts))))
                        .collect(),
                )
            } else {
                static_field.field_type.clone()
            }
        }

        other => other,
    }
}

/// Static version of GraphEdgeType - used for compile-time inventory.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GraphEdgeTypeStatic {
    /// The name of the edge type - must be unique
    pub edge_type: &'static str,
    /// The source node type of the edge (table name in scylladb)
    pub source_type: &'static str,
    /// The target node type of the edge (table name in scylladb)
    pub target_type: &'static str,
    /// Implementation of the edge store
    pub edge_store_type: EdgeStoreImplementation,
}

impl From<&GraphEdgeTypeStatic> for GraphEdgeTypeDynamic {
    fn from(value: &GraphEdgeTypeStatic) -> Self {
        GraphEdgeTypeDynamic {
            edge_type: DatabaseIdentifier::new(value.edge_type).unwrap(),
            source_type: DatabaseIdentifier::new(value.source_type).unwrap(),
            target_type: DatabaseIdentifier::new(value.target_type).unwrap(),
            edge_store_type: value.edge_store_type,
        }
    }
}

/// Compile the graph edge schema from the inventory of edge types.
/// Index into a [GraphEdgeSchema] object by edge type name, source node type, or target node type.
pub fn get_graph_edges_types_from_inventory() -> std::sync::Arc<GraphEdgeSchemaDynamic> {
    GRAPH_EDGE_SCHEMA.clone()
}

lazy_static::lazy_static! {
    static ref GRAPH_EDGE_SCHEMA: std::sync::Arc<GraphEdgeSchemaDynamic> = read_graph_edges_types_from_inventory();
}

fn read_graph_edges_types_from_inventory() -> std::sync::Arc<GraphEdgeSchemaDynamic> {
    tracing::info!("read_graph_edges_types_from_inventory()");
    let mut edges_by_types = BTreeMap::new();
    let mut edges_by_source = BTreeMap::new();
    let mut edges_by_target = BTreeMap::new();
    for edge_type_def in inventory::iter::<GraphEdgeTypeStatic> {
        let Ok(edge_type_id) = DatabaseIdentifier::new(edge_type_def.edge_type) else {
            panic!("invalid edge type name: `{}`", edge_type_def.edge_type);
        };
        let edge_type_id = GraphEdgeId(edge_type_id);

        if edges_by_types.contains_key(&edge_type_id) {
            panic!(
                "edge type `{}` defined multiple times in inventory",
                edge_type_def.edge_type
            );
        }
        edges_by_types.insert(edge_type_id.clone(), edge_type_def.into());

        let source_id = DatabaseIdentifier::new(edge_type_def.source_type).unwrap();
        let target_id = DatabaseIdentifier::new(edge_type_def.target_type).unwrap();

        edges_by_source
            .entry(source_id)
            .or_insert_with(Vec::new)
            .push(edge_type_def.into());
        edges_by_target
            .entry(target_id)
            .or_insert_with(Vec::new)
            .push(edge_type_def.into());
    }

    std::sync::Arc::new(GraphEdgeSchemaDynamic {
        edges_by_types,
        edges_by_source,
        edges_by_target,
    })
}

inventory::collect!(GraphEdgeTypeStatic);
