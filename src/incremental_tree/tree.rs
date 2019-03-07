
use bigint::U256;

pub trait IncrementalMerkleTreeTrait<H> {

    fn path(&self, filler_hashs: Vec<H>) -> MerklePath;

    fn root(&self) -> H;
}


/*pub struct IncrementalMerkleTree<H> {

}


impl<H> IncrementalMerkleTreeTrait<H> for IncrementalMerkleTree<H> {

    fn path(&self, filler_hashs: Vec<H>) -> MerklePath {

    }

    fn root(&self) -> H {

    }
}*/



pub struct MerklePath {

}

impl MerklePath {

}


pub trait IncrementalWitnessTrait<H>{

    fn path(&self) -> MerklePath;

    fn partial_path(&self) -> Vec<H>;

    fn root(&self) -> H;
}

/*
pub struct IncrementalWitness<H> {
    pub tree: IncrementalMerkleTree<H>,
    pub filled: Vec<H>,
}

impl<H> IncrementalWitnessTrait<H> for IncrementalWitness<H> {
    /*MerklePath path() const {
    return tree.path(partial_path());
    }*/
    fn path(&self) -> MerklePath {
        self.tree.path(self.partial_path())
    }

    fn partial_path(&self) -> Vec<H> {

    }

    /*Hash root() const {
    return tree.root(Depth, partial_path());
    }*/
    fn root(&self) -> H {
        self.tree.root()
    }

}
*/

pub type SaplingWitness = IncrementalWitnessTrait<U256>;