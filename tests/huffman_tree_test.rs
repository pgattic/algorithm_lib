
mod huffman_tree_test {
    use algorithm_lib::huffman_tree;
    
    #[test]
    fn test_profile() {
        let text = "the rain in spain stays mainly in the plain";
        let profile = huffman_tree::profile(text);
        assert_eq!(profile.get(&'t'), Some(&3));
        assert_eq!(profile.get(&'h'), Some(&2));
        assert_eq!(profile.get(&'e'), Some(&2));
        assert_eq!(profile.get(&' '), Some(&8));
        assert_eq!(profile.get(&'r'), Some(&1));
        assert_eq!(profile.get(&'a'), Some(&5));
        assert_eq!(profile.get(&'i'), Some(&6));
        assert_eq!(profile.get(&'n'), Some(&6));
        assert_eq!(profile.get(&'s'), Some(&3));
        assert_eq!(profile.get(&'p'), Some(&2));
        assert_eq!(profile.get(&'y'), Some(&2));
        assert_eq!(profile.get(&'m'), Some(&1));
        assert_eq!(profile.get(&'l'), Some(&2));
        assert_eq!(profile.len(), 13);
    }
    
    #[test]
    fn test_encode() {
        let text = "the rain in spain stays mainly in the plain";
        let profile = huffman_tree::profile(text);
        let tree = huffman_tree::build_tree(&profile);
        let map = huffman_tree::create_encoding_map(&tree);
        let encoded = huffman_tree::encode(text, &map).unwrap();
        assert_eq!(encoded.len(), 149); // Optimal encoded size, in bits
    }

    #[test]
    fn test_encode_decode() {
        let text = "the rain in spain stays mainly in the plain";
        let profile = huffman_tree::profile(text);
        let tree = huffman_tree::build_tree(&profile);
        let map = huffman_tree::create_encoding_map(&tree);
        let encoded = huffman_tree::encode(text, &map).unwrap();
        // Verify that the same data comes back out
        assert_eq!(huffman_tree::decode(&encoded, &tree).unwrap(), text);
    }
}

