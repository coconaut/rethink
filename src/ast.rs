use proto::*;

// ----------------------------------------------------
// Main RQL Query traits - semi polymorphic
// ----------------------------------------------------
pub trait RQLQuery {
    // TODO: run fn
    fn eq(&self) -> Term {Term {tt: TermType::Eq, st: "eq"}}

}


pub trait RQLBoolOperQuery : RQLQuery {

}

pub trait RQLBiOperQuery : RQLQuery {

}

pub trait RQLBiCompareOperQuery : RQLQuery {

}

pub trait RQLTopLevelQuery : RQLQuery {

}

pub trait RQLMethodQuery : RQLQuery {
    // TODO: compose fn
}

pub trait RQLBracketQuery : RQLMethodQuery {

}

// ----------------------------------------------------
// Semi specific structs
// ----------------------------------------------------


pub struct Datum {

}
impl Datum {

}
impl RQLQuery for Datum {}


pub struct MakeArray {

}
impl MakeArray {

}
impl RQLQuery for MakeArray {}


pub struct MakeObj {

}
impl MakeObj {

}
impl RQLQuery for MakeObj {}


pub struct Var {
    tt: TermType
}
impl Var {

    //TODO: fn compose
}

// ----------------------------------------------------
// Query structs. All should default impl their
// respective query types, should enable the method chaining
// without rewriting.
// Doesn't seem worth a bunch of dup structs with same props,
// so trying to keep it simpler with Terms and let the
// query funcs assign the defaults, since we'd need to do it
// anyway.
// ----------------------------------------------------

pub struct Term {
    tt: TermType,
    st: &'static str
}
impl RQLQuery for Term { }


pub struct TopLevelTerm {
    tt: TermType,
    st: &'static str
}
impl RQLTopLevelQuery for TopLevelTerm {}


pub struct MethodTerm {
    tt: TermType,
    st: &'static str
}
impl RQLMethodQuery for MethodTerm {}


pub struct BiCompareOperTerm {
    tt: TermType,
    st: &'static str
}
impl RQLBiCompareOperQuery for BiCompareOperTerm {}


pub struct BiOperTerm {
    tt: TermType,
    st: &'static str
}
impl RQLBiOperQuery for BiOperTerm {}


pub struct BracketTerm {
    tt: TermType,
    st: &'static str
}
impl RQLBracketQuery for BracketTerm {}


// ----------------------------------------------------
// Main db structs
// ----------------------------------------------------

pub struct Table {
    fn get(&self) -> Term {tt: TermType::GET, st: "get"}
}
impl RQLQuery for Table {}

pub struct DB {

}
