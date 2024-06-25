
mod huffman_tree_test {
    use algorithm_lib::huffman_tree;
    use algorithm_lib::huffman_tree::{TreeNode, NodeContent};
    
    #[test]
    fn test_profile() {
        let text = "the rain in spain stays mainly in the plain";
        let profile = huffman_tree::profile(text);
        assert_eq!(profile.get(&b't'), Some(&3));
        assert_eq!(profile.get(&b'h'), Some(&2));
        assert_eq!(profile.get(&b'e'), Some(&2));
        assert_eq!(profile.get(&b' '), Some(&8));
        assert_eq!(profile.get(&b'r'), Some(&1));
        assert_eq!(profile.get(&b'a'), Some(&5));
        assert_eq!(profile.get(&b'i'), Some(&6));
        assert_eq!(profile.get(&b'n'), Some(&6));
        assert_eq!(profile.get(&b's'), Some(&3));
        assert_eq!(profile.get(&b'p'), Some(&2));
        assert_eq!(profile.get(&b'y'), Some(&2));
        assert_eq!(profile.get(&b'm'), Some(&1));
        assert_eq!(profile.get(&b'l'), Some(&2));
        assert_eq!(profile.len(), 13);
    }
    
    #[test]
    fn test_build_tree() {
        let text = "aabbbccccdde";
        let profile = huffman_tree::profile(text);
        let tree = huffman_tree::build_tree(profile);
        assert_eq!(tree.count, 12);
        if let NodeContent::Children(left, right) = tree.content {
            assert_eq!(left.count, 5);
            if let NodeContent::Children(left_left, left_right) = left.content {
                assert_eq!(left_left.count, 2);
                if let NodeContent::Letter(left_left_letter) = left_left.content {
                    assert_eq!(left_left_letter, b'd');
                }
                assert_eq!(left_right.count, 3);
                if let NodeContent::Children(left_right_left, left_right_right) = left_right.content {
                    assert_eq!(left_right_left.count, 1);
                    if let NodeContent::Letter(left_right_left_letter) = left_right_left.content {
                        assert_eq!(left_right_left_letter, b'e');
                    }
                    assert_eq!(left_right_right.count, 2);
                    if let NodeContent::Letter(left_right_right_letter) = left_right_right.content {
                        assert_eq!(left_right_right_letter, b'a');
                    }
                }
            }
            assert_eq!(right.count, 7);
            if let NodeContent::Children(right_left, right_right) = right.content {
                assert_eq!(right_left.count, 3);
                if let NodeContent::Letter(right_left_letter) = right_left.content {
                    assert_eq!(right_left_letter, b'b');
                }
                assert_eq!(right_right.count, 4);
                if let NodeContent::Letter(right_right_letter) = right_right.content {
                    assert_eq!(right_right_letter, b'c');
                }
            }
        }
    }

    //[Test]
    //public void Test3_CreateEncodingMap()
    //{
    //    var text = "the rain in spain stays mainly in the plain";
    //    var profile = HuffmanTree.Profile(text);
    //    var tree = HuffmanTree.BuildTree(profile);
    //    var map = HuffmanTree.CreateEncodingMap(tree);
    //    Assert.That(map['l'], Is.EqualTo("0000"));
    //    Assert.That(map['e'], Is.EqualTo("0001"));
    //    Assert.That(map['h'], Is.EqualTo("0010"));
    //    Assert.That(map['r'], Is.EqualTo("00110"));
    //    Assert.That(map['m'], Is.EqualTo("00111"));
    //    Assert.That(map['p'], Is.EqualTo("0100"));
    //    Assert.That(map['y'], Is.EqualTo("0101"));
    //    Assert.That(map['a'], Is.EqualTo("011"));
    //    Assert.That(map['s'], Is.EqualTo("1000"));
    //    Assert.That(map['t'], Is.EqualTo("1001"));
    //    Assert.That(map['n'], Is.EqualTo("101"));
    //    Assert.That(map['i'], Is.EqualTo("110"));
    //    Assert.That(map[' '], Is.EqualTo("111"));
    //    Assert.That(map.Count, Is.EqualTo(13));
    //    Assert.Pass();
    //
    //}
    //
    //[Test]
    //public void Test4_Encode()
    //{
    //    var text = "the rain in spain stays mainly in the plain";
    //    var profile = HuffmanTree.Profile(text);
    //    var tree = HuffmanTree.BuildTree(profile);
    //    var map = HuffmanTree.CreateEncodingMap(tree);
    //    var encoded = HuffmanTree.Encode(text, map);
    //    Assert.That(encoded, Is.EqualTo("10010010000111100110011110101111110101111100001000111101011111000100101101011000111001110111101010000010111111010111110010010000111101000000011110101"));
    //    Assert.Pass();
    //
    //}
    //
    //[Test]
    //public void Test5_Decode()
    //{
    //    var text = "the rain in spain stays mainly in the plain";
    //    var profile = HuffmanTree.Profile(text);
    //    var tree = HuffmanTree.BuildTree(profile);
    //    var map = HuffmanTree.CreateEncodingMap(tree);
    //    var encoded = HuffmanTree.Encode(text, map);
    //    Assert.That(encoded, Is.EqualTo("10010010000111100110011110101111110101111100001000111101011111000100101101011000111001110111101010000010111111010111110010010000111101000000011110101"));
    //    var decoded = HuffmanTree.Decode(encoded, tree);
    //    Assert.That(decoded, Is.EqualTo("the rain in spain stays mainly in the plain"));
    //    Assert.Pass();
    //
    //}
}

