// Every module (i.e. file) is parsed into a separate AST data structure. For
// efficiency, the parser also resolves all scopes and binds all symbols in the
// tree.
//
// Identifiers in the tree are referenced by a Ref, which is a pointer into the
// symbol table for the file. The symbol table is stored as a top-level field
// in the AST so it can be accessed without traversing the tree. For example,
// a renaming pass can iterate over the symbol table without touching the tree.
//
// Parse trees are intended to be immutable. That makes it easy to build an
// incremental compiler with a "watch" mode that can avoid re-parsing files
// that have already been parsed. Any passes that operate on an AST after it
// has been parsed should create a copy of the mutated parts of the tree
// instead of mutating the original tree.

use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::path::PathBuf;
use std::sync::Arc;

// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_Precedence
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Operator {
    Lowest = 0,
    Comma,
    Spread,
    Yield,
    Assign,
    Conditional,
    NullishCoalescing,
    LogicalOr,
    LogicalAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    Equals,
    Compare,
    Shift,
    Add,
    Multiply,
    Exponentiation,
    Prefix,
    Postfix,
    New,
    Call,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum OperatorCode {
    // If you add a new token, remember to add it to "OpTable" too

    // Prefix
    UnOpPos = 0,
    UnOpNeg,
    UnOpCpl,
    UnOpNot,
    UnOpVoid,
    UnOpTypeof,
    UnOpDelete,

    // Prefix update
    UnOpPreDec,
    UnOpPreInc,

    // Postfix update
    UnOpPostDec,
    UnOpPostInc,

    // Left-associative
    BinOpAdd,
    BinOpSub,
    BinOpMul,
    BinOpDiv,
    BinOpRem,
    BinOpPow,
    BinOpLt,
    BinOpLe,
    BinOpGt,
    BinOpGe,
    BinOpIn,
    BinOpInstanceof,
    BinOpShl,
    BinOpShr,
    BinOpUShr,
    BinOpLooseEq,
    BinOpLooseNe,
    BinOpStrictEq,
    BinOpStrictNe,
    BinOpNullishCoalescing,
    BinOpLogicalOr,
    BinOpLogicalAnd,
    BinOpBitwiseOr,
    BinOpBitwiseAnd,
    BinOpBitwiseXor,

    // Non-associative
    BinOpComma,

    // Right-associative
    BinOpAssign,
    BinOpAddAssign,
    BinOpSubAssign,
    BinOpMulAssign,
    BinOpDivAssign,
    BinOpRemAssign,
    BinOpPowAssign,
    BinOpShlAssign,
    BinOpShrAssign,
    BinOpUShrAssign,
    BinOpBitwiseOrAssign,
    BinOpBitwiseAndAssign,
    BinOpBitwiseXorAssign,
}

impl OperatorCode {
    pub fn is_prefix(self) -> bool {
        self < OperatorCode::UnOpPostDec
    }

    pub fn is_unary_update(self) -> bool {
        self >= OperatorCode::UnOpPreDec && self <= OperatorCode::UnOpPostInc
    }

    pub fn is_left_associative(self) -> bool {
        self >= OperatorCode::BinOpAdd
            && self < OperatorCode::BinOpComma
            && self != OperatorCode::BinOpPow
    }

    pub fn is_right_associative(self) -> bool {
        self >= OperatorCode::BinOpAssign || self == OperatorCode::BinOpPow
    }

    pub fn is_binary_assign(self) -> bool {
        self >= OperatorCode::BinOpAssign
    }
}

pub struct OperatorTableEntry {
    pub text: &'static str,
    pub level: Operator,
    pub is_keyword: bool,
}

macro_rules! make_entry {
    ($t:expr, $l:expr, $k:expr) => {
        OperatorTableEntry {
            text: $t,
            level: $l,
            is_keyword: $k,
        }
    };
}

pub const OPERATOR_TABLE: [OperatorTableEntry; 50] = [
    make_entry!("+", Operator::Prefix, false),
    make_entry!("-", Operator::Prefix, false),
    make_entry!("~", Operator::Prefix, false),
    make_entry!("!", Operator::Prefix, false),
    make_entry!("void", Operator::Prefix, true),
    make_entry!("typeof", Operator::Prefix, true),
    make_entry!("delete", Operator::Prefix, true),
    // Prefix update
    make_entry!("--", Operator::Prefix, false),
    make_entry!("++", Operator::Prefix, false),
    // Postfix update
    make_entry!("--", Operator::Postfix, false),
    make_entry!("++", Operator::Postfix, false),
    // Operator::eft-associative
    make_entry!("+", Operator::Add, false),
    make_entry!("-", Operator::Add, false),
    make_entry!("*", Operator::Multiply, false),
    make_entry!("/", Operator::Multiply, false),
    make_entry!("%", Operator::Multiply, false),
    make_entry!("**", Operator::Exponentiation, false), // Right-associative
    make_entry!("<", Operator::Compare, false),
    make_entry!("<=", Operator::Compare, false),
    make_entry!(">", Operator::Compare, false),
    make_entry!(">=", Operator::Compare, false),
    make_entry!("in", Operator::Compare, true),
    make_entry!("instanceof", Operator::Compare, true),
    make_entry!("<<", Operator::Shift, false),
    make_entry!(">>", Operator::Shift, false),
    make_entry!(">>>", Operator::Shift, false),
    make_entry!("==", Operator::Equals, false),
    make_entry!("!=", Operator::Equals, false),
    make_entry!("===", Operator::Equals, false),
    make_entry!("!==", Operator::Equals, false),
    make_entry!("??", Operator::NullishCoalescing, false),
    make_entry!("||", Operator::LogicalOr, false),
    make_entry!("&&", Operator::LogicalAnd, false),
    make_entry!("|", Operator::BitwiseOr, false),
    make_entry!("&", Operator::BitwiseAnd, false),
    make_entry!("^", Operator::BitwiseXor, false),
    // Non-associative
    make_entry!(",", Operator::Comma, false),
    // Right-associative
    make_entry!("=", Operator::Assign, false),
    make_entry!("+=", Operator::Assign, false),
    make_entry!("-=", Operator::Assign, false),
    make_entry!("*=", Operator::Assign, false),
    make_entry!("/=", Operator::Assign, false),
    make_entry!("%=", Operator::Assign, false),
    make_entry!("**=", Operator::Assign, false),
    make_entry!("<<=", Operator::Assign, false),
    make_entry!(">>=", Operator::Assign, false),
    make_entry!(">>>=", Operator::Assign, false),
    make_entry!("|=", Operator::Assign, false),
    make_entry!("&=", Operator::Assign, false),
    make_entry!("^=", Operator::Assign, false),
];

pub type Location = usize;

#[derive(Debug, Clone)]
pub struct LocationRef {
    pub loc: Location,
    pub reference: Reference,
}

#[derive(Debug, Clone)]
pub struct Path {
    pub loc: Location,
    pub text: String,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum PropertyKind {
    PropertyNormal = 0,
    PropertyGet,
    PropertySet,
    PropertySpread,
}

#[derive(Debug, Clone)]
pub struct Property {
    pub kind: PropertyKind,
    pub is_computed: bool,
    pub is_method: bool,
    pub is_static: bool,
    pub key: Expr,
}

//type PropertyBinding struct {
// 	IsComputed   bool
// 	IsSpread     bool
// 	Key          Expr
// 	Value        Binding
// 	DefaultValue *Expr
// }

#[derive(Debug, Clone)]
pub struct PropertyBinding {
    pub is_computed: bool,
    pub is_spread: bool,
    pub key: Expr,
    pub value: Binding,
    pub default_value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct Arg {
    // "constructor(public x: boolean) {}"
    pub is_typescirpt_ctor_field: bool,
    pub binding: Binding,
    pub default_: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Option<LocationRef>,
    pub args: Vec<Arg>,
    pub is_async: bool,
    pub is_generator: bool,
    pub has_rest_arg: bool,
    pub body: (),
}

#[derive(Debug, Clone)]
pub struct FunctionBody {
    pub location: Location,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: LocationRef,
    pub extends: Expr,
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone)]
pub struct ArrayBinding {
    pub binding: Binding,
    pub default_value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct Binding {
    pub location: Location,
    pub data: Box<BindingKind>,
}

#[derive(Debug, Clone)]
pub enum BindingKind {
    Missing,
    Identifier {
        reference: Reference,
    },
    Array {
        items: Vec<ArrayBinding>,
        has_spread: bool,
    },
    Object {
        properties: Vec<PropertyBinding>,
    },
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub location: Location,
    pub data: Box<ExprKind>,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Array {
        items: Vec<Expr>,
    },
    Unary {
        op_code: OperatorCode,
        value: Expr,
    },
    Binary {
        op_code: OperatorCode,
        left: Expr,
        right: Expr,
    },
    Boolean {
        value: bool,
    },
    Super,
    Null,
    Undefined,
    This,
    New {
        target: Expr,
        args: Vec<Expr>,
    },
    NewTarget,
    ImportMeta,
    Call {
        target: Expr,
        args: Vec<Expr>,
        is_optional_chain: bool,
        is_parenthesized: bool,
        is_direct_eval: bool,
    },
    RuntimeCall {
        sym: u16, // TODO: fix me --> runtime.Sym
        args: Vec<Expr>,
    },
    Dot {
        target: Expr,
        name: String,
        name_location: Location,
        is_optional_chain: bool,
        is_parenthesized: bool,
    },
    Index {
        target: Expr,
        index: Expr,
        is_optional_chain: bool,
        is_parenthesized: bool,
    },
    Arrow {
        is_async: bool,
        args: Vec<Expr>,
        has_rest_arg: bool,
        is_parenthesized: bool,
        prefer_expr: bool,
        body: FunctionBody,
    },
    Function {},
    Class {
        class: Class,
    },
    Identifier {
        reference: Reference,
    },

    // This is similar to an Identifier but it represents a reference to an ES6
    // import item.
    //
    // Depending on how the code is linked, the file containing this ImportIdentifier
    // may or may not be in the same module group as the file it was imported from.
    //
    // If it's the same module group than we can just merge the import item symbol
    // with the corresponding symbol that was imported, effectively renaming them
    // to be the same thing and statically binding them together.
    //
    // But if it's a different module group, then the import must be dynamically
    // evaluated using a property access off the corresponding namespace symbol,
    // which represents the result of a require() call.
    //
    // It's stored as a separate type so it's not easy to confuse with a plain
    // identifier. For example, it'd be bad if code trying to convert "{x: x}" into
    // "{x}" shorthand syntax wasn't aware that the "x" in this case is actually
    // "{x: importedNamespace.x}". This separate type forces code to opt-in to
    // doing this instead of opt-out.
    ImportIdentifier {
        reference: Reference,
    },
    JSXElement {},
    Missing,
    Number {
        value: f64,
    },
    BigInt {
        value: String,
    },
    Object {
        properties: Vec<Property>,
    },
    Spread {
        value: Expr,
    },
    String {
        value: Vec<u16>,
    },
    Template {
        tag: Expr,
        head: Vec<u16>,
        head_raw: String, // This is only filled out for tagged template literals
        parts: Vec<TemplatePart>,
    },
    RegExp {
        value: String,
    },
    Await {
        value: Expr,
    },
    Yield {
        value: Expr,
        is_star: bool,
    },
    If {
        test: Expr,
        yes: Expr,
        no: Expr,
    },
    Require {
        path: Path,
        is_es6_import: bool,
    },
    Import {
        expr: Expr,
    },
}

#[derive(Debug, Clone)]
pub struct TemplatePart {
    pub value: Expr,
    pub tail: Vec<u16>,
    pub tail_raw: String, // This is only filled out for tagged template literals
}

pub fn join_with_comma(a: Expr, b: Expr) -> Expr {
    Expr {
        location: a.location,
        data: Box::new(ExprKind::Binary {
            op_code: OperatorCode::BinOpComma,
            left: a,
            right: b,
        }),
    }
}

pub fn join_all_with_comma<I: Iterator<Item = Expr>>(mut all: I) -> Expr {
    let first = all.next().unwrap();
    all.fold(first, |a, b| join_with_comma(a, b))
}

#[derive(Debug, Clone)]
pub enum ExprOrStmt {
    Expr(Expr),
    Stmt,
}

#[derive(Debug, Clone)]
pub struct Stmt {
    pub location: Location,
    pub data: Box<StmtKind>,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    Block {
        stmts: Vec<Stmt>,
    },
    Empty,
    TypeScript,
    Debugger,
    Directive {
        value: Vec<u16>,
    },
    ExportClause {
        items: Vec<ClauseItem>,
    },
    ExportFrom {
        items: Vec<ClauseItem>,
        namespace: Reference,
        path: Path,
    },
    ExportDefault {
        default_name: LocationRef,
        value: ExprOrStmt, // May be a SFunction or SClass
    },
    ExportStar {
        item: Option<ClauseItem>,
        path: Path,
    },
    ExportEquals {
        value: Expr,
    },
    Expr {
        value: Expr,
    },
    Enum {
        name: LocationRef,
        arg: Reference,
        values: Vec<EnumValue>,
        is_export: bool,
    },
    Namespace {
        name: LocationRef,
        arg: Reference,
        stmts: Vec<Stmt>,
        is_export: bool,
    },
    Function {
        function: Function,
        is_export: bool,
    },
    Class {
        class: Class,
        is_export: bool,
    },
    Label {
        name: LocationRef,
        stmt: Stmt,
    },
    If {
        test: Expr,
        yes: Stmt,
        no: Option<Stmt>,
    },
    For {
        init: Option<Stmt>, // May be a SConst, SLet, SVar, or SExpr
        test: Option<Expr>,
        update: Option<Expr>,
        body: Stmt,
    },
    ForIn {
        init: Stmt, // May be a SConst, SLet, SVar, or SExpr
        value: Expr,
        body: Stmt,
    },
    ForOf {
        is_await: bool,
        init: Stmt,
        value: Expr,
        body: Stmt,
    },
    DoWhile {
        body: Stmt,
        test: Expr,
    },
    While {
        test: Expr,
        body: Stmt,
    },
    With {
        value: Expr,
        body_location: Location,
        body: Stmt,
    },
    Catch(Catch),
    Finally(Finally),
    Try {
        body: Vec<Stmt>,
        catch: Option<Catch>,
        finally: Option<Finally>,
    },
    Switch {
        test: Expr,
        body_location: Location,
        cases: Vec<Case>,
    },

    // This object represents all of these types of import statements:
    //
    //   import 'path'
    //   import {item1, item2} from 'path'
    //   import * as ns from 'path'
    //   import defaultItem, {item1, item2} from 'path'
    //   import defaultItem, * as ns from 'path'
    //
    // Many parts are optional and can be combined in different ways. The only
    // restriction is that you cannot have both a clause and a star namespace.
    Import {
        // If this is a star import: This is a Ref for the namespace symbol. The Loc
        // for the symbol is StarLoc.
        //
        // Otherwise: This is an auto-generated Ref for the namespace representing
        // the imported file. In this case StarLoc is nil. The NamespaceRef is used
        // when converting this module to a CommonJS module.
        namespace_symbol: NamespaceSymbol,
        default_name: Option<LocationRef>,
        path: Path,
    },
    Return {
        value: Option<Expr>,
    },
    Throw {
        value: Expr,
    },
    Local {
        decls: Vec<Decl>,
        kind: LocalKind,
        is_export: bool,
        // The TypeScript compiler doesn't generate code for "import foo = bar"
        // statements inside namespaces where the import is never used.
        was_ts_import_equals_in_namespace: bool,
    },
    Break {
        name: Option<LocationRef>,
    },
    Continue {
        name: Option<LocationRef>,
    },
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum LocalKind {
    Var = 0,
    Let,
    Const,
}

#[derive(Debug, Clone)]
pub enum NamespaceSymbol {
    Clause {
        items: Vec<ClauseItem>,
    },
    Star {
        location: Location,
        namespace_ref: Reference,
    },
}

#[derive(Debug, Clone)]
pub struct Catch {
    pub location: Location,
    pub binding: Option<Binding>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Finally {
    pub location: Location,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Case {
    pub value: Option<Expr>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct EnumValue {
    pub location: Location,
    pub reference: Reference,
    pub name: Vec<u16>,
    pub value: Option<Expr>,
}

//func IsSuperCall(stmt Stmt) bool {
// 	if expr, ok := stmt.Data.(*SExpr); ok {
// 		if call, ok := expr.Value.Data.(*ECall); ok {
// 			if _, ok := call.Target.Data.(*ESuper); ok {
// 				return true
// 			}
// 		}
// 	}
// 	return false
// }

impl Stmt {
    pub fn is_super_call(&self) -> bool {
        if let StmtKind::Expr { value } = self.data.as_ref() {
            if let ExprKind::Call { target, .. } = value.data.as_ref() {
                if let ExprKind::Super = target.data.as_ref() {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Debug, Clone)]
pub struct ClauseItem {
    pub alias: String,
    pub alias_location: Location,
    pub name: LocationRef,
}

#[derive(Debug, Clone)]
pub struct Decl {
    pub binding: Binding,
    pub value: Option<Expr>,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum SymbolKind {
    // An unbound symbol is one that isn't declared in the file it's referenced
    // in. For example, using "window" without declaring it will be unbound.
    Unbound = 0,

    // This has special merging behavior. You're allowed to re-declare these
    // symbols more than once in the same scope. These symbols are also hoisted
    // out of the scope they are declared in to the closest containing function
    // or module scope. These are the symbols with this kind:
    //
    // - Function arguments
    // - Function statements
    // - Variables declared using "var"
    //
    Hoisted,
    HoistedFunction,

    // There's a weird special case where catch variables declared using a simple
    // identifier (i.e. not a binding pattern) block hoisted variables instead of
    // becoming an error:
    //
    //   var e = 0;
    //   try { throw 1 } catch (e) {
    //     print(e) // 1
    //     var e = 2
    //     print(e) // 2
    //   }
    //   print(e) // 0 (since the hoisting stops at the catch block boundary)
    //
    // However, other forms are still a syntax error:
    //
    //   try {} catch (e) { let e }
    //   try {} catch ({e}) { var e }
    //
    // This symbol is for handling this weird special case.
    CatchIdentifier,

    // Classes can merge with TypeScript namespaces.
    Class,

    // TypeScript enums can merge with TypeScript namespaces and other TypeScript
    // enums.
    TSEnum,

    // TypeScript namespaces can merge with classes, functions, TypeScript enums,
    // and other TypeScript namespaces.
    TSNamespace,

    // In TypeScript, imports are allowed to silently collide with symbols within
    // the module. Presumably this is because the imports may be type-only.
    TSImport,

    // This annotates all other symbols that don't have special behavior.
    Other,
}

impl SymbolKind {
    pub fn is_hoisted(self) -> bool {
        self == SymbolKind::Hoisted || self == SymbolKind::HoistedFunction
    }
}

pub const INVALID_REF: Reference = Reference { outer: 0, inner: 0 };

// Files are parsed in parallel for speed. We want to allow each parser to
// generate symbol IDs that won't conflict with each other. We also want to be
// able to quickly merge symbol tables from all files into one giant symbol
// table.
//
// We can accomplish both goals by giving each symbol ID two parts: an outer
// index that is unique to the parser goroutine, and an inner index that
// increments as the parser generates new symbol IDs. Then a symbol map can
// be an array of arrays indexed first by outer index, then by inner index.
// The maps can be merged quickly by creating a single outer array containing
// all inner arrays from all parsed files.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct Reference {
    pub outer: usize,
    pub inner: usize,
}

impl Reference {
    pub const fn new(outer: usize, inner: usize) -> Self {
        Self { outer, inner }
    }
}

#[derive(Debug, Clone)]
pub struct NamespaceAlias {
    pub namespace_ref: Reference,
    pub alias: String,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub kind: SymbolKind,

    // Certain symbols must not be renamed or minified. For example, the
    // "arguments" variable is declared by the runtime for every function.
    // Renaming can also break any identifier used inside a "with" statement.
    pub must_not_be_renamed: bool,

    // An estimate of the number of uses of this symbol. This is used for
    // minification (to prefer shorter names for more frequently used symbols).
    // The reason why this is an estimate instead of an accurate count is that
    // it's not updated during dead code elimination for speed. I figure that
    // even without updating after parsing it's still a pretty good heuristic.
    pub use_count_estimate: u32,
    pub name: String,

    // Used by the parser for single pass parsing. Symbols that have been merged
    // form a linked-list where the last link is the symbol to use. This link is
    // an invalid ref if it's the last link. If this isn't invalid, you need to
    // FollowSymbols to get the real one.
    pub link: Reference,

    // This is used for symbols that represent items in the import clause of an
    // ES6 import statement. These should always be referenced by EImportIdentifier
    // instead of an EIdentifier. When this is present, the expression should
    // be printed as a property access off the namespace instead of as a bare
    // identifier.
    //
    // For correctness, this must be stored on the symbol instead of indirectly
    // associated with the Ref for the symbol somehow. In ES6 "flat bundling"
    // mode, re-exported symbols are collapsed using MergeSymbols() and renamed
    // symbols from other files that end up at this symbol must be able to tell
    // if it has a namespace alias.
    pub namespace_alias: Arc<NamespaceAlias>,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum ScopeKind {
    Block = 0,
    With,
    Label,
    ClassName,

    // The scopes below stop hoisted variables from extending into parent scopes
    Entry, // This is a module, TypeScript enum, or TypeScript namespace
    FunctionArgs,
    FunctionBody,
}

impl ScopeKind {
    pub fn stops_hoisting(self) -> bool {
        self >= ScopeKind::Entry
    }
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub kind: ScopeKind,
    pub parent: Arc<Scope>,
    pub children: Vec<Arc<Scope>>,
    pub members: HashMap<String, Reference>,
    pub generated: Vec<Reference>,

    // This is used to store the ref of the label symbol for ScopeLabel scopes.
    pub label_ref: Reference,

    // If a scope contains a direct eval() expression, then none of the symbols
    // inside that scope can be renamed. We conservatively assume that the
    // evaluated code might reference anything that it has access to.
    pub contains_direct_eval: bool,
}

#[derive(Debug, Clone)]
pub struct SymbolMap {
    // This could be represented as a "map[Ref]Symbol" but a two-level array was
    // more efficient in profiles. This appears to be because it doesn't involve
    // a hash. This representation also makes it trivial to quickly merge symbol
    // maps from multiple files together. Each file only generates symbols in a
    // single inner array, so you can join the maps together by just make a
    // single outer array containing all of the inner arrays. See the comment on
    // "Ref" for more detail.
    pub outer: Vec<Vec<Symbol>>,
}

impl SymbolMap {
    pub fn new(src_count: usize) -> Self {
        Self {
            outer: vec![vec![]; src_count],
        }
    }

    pub fn set(&mut self, reference: Reference, symbol: Symbol) {
        self[reference] = symbol;
    }

    pub fn set_kind(&mut self, reference: Reference, kind: SymbolKind) {
        self[reference].kind = kind;
    }

    pub fn set_namespace_alias(&mut self, reference: Reference, alias: Arc<NamespaceAlias>) {
        self[reference].namespace_alias = alias;
    }

    pub fn increment_use_count_estimate(&mut self, reference: Reference) {
        self[reference].use_count_estimate += 1;
    }
}

impl Index<Reference> for SymbolMap {
    type Output = Symbol;

    fn index(&self, index: Reference) -> &Self::Output {
        &self.outer[index.outer][index.inner]
    }
}

impl IndexMut<Reference> for SymbolMap {
    fn index_mut(&mut self, index: Reference) -> &mut Self::Output {
        &mut self.outer[index.outer][index.inner]
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum ImportKind {
    Stmt = 0,
    Require,
    Dynamic,
}

#[derive(Debug, Clone)]
pub struct ImportPath {
    pub path: Path,
    pub kind: ImportKind,
}

//type AST struct {
#[derive(Debug, Clone)]
pub struct AST {
    pub import_paths: Vec<ImportPath>,
    pub was_typescript: bool,

    // This is true if something used the "exports" or "module" variables, which
    // means they could have exported something. It's also true if the file
    // contains a top-level return statement. When a file uses CommonJS features,
    // it's not a candidate for "flat bundling" and must be wrapped in its own
    // closure.
    pub uses_commonjs_features: bool,
    pub hash_bang: String,
    pub stmts: Vec<Stmt>,
    pub symbols: SymbolMap,
    pub module_scope: Scope,
    pub exports_ref: Reference,
    pub module_ref: Reference,

    // This is a bitwise-or of all runtime symbols used by this AST. Runtime
    // symbols are used by ERuntimeCall expressions.
    pub used_runtime_symbols: (), //TODO: runtime.Syn
}

// Returns the canonical ref that represents the ref for the provided symbol.
// This may not be the provided ref if the symbol has been merged with another
// symbol.
pub fn follow_symbols(symbols: &mut SymbolMap, reference: Reference) -> Reference {
    let sym_link = symbols[reference].link;
    if sym_link == INVALID_REF {
        return reference;
    }

    let link = follow_symbols(symbols, sym_link);

    // Only write if needed to avoid concurrent map update hazards
    if sym_link != link {
        symbols[reference].link = link;
    }

    link
}
// Use this before calling "FollowSymbols" from separate threads to avoid
// concurrent map update hazards. In Go, mutating a map is not threadsafe
// but reading from a map is. Calling "FollowAllSymbols" first ensures that
// all mutation is done up front.
pub fn follow_all_symbols(symbols: &mut SymbolMap) {
    let outer_len = symbols.outer.len();
    if outer_len > 0 {
        for i in 0..outer_len {
            let inner_len = symbols.outer[i].len();
            for j in 0..inner_len {
                follow_symbols(symbols, Reference::new(i, j));
            }
        }
    }
}

// Makes "old" point to "new" by joining the linked lists for the two symbols
// together. That way "FollowSymbols" on both "old" and "new" will result in
// the same ref.
pub fn merge_symbols(symbols: &mut SymbolMap, old: Reference, new: Reference) -> Reference {
    // 	if old == new {
    // 		return new
    // 	}
    //
    // 	oldSymbol := symbols.Get(old)
    // 	if oldSymbol.Link != InvalidRef {
    // 		oldSymbol.Link = MergeSymbols(symbols, oldSymbol.Link, new)
    // 		symbols.Set(old, oldSymbol)
    // 		return oldSymbol.Link
    // 	}

    if old == new {
        return new;
    }

    let old_link = symbols[old].link;
    if old_link != INVALID_REF {
        symbols[old].link = merge_symbols(symbols, old_link, new);
        return old_link;
    }

    // 	newSymbol := symbols.Get(new)
    // 	if newSymbol.Link != InvalidRef {
    // 		newSymbol.Link = MergeSymbols(symbols, old, newSymbol.Link)
    // 		symbols.Set(new, newSymbol)
    // 		return newSymbol.Link
    // 	}
    let new_link = symbols[new].link;
    if new_link != INVALID_REF {
        symbols[new].link = merge_symbols(symbols, old, new_link);
        return new_link;
    }

    // 	oldSymbol.Link = new
    // 	newSymbol.UseCountEstimate += oldSymbol.UseCountEstimate
    // 	if oldSymbol.MustNotBeRenamed {
    // 		newSymbol.MustNotBeRenamed = true
    // 	}
    // 	symbols.Set(old, oldSymbol)
    // 	symbols.Set(new, newSymbol)
    // 	return new
    symbols[old].link = new;
    symbols[new].use_count_estimate += symbols[old].use_count_estimate;
    if symbols[old].must_not_be_renamed {
        symbols[new].must_not_be_renamed = true;
    }

    new
}

pub fn generate_non_unique_name_from_path<P: Into<PathBuf>>(path: P) -> String {
    let path = path.into();
    let mut name = String::new();

    // Get the file name without the extension
    if let Some(Some(a)) = path.file_stem().map(|s| s.to_str()) {
        let mut tail: Option<char> = None;
        // Convert it to an ASCII identifier
        for c in a.chars() {
            if ('A'..='Z').contains(&c)
                || ('a'..='z').contains(&c)
                || (!name.is_empty() && ('0'..'9').contains(&c))
            {
                name.push(c);
                tail = Some(c);
            } else {
                if !name.is_empty() {
                    if let Some('_') = tail {
                    } else {
                        name.push('_');
                        tail = Some('_');
                    }
                }
            }
        }
    }

    // Make sure the name isn't empty
    if name.is_empty() {
        return "_".into();
    }

    name
}
