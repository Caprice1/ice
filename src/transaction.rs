

use std::collections::HashMap;
use crate::sendmany::SaplingOutPoint;
use crate::sendmany::SaplingNoteData;



pub struct Transaction{
    //std::map<SaplingOutPoint, SaplingNoteData> mapSaplingData;
    pub mapSaplingData: HashMap<SaplingOutPoint, SaplingNoteData>,
}