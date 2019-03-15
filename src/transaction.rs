

use std::collections::HashMap;
use crate::sendmany::SaplingOutPoint;
use crate::sendmany::SaplingNoteData;

pub type NoteDataMap =  HashMap<SaplingOutPoint, SaplingNoteData>;

pub struct Transaction{
    //std::map<SaplingOutPoint, SaplingNoteData> mapSaplingData;
    pub mapSaplingData: NoteDataMap,
}