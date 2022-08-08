#[derive(Clone, Debug, PartialEq)]
pub struct Import {
    pub assignment: ImportAssignment,
    pub source: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ImportAssignment {
    Direct(ImportAssignmentDirect),
    Destructured(ImportAssignmentDestructured)
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImportAssignmentDirect {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImportAssignmentDestructured {
    pub fields: Vec<ImportAssignmentDestructuredField>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImportAssignmentDestructuredField {
    pub name: String,
    pub alias: Option<String>,
}


