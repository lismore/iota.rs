pub mod testsuite {
    use curl::*;

    #[cfg(test)]
    mod inner {
        use super::*;
        use trytes::*;

        pub fn hash_works(curl: &mut Curl)
            where Curl<T>: Sponge
        {
            let trans: Trinary = "9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              999999999999999999999999999999T999999999999999999999999999999\
                              99999999999999999999999OLOB99999999999999999999999"
                    .chars()
                    .collect();

            let ex_hash: Trinary = "TAQCQAEBHLLYKAZWMNSXUPWQICMFSKWPEGQBNM9AQMGLFZGME9REOZTQIJQRKYH\
                             DANIYSMFYPVABX9999"
                    .chars()
                    .collect();

            curl.absorb(&trans.trits());
            let hash: Trinary = curl.squeeze(HASH_LENGTH).into_iter().collect();

            assert_eq!(hash, ex_hash);
        }
    }

    #[cfg(test)]
    pub fn run<C: Curl + Default>() {
        let new_curl = || C::default();

        // run tests
        inner::hash_works(&mut new_curl());
    }

}
