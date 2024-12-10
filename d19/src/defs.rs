use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Obj {
    pub x: u64,
    pub m: u64,
    pub a: u64,
    pub s: u64,
}

impl Obj {
    pub fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Val {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Op {
    Lt,
    Gt,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rule {
    pub name: String,
    pub ops: Vec<(Val, Op, u64, String)>,
    pub ow: String,
}

impl Rule {
    pub fn apply(&self, obj: &Obj) -> String {
        for (val, op, num, dest) in &self.ops {
            let obj_val = match val {
                Val::X => obj.x,
                Val::M => obj.m,
                Val::A => obj.a,
                Val::S => obj.s,
            };
            let res = match op {
                Op::Lt => obj_val < *num,
                Op::Gt => obj_val > *num,
            };
            if res {
                return dest.clone();
            }
        }
        self.ow.clone()
    }
}
