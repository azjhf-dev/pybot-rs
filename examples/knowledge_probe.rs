use pybot_rs::{pyrt::{self,value::*,exc},memory};
fn main(){
 pyrt::install_hook();gen_meta::register_all();
 let text=std::env::args().nth(1).unwrap_or("I smell gas in my kitchen".into());
 let mem=memory::new_memory();
 let u=exc::catch(||pyrt::xrefs::xcall("understanding","understand",&[py_str(&text),mem.clone()]));
 match u {Ok(u)=>{println!("UNDERSTOOD {}",py_repr(&u));let r=exc::catch(||pyrt::xrefs::xcall("knowledge.lookup","find",&[u.clone()]));match r {Ok(v)=>{println!("FOUND {}",py_repr(&v));if matches!(&v,Py::Instance(_)){println!("ID {}",py_repr(&pyrt::builtins::gattr(&v,"id")));}},Err(e)=>println!("FIND ERROR {} {}",e.class_name,e.message)}},Err(e)=>println!("UNDERSTAND ERROR {} {}",e.class_name,e.message)}
}
