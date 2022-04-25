
#[cfg(test)]
mod id_generator {
    //TODO add benchmark
    use crate::utils::IdGenerator;

    #[test]
    fn generate_id() {
        let mut generator = IdGenerator::default();
        let id = generator.next_id();
        assert_ne!(id, 0);
    }

    #[test]
    fn generate_multi_id() {
        let mut generator = IdGenerator::new(u16::MAX, u16::MAX);
        let mut ids = Vec::new();
        for _i in 1..32 {
            let id = generator.next_id();
            ids.push(id);
            assert_ne!(id, 0);
        }
        let mut ids_dedub = ids.clone();
        ids_dedub.dedup();
        assert_eq!(ids_dedub.len(), ids.len());
    }
}

#[cfg(test)]
mod pool {}