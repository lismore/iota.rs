use curl::*;
use tmath::*;
use trytes::*;
use search::*;

pub struct CpuHam;

fn prepare_search<C: Curl<Trit>>(input: &[Trit], out: &mut [BCTrit], curl: &mut C) {
    let mut space = [0 as Trit; 128];

    let length_trits: &[Trit] = {
        let l = (input.len() / TRITS_PER_TRYTE) as isize;
        let min = num::min_trits(l);
        num::int2trits(l, &mut space[0..min]);
        &space[0..min]
    };

    curl.absorb(length_trits);
    curl.absorb(input);

    for (&t, mut bct) in curl.state().iter().zip(out.iter_mut()) {
        *bct = trit_to_bct(t);
    }

    (&mut out[0..4]).offset();
}

impl HammingNonce<Trit> for CpuHam {
    fn search<C: Curl<Trit>, CB: Curl<BCTrit>>(
        input: &[Trit],
        security: u8,
        length: usize,
        out: &mut [Trit],
        tcurl: &mut C,
        bcurl: &mut CB,
    ) -> Option<usize> {
        let mut bct: [BCTrit; STATE_LENGTH] = [(0, 0); STATE_LENGTH];

        prepare_search(input, &mut bct, tcurl);

        search_cpu(&mut bct, length, out, bcurl, 0, move |t: &[BCTrit]| {
            let mux = TrinaryDemultiplexer::new(t);
            for i in 0..mux.len() {
                if mux.get(i)
                    .take(security as usize * t.len() / 3)
                    .sum::<Trit>() == 0
                {
                    return Some(i);
                }
            }
            None
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpucurl::*;
    use curl_tests;

    #[test]
    pub fn run_testsuite() {
        curl_tests::run_ham_search::<CpuHam, CpuCurl<Trit>, CpuCurl<BCTrit>>();
    }

}
