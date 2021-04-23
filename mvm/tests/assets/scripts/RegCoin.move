script {
    use 0x1::Pontem;

   fun register_coin<Coin: copyable>(denom: vector<u8>, decimals: u8) {
        Pontem::register_coin<Coin>(denom, decimals);
   }
}