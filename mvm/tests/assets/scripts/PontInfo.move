script {
    use PontemFramework::Pontem;
    use PontemFramework::PONT::PONT;

    fun pont_info(expected: u128) {
        assert(expected == Pontem::market_cap<PONT>(), 1);
    }
}
