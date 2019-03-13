

use std::collections::HashMap;
use crate::sendmany::SaplingOutPoint;
use crate::sendmany::SaplingNoteData;



pub struct Transaction<'a>{
    //std::map<SaplingOutPoint, SaplingNoteData> mapSaplingData;
    pub mapSaplingData: HashMap<SaplingOutPoint, SaplingNoteData<'a>>,
}