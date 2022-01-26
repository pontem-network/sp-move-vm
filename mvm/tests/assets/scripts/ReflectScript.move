script {
    use Std::Reflect;

    fun test_reflect<Type>(expected_addr: address, expected_mod_name: vector<u8>, expected_struct_name: vector<u8>) {
        assert!(expected_addr == Reflect::mod_address_of<Type>(), 2);
        assert!(expected_mod_name == Reflect::mod_name_of<Type>(), 3);
        assert!(expected_struct_name == Reflect::type_name_of<Type>(), 4);
    }
}
