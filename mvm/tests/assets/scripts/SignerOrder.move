script {
    use Std::Signer;

    fun signer_order(s1: signer, s2: signer, s3: signer) {
        assert!(Signer::address_of(&s1) == @0x1, 1);
        assert!(Signer::address_of(&s2) == @0x2, 2);
        assert!(Signer::address_of(&s3) == @0x3, 3);
    }
}
