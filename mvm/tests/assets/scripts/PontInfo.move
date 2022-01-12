script {
    use PontemFramework::Token;
    use PontemFramework::PONT::PONT;

    fun pont_info(expected: u128) {
        assert(expected == Token::total_value<PONT>(), 1);
    }
}
