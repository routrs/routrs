use routrs::prelude::*;
use routrs_highways::HIGHWAYS;

fn it_reads_geograph(geograph: &Geograph) {
    assert!(geograph.nodes().count() > 1);
}

#[test]
fn it_reads_highways() {
    it_reads_geograph(&HIGHWAYS);
}
