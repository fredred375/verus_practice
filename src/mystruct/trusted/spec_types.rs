use vstd::prelude::*;

verus!{

pub struct MyStructView {
    pub field: Option<Seq<MyNumView>>
}

impl MyStructView {
    pub open spec fn well_formed(&self) -> bool {
        self.field.is_Some() ==> (forall |i: int| 0 <= i < self.field.get_Some_0().len() ==> #[trigger] self.field.get_Some_0()[i].state_validation())
    }
}


pub struct MyNumView {
    pub num: i32
}

impl MyNumView {
    pub open spec fn state_validation(&self) -> bool {
        self.num != 0
    }
}

}