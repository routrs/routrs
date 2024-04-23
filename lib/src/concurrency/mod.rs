use crate::{geograph::ShortestPath, prelude::*};
pub use rayon::prelude::*;

pub struct Leg<T: Geolocalizable + Send>(pub (T, T));
impl<T: Geolocalizable + Send + Sync> Leg<T> {
    pub fn origin(&self) -> &T {
        &self.0 .0
    }

    pub fn destination(&self) -> &T {
        &self.0 .1
    }
}

pub trait ParallelDistanceCalculator<T: Geolocalizable + Send + Sync> {
    fn par_distance(&self, legs: &[Leg<T>]) -> Vec<ShortestPath>;
}

impl<T: Geolocalizable + Send + Sync> ParallelDistanceCalculator<T> for Geograph {
    fn par_distance(&self, legs: &[Leg<T>]) -> Vec<ShortestPath> {
        legs.par_iter()
            .map(|leg| self.shortest_path(leg.origin(), leg.destination()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geograph::{Geograph, Node};

    fn geograph_fixture() -> Geograph {
        let mut geograph = Geograph::new("Test Geograph");
        geograph
            .add(Node::new(0, (0.0, 0.0), vec![1]))
            .add(Node::new(1, (1.0, 1.0), vec![0, 2, 3, 4]))
            .add(Node::new(2, (2.0, 2.0), vec![1, 3, 5]))
            .add(Node::new(3, (3.0, 3.0), vec![1, 2, 3, 4, 5]))
            .add(Node::new(4, (4.0, 4.0), vec![1, 3, 4, 5]))
            .add(Node::new(5, (5.0, 5.0), vec![3, 4]));

        geograph
    }

    #[test]
    fn it_calculates_distance() {
        let geograph = geograph_fixture();
        let from: Geoloc = (40.6759, -74.0504); // USNYC
        let to: Geoloc = (41.0067858, 28.9732219); // TRIST
        let legs: Vec<_> = (0..100).map(|_| Leg((from, to))).collect();

        let shortest_paths = geograph.par_distance(&legs);
        let first = shortest_paths.first().unwrap();
        let (distance, path, path_type) = first;

        assert_eq!(shortest_paths.len(), legs.len());
        assert_eq!(*distance, 14116.87577572815);
        assert_eq!(path.len(), 6);
        assert_eq!(*path_type, PathType::ViaWaypoints);
    }
}
