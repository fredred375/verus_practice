use vstd::prelude::*;

verus!{

pub struct MyStructView {
    pub field: Option<Seq<i32>>
}

impl MyStructView {
    pub open spec fn well_formed(&self) -> bool {
        self.field.is_Some() ==> (forall |i: int| 0 <= i < self.field.get_Some_0().len() ==> state_validation(self.field.get_Some_0()[i]))
    }
}

pub open spec fn state_validation(num: i32) -> bool {
    num != 0
}

}