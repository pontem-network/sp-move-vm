script {
    use DiemFramework::Diem;
    use DiemFramework::PONT::PONT;

    fun pont_info(expected: u128) {
        assert(expected == Diem::market_cap<PONT>(), 1);
    }
}