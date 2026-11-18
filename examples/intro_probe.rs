use pybot_rs::pyrt::{self,value::*,exc};
fn main(){std::thread::Builder::new().stack_size(64*1024*1024).spawn(||{
pyrt::install_hook();gen_meta::register_all();
let text=py_str("my name is Alex");let mem=py_dict(vec![]);let mut mods=gen_meta::registry();mods.sort_by(|a,b|(b.priority)().cmp(&(a.priority)()).then(a.name.cmp(b.name)));
for m in mods {if let Some(f)=m.observe {if let Err(e)=exc::catch(||f(&[text.clone(),mem.clone()])){println!("OBSERVER {} {} {}",m.name,e.class_name,e.message)}}}
for key in ["user","facts_log","understood"] {println!("{} {}",key,py_repr(&py_getitem(&mem,&py_str(key)).unwrap_or(Py::None)));}
for f in ["_just_met","_respond","respond"] {let args=if f=="_just_met"{vec![mem.clone()]}else{vec![text.clone(),mem.clone()]}; match exc::catch(||pyrt::xrefs::xcall("modules.chat",f,&args)){Ok(v)=>println!("{} {}",f,py_repr(&v)),Err(e)=>println!("{} ERROR {} {}",f,e.class_name,e.message)}}
}).unwrap().join().unwrap();}
