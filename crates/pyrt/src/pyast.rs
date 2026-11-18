//! Minimal expression AST for the upstream arithmetic reader. No code execution.
use crate::value::*;
use crate::exc::PyExc;
use std::rc::Rc;
use std::cell::RefCell;
use indexmap::IndexMap;
fn node(kind:&str, fields:Vec<(&str,Py)>)->Py {
    Py::Instance(Rc::new(PyInstance{class_name:format!("ast.{}",kind),attrs:Rc::new(RefCell::new(fields.into_iter().map(|(k,v)|(k.to_string(),v)).collect::<IndexMap<_,_>>()))}))
}
struct Parser { chars:Vec<char>, at:usize }
impl Parser {
 fn ws(&mut self){while self.chars.get(self.at).map_or(false,|c|c.is_whitespace()){self.at+=1;}}
 fn peek(&mut self)->Option<char>{self.ws();self.chars.get(self.at).copied()}
 fn expr(&mut self, min:u8)->Result<Py,PyExc>{
  let mut left=match self.peek(){
   Some('+')|Some('-')=>{let c=self.chars[self.at];self.at+=1; let v=self.expr(25)?;node("UnaryOp",vec![("op",node(if c=='+'{"UAdd"}else{"USub"},vec![])),("operand",v)])},
   Some('(')=>{self.at+=1;let v=self.expr(0)?;if self.peek()!=Some(')'){return Err(val_err("missing closing parenthesis"));}self.at+=1;v},
   Some(c) if c.is_ascii_digit()||c=='.'=>{let begin=self.at;while self.chars.get(self.at).map_or(false,|c|c.is_ascii_digit()||*c=='.'){self.at+=1;}let text=self.chars[begin..self.at].iter().collect::<String>();let value=if let Ok(i)=text.parse::<i64>(){Py::Int(i)}else{Py::Float(text.parse::<f64>().map_err(|_|val_err("invalid number"))?)};node("Constant",vec![("value",value)])},
   _=>return Err(val_err("not arithmetic"))
  };
  loop {
   self.ws();let c=match self.chars.get(self.at){Some(c)=>*c,None=>break};
   let power=c=='*'&&self.chars.get(self.at+1)==Some(&'*');
   let (prec,kind)=if power{(30,"Pow")}else{match c {'+'=> (10,"Add"),'-'=>(10,"Sub"),'*'=>(20,"Mult"),'/'=>(20,"Div"),'%'=>(20,"Mod"),_=>break}};
   if prec<min{break;}self.at+=if power{2}else{1};let right=self.expr(if power{prec}else{prec+1})?;
   left=node("BinOp",vec![("left",left),("op",node(kind,vec![])),("right",right)]);
  }
  Ok(left)
 }
}
pub fn parse(text:&str)->PyResult{let mut p=Parser{chars:text.chars().collect(),at:0};let body=p.expr(0)?;p.ws();if p.at!=p.chars.len(){return Err(val_err("not arithmetic"));}Ok(node("Expression",vec![("body",body)]))}
