#[cfg(test)]
mod json {
    use std::fs;

    use routrs::geograph::json;

    fn it_reads_dataset(dataset: &str) {
        let json_data = fs::read_to_string(format!("data/{}.json", dataset)).unwrap();
        let json_geograph = serde_json::from_str::<json::JsonGeograph>(&json_data).unwrap();
        assert_eq!(json_geograph.geograph, dataset);
        assert!(json_geograph.nodes.len() > 1);
    }

    #[test]
    fn it_reads_marnet() {
        it_reads_dataset("marnet");
    }
    #[test]
    fn it_reads_highways() {
        it_reads_dataset("highways");
    }
    #[test]
    fn it_reads_railways() {
        it_reads_dataset("railways");
    }
}
