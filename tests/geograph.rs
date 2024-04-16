use routrs::{geograph::Geograph, HIGHWAYS, MARNET, RAILWAYS};

fn it_reads_geograph(geograph: &Geograph) {
    assert!(geograph.nodes().count() > 1);
}

#[test]
fn it_reads_marnet() {
    it_reads_geograph(&MARNET);
}
#[test]
fn it_reads_highways() {
    it_reads_geograph(&HIGHWAYS);
}
#[test]
fn it_reads_railways() {
    it_reads_geograph(&RAILWAYS);
}
