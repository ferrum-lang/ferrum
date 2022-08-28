#[derive(Clone, Debug, PartialEq)]
pub struct Import {
    pub assignment: ImportAssignment,
    pub source: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ImportAssignment {
    Direct(ImportAssignmentDirect),
    Destructured(ImportAssignmentDestruct)
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImportAssignmentDirect {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImportAssignmentDestruct {
    pub items: Vec<ImportAssignDestructItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ImportAssignDestructItem {
    Field(ImportAssignDestructField),
    Spread(ImportAssignDestructSpread),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImportAssignDestructField {
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImportAssignDestructSpread {
    pub name: String,
}

