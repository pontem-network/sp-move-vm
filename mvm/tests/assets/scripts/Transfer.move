script {
    use PontemFramework::PontAccount;
    use PontemFramework::PONT::PONT;
    use Std::Signer;
    use Std::Errors;

    fun transfer(from: signer, to: signer, from_balance: u64, to_balance: u64, to_move: u64) {
        assert!(PontAccount::balance<PONT>(Signer::address_of(&from)) == from_balance, Errors::custom(0));
        assert!(PontAccount::balance<PONT>(Signer::address_of(&to)) == to_balance, Errors::custom(1));

        PontAccount::pay_from<PONT>(&from, Signer::address_of(&to), to_move, x"");

        assert!(PontAccount::balance<PONT>(Signer::address_of(&from)) == from_balance - to_move, Errors::custom(2));
        assert!(PontAccount::balance<PONT>(Signer::address_of(&to)) == to_balance + to_move, Errors::custom(3));
    }
}
