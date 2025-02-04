use hoover3_types::{db_schema::GraphEdgeType, identifier::DatabaseIdentifier};

pub trait GraphEdgeIdentifier: Sized {
    fn name(&self) -> DatabaseIdentifier;
    fn to_owned(&self) -> GraphEdgeType {
        GraphEdgeType { name: self.name() }
    }
}

impl GraphEdgeIdentifier for GraphEdgeType {
    fn name(&self) -> DatabaseIdentifier {
        self.name.clone()
    }
}

macro_rules! declare_edge {
    ($id:ident, $ex:expr) => {
        pub struct $id;
        impl GraphEdgeIdentifier for $id {
            fn name(&self) -> DatabaseIdentifier {
                DatabaseIdentifier::new($ex).expect("invalid edge name: is not DatabaseIdentifier")
            }
        }
    };
}

declare_edge!(FilesystemParentEdge, "filesystem_parent");

pub fn get_all_nebula_edge_types() -> Vec<GraphEdgeType> {
    vec![FilesystemParentEdge.to_owned()]
}
