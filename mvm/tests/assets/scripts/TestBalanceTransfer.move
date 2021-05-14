script {
    use 0x1::PONT;
    use 0x1::Pontem;

    fun test_balance_transfer(alice: signer, bob: signer, amount: u128) {
        Pontem::register_coin<PONT::T>(x"", 2);

        assert(Pontem::get_native_balance<PONT::T>(&alice) >= amount, 1);
        assert(amount > 3, 2);

        let ponts_0 = Pontem::deposit_native<PONT::T>(&alice, amount - 3);
        let ponts_1 = Pontem::deposit_native<PONT::T>(&alice, 1);
        let ponts_2 = Pontem::deposit_native<PONT::T>(&alice, 1);
        let ponts_3 = Pontem::deposit_native<PONT::T>(&alice, 1);

        let ponts = Pontem::join(ponts_0, ponts_1);
        let ponts = Pontem::join(ponts, ponts_2);
        let ponts = Pontem::join(ponts, ponts_3);

        Pontem::store(&bob, ponts);
    }
}