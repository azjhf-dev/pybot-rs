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

#[test]
fn registry_and_arbiter_preserve_the_safety_band() {
    std::thread::Builder::new().stack_size(64*1024*1024).spawn(|| {
        pyrt::install_hook();gen_meta::register_all();
        let registry=gen_meta::registry();
        let safety=registry.iter().find(|m|m.name=="safety").unwrap();
        assert_eq!((safety.priority)(),9000);assert!(safety.respond.is_some());
        assert!(registry.iter().filter(|m|m.respond.is_some()).all(|m|(m.priority)()<=9000));
        let candidates=py_list(vec![
            py_dict(vec![(py_str("module"),py_str("safety")),(py_str("priority"),Py::Int(9000)),(py_str("reply"),py_str("Safety reply"))]),
            py_dict(vec![(py_str("module"),py_str("answers")),(py_str("priority"),Py::Int(5000)),(py_str("reply"),py_str("Ordinary reply"))])]);
        let picked=gen_meta::arbiter_choose().unwrap()(&[py_str("hello"),py_dict(vec![]),candidates]);
        assert!(py_eq(&picked,&Py::Int(0)),"{}",py_repr(&picked));
    }).unwrap().join().unwrap();
}
