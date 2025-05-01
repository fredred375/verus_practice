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
    // requires
    //     mystruct@.field.is_Some() ==> mystruct@.field.get_Some_0().len() < 100,
    ensures
        mystruct@.well_formed() == res
{
    let mut result: bool = true;
    if let Some(f) = mystruct.field() {
        let mut idx: usize = 0;
        let fb = exec_types::map_vec(f);
        while idx < fb.len()
            invariant
                idx <= fb.len(),
                result == forall |i: int| 0 <= i < idx ==> spec_types::state_validation(fb[i]),
        {
            result = result && exec_types::state_validation(fb[idx]);
            idx += 1;
        }
    }
    result
}

}