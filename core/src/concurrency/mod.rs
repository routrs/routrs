use crate::{geograph::DistanceResult, prelude::*};
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
    fn par_distance(&self, legs: &[Leg<T>]) -> Vec<DistanceResult>;
}

impl<T: Geolocalizable + Send + Sync> ParallelDistanceCalculator<T> for Geograph {
    fn par_distance(&self, legs: &[Leg<T>]) -> Vec<DistanceResult> {
        legs.par_iter()
            .map(|leg| self.distance(leg.origin(), leg.destination()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MARITIME;

    #[test]
    fn it_calculates_distance() {
        let from: Geoloc = (40.6759, -74.0504); // USNYC
        let to: Geoloc = (41.0067858, 28.9732219); // TRIST
        let mut legs = Vec::new();
        for _ in 0..100 {
            legs.push(Leg((from, to)));
        }

        let results = MARITIME.par_distance(&legs);
        let first = results.first().unwrap().as_ref().unwrap();
        let (distance, path, path_type) = first;

        assert_eq!(results.len(), legs.len());
        assert_eq!(*distance, 9224.95741604269);
        assert_eq!(path.len(), 118);
        assert_eq!(*path_type, PathType::ViaWaypoints);
    }
}
