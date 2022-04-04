#[cfg(test)]
mod tests {
    use crate::coordinates::coordinates::Coordinates;
    use std::borrow::BorrowMut;
    use uom::si::length::{kilometer, meter, Length};
    // use xorshift::{Rng, SeedableRng, Xoroshiro128};

    #[test]
    fn coordinates_hash_correctly() {
        let coordinates = Coordinates::new(0, 0, 0);
        assert_eq!(coordinates.hash, 2720667137516918932);
        assert_eq!(coordinates.x, 0);
        assert_eq!(coordinates.y, 0);
        assert_eq!(coordinates.z, 0);
    }

    #[test]
    fn coordinates_hash_diffrently() {
        let coordinatesA = Coordinates::new(-9, 13, 17);
        let coordinatesB = Coordinates::new(-9, 17, 13);
        assert_ne!(coordinatesA.hash, coordinatesB.hash);
    }

    #[test]
    fn coordinates_hash_correctly_with_negative_values() {
        let coordinates = Coordinates::new(-1, -2, -3);
        assert_eq!(coordinates.hash, 14052202922209838698);
        assert_eq!(coordinates.x, -1);
        assert_eq!(coordinates.y, -2);
        assert_eq!(coordinates.z, -3);
    }

    #[test]
    fn coordinates_equality_check_works() {
        let coordinates1 = Coordinates::new(1, 2, 3);
        let coordinates2 = Coordinates::new(1, 2, 3);
        assert_eq!(coordinates1, coordinates2);

        let coordinates3 = Coordinates::new(1, 3, 3);
        let coordinates4 = Coordinates::new(1, 2, 3);
        assert_ne!(coordinates3, coordinates4);
    }

    #[test]
    fn coordinates_distance() {
        let coordinates1 = Coordinates::new(1, 2, 3);
        let coordinates2 = Coordinates::new(9, 2, 8);

        assert_eq!(
            Coordinates::get_distance(&coordinates1, &coordinates2),
            9.433981
        )
    }

    #[test]
    fn coordinates_distance_zero() {
        let coordinates1 = Coordinates::new(1, 2, 3);
        let coordinates2 = Coordinates::new(1, 2, 3);

        assert_eq!(Coordinates::get_distance(&coordinates1, &coordinates2), 0.0)
    }

    #[test]
    fn coordinates_distance_negative() {
        let coordinates1 = Coordinates::new(-10, -7, -1);
        let coordinates2 = Coordinates::new(-3, -9, -1);

        assert_eq!(
            Coordinates::get_distance(&coordinates1, &coordinates2),
            7.28011
        )
    }

    #[test]
    fn coordinates_subtraction_test() {
        let coordinates1 = Coordinates::new(1, 1, 1);
        let coordinates2 = Coordinates::new(3, 3, 3);

        assert_eq!(coordinates1 - coordinates2, 3.4641016151377544);

        let coordinates1 = Coordinates::new(-21, -17, 11);
        let coordinates2 = Coordinates::new(32, 356, 0);

        assert_eq!(coordinates1 - coordinates2, 376.90715036995516)
    }
}
