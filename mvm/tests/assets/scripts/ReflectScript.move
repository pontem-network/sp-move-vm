script {
    use Std::Reflect;

    fun test_reflect<Inst>(expected_addr: address, expected_mod_name: vector<u8>, expected_struct_name: vector<u8>) {
        let (addr, mod_name, struct_name) = Reflect::type_of<Inst>();
        assert(expected_addr == addr, 2);
        assert(expected_mod_name == mod_name, 3);
        assert(expected_struct_name == struct_name, 4);
    }
}
