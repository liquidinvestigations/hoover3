#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GraphEdgeType {
    pub name: &'static str,
}

#[allow(non_upper_case_globals)]
pub const FilesystemParentEdge: GraphEdgeType = GraphEdgeType {
    name: "filesystem_parent",
};

pub const ALL_NEBULA_EDGES: &[GraphEdgeType] = &[FilesystemParentEdge];
