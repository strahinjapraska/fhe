use crate::math::{ring::{add, binary_random_element, mul}, util::scale};

use super::{ciphertext::Ciphertext, params::Params, plaintext::Plaintext};
use rug::Integer;
pub struct SecretKey{
    pub(crate) secret: Vec<Integer>,  
    pub (crate) params: Params,  
}


impl SecretKey{
   
    pub(crate) fn new(params: &Params) -> SecretKey{
        let secret = binary_random_element(params.n);
        
        SecretKey{params: params.clone() , secret}
    }

    pub fn decrypt(&self, ct: &Ciphertext) -> Plaintext{
        let c0 = &ct.c0; 
        let c1 = &ct.c1; 

        Plaintext{message: 
            scale(
                &add(c0, 
                &mul(c1, &self.secret, &self.params.p, &self.params.w, &self.params.w_inv, &self.params.phi, &self.params.phi_inv),
                &self.params.p), 
            &self.params.p, &self.params.t 
            )
        }

    }

}


