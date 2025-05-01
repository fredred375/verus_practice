use vstd::prelude::*;
use crate::mystruct::trusted::exec_types;
use crate::mystruct::trusted::spec_types;


verus!{

/*
 * checks exec_type::MyStruct
 * notice that well_formed can be inferred automatically by verus
 * since we assumed our field() ensures
 *      match field@ {
 *          Some(x) => self@.field == Some(x),
 *          None => self@.field == None::<bool>,
 *      }
 */
pub fn check_mystruct(mystruct: &exec_types::MyStruct) -> (res: bool)
    ensures
        mystruct@.well_formed() == res
{
    if let Some(f) = mystruct.field() {
        let mut result: bool = true;
        let mut idx: usize = 0;
        while idx < f.len()
            invariant
                idx <= f.len(),
                result == forall |i: int| 0 <= i < idx ==> spec_types::state_validation(f[i]@),
        {
            result = result && exec_types::state_validation(&f[idx]);
            idx += 1;
        }
        assert(result == (forall |i: int| 0 <= i < f.len() ==> spec_types::state_validation(f[i]@)));
        assert(forall |i: int| 0 <= i < mystruct@.field.get_Some_0().len() ==> (mystruct@.field.get_Some_0()[i] == f[i]@));
        assert(mystruct@.field.get_Some_0().len() == f.len());
        assert(forall |i: int| 0 <= i < f.len() ==> (mystruct@.field.get_Some_0()[i] == f[i]@));
        // assert(result == (forall |i: int| 0 <= i < f.len() ==> spec_types::state_validation(mystruct@.field.get_Some_0()[i])));
        result
    } else {
        true
    }
}

}