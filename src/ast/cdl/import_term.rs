use super::{FromTerm, PathTerm, SelectionBlockTerm, SemicolumnTerm, UsingTerm};
use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
#[ast_term(visitor_path = "process_import")]
pub struct ImportTerm {
    #[subnode_prop]
    using: Box<UsingTerm>,
    #[subnode_prop]
    selection_block: Box<SelectionBlockTerm>,
    #[subnode_prop]
    from: Box<FromTerm>,
    #[subnode_prop]
    path: Box<PathTerm>,
    #[subnode_prop]
    semicolumn: Box<SemicolumnTerm>,
}
