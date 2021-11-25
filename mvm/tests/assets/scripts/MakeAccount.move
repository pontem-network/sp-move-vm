script {
    use DiemFramework::DiemAccount;
    use DiemFramework::PONT::PONT;

    fun make_account(root: signer, addr: address) {
        DiemAccount::create_child_vasp_account<PONT>(&root, addr, x"", true);
    }
}