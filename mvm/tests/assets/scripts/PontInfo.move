script {
    use PontemFramework::Token;
    use PontemFramework::NOX::NOX;

    fun pont_info(expected: u128) {
        assert!(expected == Token::total_value<NOX>(), 1);
    }
}
