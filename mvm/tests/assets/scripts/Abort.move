script {
    use Assets::Abort;

    fun error(_signer: signer) {
        Abort::error(13);
    }
}