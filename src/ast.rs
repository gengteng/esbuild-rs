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

type Location = usize;

#[derive(Debug)]
pub struct LocationRef {
    pub loc: Location,
    pub ref_: Ref,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct Ref {
    outer: u32,
    inner: u32,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct PropertyBinding {
    pub is_computed: bool,
    pub is_spread: bool,
    pub key: Expr,
    pub value: Binding,
    pub default_value: Option<Expr>,
}

#[derive(Debug)]
pub struct Arg {
    // "constructor(public x: boolean) {}"
    pub is_typescirpt_ctor_field: bool,
    pub binding: Binding,
    pub default_: Option<Expr>,
}

#[derive(Debug)]
pub struct Function {
    pub name: Option<LocationRef>,
    pub args: Vec<Arg>,
    pub is_async: bool,
    pub is_generator: bool,
    pub has_rest_arg: bool,
    pub body: (),
}

#[derive(Debug)]
pub struct FunctionBody {
    pub location: Location,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub struct Class {
    pub name: LocationRef,
    pub extends: Expr,
    pub properties: Vec<Property>,
}

#[derive(Debug)]
pub struct ArrayBinding {
    pub binding: Binding,
    pub default_value: Option<Expr>,
}

#[derive(Debug)]
pub struct Binding {
    pub location: Location,
    pub data: Box<BindingKind>,
}

#[derive(Debug)]
pub enum BindingKind {
    Missing,
    Identifier {
        ref_: Ref,
    },
    Array {
        items: Vec<ArrayBinding>,
        has_spread: bool,
    },
    Object {
        properties: Vec<PropertyBinding>,
    },
}

#[derive(Debug)]
pub struct Expr {
    pub location: Location,
    pub data: Box<ExprKind>,
}

#[derive(Debug)]
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
        ref_: Ref,
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
        ref_: Ref,
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
        head_raw: String,
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ExprOrStmt {
    Expr(Expr),
    Stmt,
}

#[derive(Debug)]
pub struct Stmt {
    pub location: Location,
    pub data: Box<StmtKind>,
}

#[derive(Debug)]
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
        namespace: Ref,
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
        arg: Ref,
        values: Vec<EnumValue>,
        is_export: bool,
    },
    Namespace {
        name: LocationRef,
        arg: Ref,
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
        namespace_symbol: NameSpaceSymbol,
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

#[derive(Debug)]
pub enum NameSpaceSymbol {
    Clause {
        items: Vec<ClauseItem>,
    },
    Star {
        location: Location,
        namespace_ref: Ref,
    },
}

#[derive(Debug)]
pub struct Catch {
    pub location: Location,
    pub binding: Option<Binding>,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub struct Finally {
    pub location: Location,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub struct Case {
    pub value: Option<Expr>,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub struct EnumValue {
    pub location: Location,
    pub ref_: Ref,
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

#[derive(Debug)]
pub struct ClauseItem {
    pub alias: String,
    pub alias_location: Location,
    pub name: LocationRef,
}

#[derive(Debug)]
pub struct Decl {
    pub binding: Binding,
    value: Option<Expr>,
}
