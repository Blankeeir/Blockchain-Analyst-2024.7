use bellman::{
    groth16::Proof,
    Circuit, ConstraintSystem, SynthesisError,
};
use pairing::Engine;
use ff::PrimeField;

#[derive(Clone)]
pub struct VerifierCircuit<E: Engine> {
    pub proof: Option<Proof<E>>,
    pub public_input: Option<E::Fr>,
}

impl<E: Engine> Circuit<E::Fr> for VerifierCircuit<E>
where
    E::Fr: PrimeField,
{
    fn synthesize<CS: ConstraintSystem<E::Fr>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        // Allocate proof elements as public inputs
        let a = self.proof.as_ref().map(|p| p.a);
        let b = self.proof.as_ref().map(|p| p.b);
        let c = self.proof.as_ref().map(|p| p.c);

        // Allocate proof.a as public inputs
        let a0 = cs.alloc_input(
            || "proof.a.0",
            || a.ok_or(SynthesisError::AssignmentMissing).map(|p| p.0),
        )?;
        let a1 = cs.alloc_input(
            || "proof.a.1",
            || a.ok_or(SynthesisError::AssignmentMissing).map(|p| p.1),
        )?;

        // Allocate proof.b as public inputs
        let b00 = cs.alloc_input(
            || "proof.b.0.0",
            || b.ok_or(SynthesisError::AssignmentMissing).map(|p| p.0 .0),
        )?;
        let b01 = cs.alloc_input(
            || "proof.b.0.1",
            || b.ok_or(SynthesisError::AssignmentMissing).map(|p| p.0 .1),
        )?;
        let b10 = cs.alloc_input(
            || "proof.b.1.0",
            || b.ok_or(SynthesisError::AssignmentMissing).map(|p| p.1 .0),
        )?;
        let b11 = cs.alloc_input(
            || "proof.b.1.1",
            || b.ok_or(SynthesisError::AssignmentMissing).map(|p| p.1 .1),
        )?;

        // Allocate proof.c as public inputs
        let c0 = cs.alloc_input(
            || "proof.c.0",
            || c.ok_or(SynthesisError::AssignmentMissing).map(|p| p.0),
        )?;
        let c1 = cs.alloc_input(
            || "proof.c.1",
            || c.ok_or(SynthesisError::AssignmentMissing).map(|p| p.1),
        )?;

        // Allocate the actual public input
        let pub_input = cs.alloc_input(
            || "public_input",
            || self.public_input.ok_or(SynthesisError::AssignmentMissing),
        )?;

        // Placeholder constraint: public_input == 1
        // This does not perform actual verification
        cs.enforce(
            || "public_input equals 1",
            |lc| lc + pub_input,
            |lc| lc + CS::one(),
            |lc| lc + CS::one(),
        );

        Ok(())
    }
}
