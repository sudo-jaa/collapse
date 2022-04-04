#[cfg(test)]
mod tests {
    use crate::hash::hash::hash_int;

    #[test]
    fn hash_from_int() {
        let h1 = hash_int(1);
        let h2 = hash_int(2);
        let h3 = hash_int(3);
        let h4 = hash_int(4);
        let h5 = hash_int(4213);
        let h6 = hash_int(7412213);

        assert_eq!(h1, 1742378985846435984);
        assert_eq!(h2, 16336925911988107921);
        assert_eq!(h3, 568126464209439262);
        assert_eq!(h4, 15565210526001683492);
        assert_eq!(h5, 11141868147914754390);
        assert_eq!(h6, 692891098868956402);
    }
}
