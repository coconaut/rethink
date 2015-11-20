use proto::*;

struct RQLQuery {
    tt: TermType,
    st: &'static str
}

impl RQLQuery {
    fn get(&self) -> RQLQuery {
        let q = RQLQuery {tt: TermType::GET, st: "get"};
        q
    }
}
