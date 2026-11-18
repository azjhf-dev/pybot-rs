use pybot_rs::pyrt::{self, value::*, exc};
use std::io::BufRead;
fn main() {
    pyrt::install_hook();
    gen_meta::register_all();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut fields = line.splitn(2, '\t');
        let module = fields.next().unwrap();
        let text = fields.next().unwrap_or("");
        let memory = py_dict(vec![]);
        let result = exc::catch(|| pyrt::xrefs::xcall(module, "respond", &[py_str(text), memory]));
        match result {
            Ok(v) => println!("{}\t{}", module, py_repr(&v)),
            Err(e) => println!("{}\tERROR {}: {}",module,e.class_name,e.message),
        }
    }
}
