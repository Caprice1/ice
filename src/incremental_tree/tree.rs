
use bigint::U256;

/*pub trait IncrementalMerkleTreeTrait<H> {

    fn path(&self, filler_hashs: Vec<H>) -> MerklePath;

    fn root(&self) -> H;
}*/


pub struct IncrementalMerkleTree<H> {

    pub tmp: Vec<H>,

}


impl<H> IncrementalMerkleTree<H> {

    fn path(&self, filler_hashs: Vec<H>) -> Option<MerklePath> {
        None
    }

    fn root(&self) -> Option<H> {
        None
    }
}



pub struct MerklePath {

}

impl MerklePath {

}


/*pub trait IncrementalWitnessTrait<H>{

    fn path(&self) -> MerklePath;

    fn partial_path(&self) -> Vec<H>;

    fn root(&self) -> H;
}
*/


pub struct IncrementalWitness<H> {
    pub tree: IncrementalMerkleTree<H>,
    pub filled: Vec<H>,
}

impl<H> IncrementalWitness<H> {
    /*MerklePath path() const {
    return tree.path(partial_path());
    }*/
    fn path(&self) -> Option<MerklePath> {
        //self.tree.path(self.partial_path())
        None
    }

    fn partial_path(&self) -> Option<Vec<H>> {
        None
    }

    /*Hash root() const {
    return tree.root(Depth, partial_path());
    }*/
    pub fn root(&self) -> Option<H> {
        //self.tree.root()
        None
    }

}


pub type SaplingWitness = IncrementalWitness<U256>;