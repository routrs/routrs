use routrs::prelude::*;

fn it_reads_geograph(geograph: &Geograph) {
    assert!(geograph.nodes().count() > 1);
}

#[test]
fn it_reads_marnet() {
    it_reads_geograph(&MARITIME);
}
