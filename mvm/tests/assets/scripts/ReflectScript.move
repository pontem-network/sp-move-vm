script {
    use Std::Reflect;

    fun test_reflect<Type>(expected_addr: address) {
        assert!(expected_addr == Reflect::mod_address_of<Type>(), 2);
    }
}
