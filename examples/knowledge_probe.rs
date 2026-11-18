use pybot_rs::{pyrt::{self,value::*,exc},memory};
fn x(f:&str,a:&[Py])->Py{pyrt::xrefs::xcall("knowledge.lookup",f,a)}
fn main(){
 pyrt::install_hook();gen_meta::register_all();
 let text=std::env::args().nth(1).unwrap_or("I smell gas in my kitchen".into());
 let mem=memory::new_memory();
 let r=exc::catch(||{let u=pyrt::xrefs::xcall("understanding","understand",&[py_str(&text),mem.clone()]);println!("CLEAN {}",py_repr(&get(&u,"clean")));
 let forms=py_list(py_iter(&get(&u,"words")).unwrap().into_iter().map(|w|x("_forms",&[w])).collect());println!("FORMS {}",py_repr(&forms));
 let entries=pyrt::xrefs::xcall("knowledge","entries",&[]);
 for e in py_iter(&entries).unwrap(){let id=get(&e,"id");if matches!(id.as_str(), Some("rice-water") | Some("cook-a-staple")){for part in py_iter(&get(&e,"key")).unwrap(){println!("PART {}",py_repr(&x("_part_match",&[part,forms.clone()])));}println!("ENTRY {} KEY {} SCORE {}",py_repr(&id),py_repr(&get(&e,"key")),py_repr(&x("score",&[e.clone(),forms.clone(),get(&u,"clean")])));}}
 let voc=x("_vocabulary",&[entries.clone()]);let vs=py_iter(&voc).unwrap();let typed=py_list(py_iter(&get(&u,"words")).unwrap());let named=x("_named_things",&[u.clone()]);let rows=x("_line_rows",&[forms.clone(),typed,entries.clone(),vs[0].clone(),vs[1].clone(),named]);for row in py_iter(&rows).unwrap(){let rr=py_iter(&row).unwrap();if matches!(get(&rr[5],"id").as_str(),Some("rice-water")|Some("cook-a-staple")){println!("ROW {} {}",py_repr(&get(&rr[5],"id")),py_repr(&x("_row_order",&[row.clone()])));}}
 let v=x("find",&[u]);println!("FOUND {}",py_repr(&v));if matches!(&v,Py::Instance(_)){println!("ID {}",py_repr(&get(&v,"id")));}Py::None});
 if let Err(e)=r{println!("ERROR {} {}",e.class_name,e.message);}
}
fn get(v:&Py,k:&str)->Py{if matches!(v,Py::Dict(_)){py_getitem(v,&py_str(k)).unwrap_or(Py::None)}else{pyrt::builtins::gattr(v,k)}}
