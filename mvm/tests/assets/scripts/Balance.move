script {
    use 0x1::Coins;
    use 0x1::PONT;
    use 0x1::Pontem;

    fun test_balance(addr_1: signer, addr_2: signer, init_usdt: u128, init_pont: u128, init_btc: u128) {
        assert(Pontem::get_native_balance<Coins::USDT>(&addr_1) == init_usdt, 1);
        assert(Pontem::get_native_balance<PONT::T>(&addr_1) == init_pont, 2);
        assert(Pontem::get_native_balance<Coins::BTC>(&addr_1) == init_btc, 3);

        let move_usdt = init_usdt / 2;
        let usdt = Pontem::deposit_native<Coins::USDT>(&addr_1, move_usdt);
        Pontem::withdraw_native<Coins::USDT>(&addr_2, usdt);

        assert(Pontem::get_native_balance<Coins::USDT>(&addr_1) == init_usdt - move_usdt, 4);
        assert(Pontem::get_native_balance<Coins::USDT>(&addr_2) == move_usdt, 5);

        let pont_1 = Pontem::deposit_native<PONT::T>(&addr_1, 1);
        let pont_2 = Pontem::deposit_native<PONT::T>(&addr_1, 1);
        let pont_3 = Pontem::deposit_native<PONT::T>(&addr_1, 1);

        let pont_1 = Pontem::join(pont_1, pont_3);

        Pontem::withdraw_native<PONT::T>(&addr_2, pont_1);
        Pontem::withdraw_native<PONT::T>(&addr_2, pont_2);
        assert(Pontem::get_native_balance<PONT::T>(&addr_1) == init_pont - 3, 6);
        assert(Pontem::get_native_balance<PONT::T>(&addr_2) == 3, 6);
    }
}