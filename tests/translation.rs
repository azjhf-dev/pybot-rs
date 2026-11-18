use pybot_rs::pyrt::{self,value::*};

#[test]
fn nested_recursive_phrase_match_reaches_its_own_function() {
    std::thread::Builder::new().stack_size(64*1024*1024).spawn(|| {
        pyrt::install_hook();gen_meta::register_all();
        let forms=py_list(["phone","has","just","died"].iter().map(|w|py_frozenset(vec![py_str(w)])).collect());
        let span=pyrt::xrefs::xcall("knowledge.lookup","_phrase_span_between",&[forms,py_str("phone died")]);
        assert!(py_eq(&span,&py_tuple(vec![Py::Int(0),Py::Int(3)])),"{}",py_repr(&span));
    }).unwrap().join().unwrap();
}
