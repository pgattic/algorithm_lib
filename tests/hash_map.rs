
mod hash_map_tests {
    use algorithm_lib::HashMap;

    #[test]
    fn insert_and_retrieve() {
        let mut my_map: HashMap<&str> = HashMap::new();
        my_map.insert("Preston".to_string(), "Corless");
        assert_eq!(my_map.get("Preston".to_string()), Some("Corless"));
    }

    #[test]
    fn not_present() {
        let mut my_map: HashMap<&str> = HashMap::new();
        my_map.insert("Preston".to_string(), "Corless");
        assert_eq!(my_map.get("notserP".to_string()), None);
    }

    #[test]
    fn key_removal() {
        let mut my_map: HashMap<&str> = HashMap::new();
        my_map.insert("Preston".to_string(), "Corless");
        let val = my_map.remove("Preston".to_string());
        assert_eq!(my_map.get("Preston".to_string()), None);
        assert_eq!(val, Some("Corless"));
    }

    #[test]
    fn hash_collision() {
        let mut my_map: HashMap<&str> = HashMap::new();
        my_map.insert("Preston".to_string(), "Corless");
        my_map.insert("notserP".to_string(), "sselroC");
        assert_eq!(my_map.get("Preston".to_string()), Some("Corless"));
        assert_eq!(my_map.get("notserP".to_string()), Some("sselroC"));
        assert_eq!(my_map.get("rPeston".to_string()), None);
    }

    #[test]
    fn insert_overwrite() {
        let mut my_map: HashMap<&str> = HashMap::new();
        my_map.insert("Skylar".to_string(), "Marriott");
        my_map.insert("Skylar".to_string(), "Corless");
        assert_eq!(my_map.get("Skylar".to_string()), Some("Corless"));
    }
}

