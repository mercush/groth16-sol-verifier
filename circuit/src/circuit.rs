use ark_ff::Field;
use ark_relations::{
    lc, 
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};

pub fn mult<F: Field>(mut xl: F, xr: F) -> F {
    xl.mul_assign(&xr);
    xl
}

pub struct Circuit<F: Field> {
    pub xl: Option<F>,
    pub xr: Option<F>
}

impl<'a, F: Field> ConstraintSynthesizer<F> for Circuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {

        // Allocate the first component of the preimage.
        let xl_value = self.xl;
        let xl =
            cs.new_witness_variable(|| xl_value.ok_or(SynthesisError::AssignmentMissing))?;

        // Allocate the second component of the preimage.
        let xr_value = self.xr;
        let xr =
            cs.new_witness_variable(|| xr_value.ok_or(SynthesisError::AssignmentMissing))?;
        let res_value = xl_value.map(|mut e| {
            e.mul_assign(&xr_value.unwrap());
            e
        });
        let res = 
            cs.new_input_variable(|| res_value.ok_or(SynthesisError::AssignmentMissing))?;

        cs.enforce_constraint(
            lc!() + xl, 
            lc!() + xr, 
            lc!() + res)?;
        Ok(())
    }
}
