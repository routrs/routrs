use routrs::prelude::*;
use routrs_railways::RAILWAYS;

fn it_reads_geograph(geograph: &Geograph) {
    assert!(geograph.nodes().count() > 1);
}

#[test]
fn it_reads_railways() {
    it_reads_geograph(&RAILWAYS);
}
