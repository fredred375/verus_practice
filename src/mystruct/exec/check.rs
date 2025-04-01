use vstd::prelude::*;
use crate::mystruct::trusted::exec_types::*;

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
pub fn check_mystruct(mystruct: &MyStruct) -> (res: bool)
    ensures
        mystruct@.well_formed() == res
{
    mystruct.field().is_some()
}

}