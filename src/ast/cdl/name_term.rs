use ast_term_derive::ASTTerm;

#[derive(ASTTerm, PartialEq, Eq, Debug, Clone)]
#[ast_term(visitor_path = "process_name")]
pub struct NameTerm {
    #[prop]
    value: String,
}
