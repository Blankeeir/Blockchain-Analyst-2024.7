use anyhow::Result;
use plonky2::field::extension::Extendable;
use plonky2::field::types::Field;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use std::time::SystemTime;

fn create_factorization_circuit<
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
    const D: usize,
>(
    builder: &mut CircuitBuilder<F, D>,
) -> (Target, Target, Target) {
    // Allocate private inputs (a and b)
    let a = builder.add_virtual_target();
    let b = builder.add_virtual_target();

    // Compute a * b
    let ab = builder.mul(a, b);

    // Allocate public input (N)
    let n = builder.add_virtual_public_input();

    // Enforce a * b = N
    builder.connect(ab, n);

    (a, b, n)
}

fn main() -> Result<()> {
    println!("Starting Recursive ZKP for Factorization");
    // Type Definitions
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    // Integer to Factorize (Example: 15 = 3 * 5)
    let N = F::from_canonical_u32(15);

    // Timing Checkpoints
    let check0 = SystemTime::now();

    // Circuit Configuration
    let config = CircuitConfig::standard_recursion_config();

    // Step 1: Build and Prove the Factorization Circuit

    // Build the factorization circuit
    let mut factor_builder = CircuitBuilder::<F, D>::new(config.clone());
    let (a_tgt, b_tgt, n_tgt) = create_factorization_circuit::<F, C, D>(&mut factor_builder);

    // Build the circuit data for the factorization circuit
    let factor_circuit_data = factor_builder.build::<C>();

    // Create a partial witness for the factorization circuit
    let mut factor_pw = PartialWitness::new();

    // Assign values: a = 3, b = 5 (since 3 * 5 = 15)
    let a_value = F::from_canonical_u32(3);
    let b_value = F::from_canonical_u32(5);
    let n_value = N; // N = 15

    // Set the private inputs
    factor_pw.set_target(a_tgt, a_value);
    factor_pw.set_target(b_tgt, b_value);

    // Set the public input N
    factor_pw.set_target(n_tgt, n_value);

    // Generate the base proof
    let factor_proof = factor_circuit_data.prove(factor_pw)?;
    let check1 = SystemTime::now();
    println!(
        "Constructed Base Proof in {} seconds",
        check1.duration_since(check0).unwrap().as_secs()
    );

    // Verify the base proof
    factor_circuit_data.verify(factor_proof.clone())?;
    let check2 = SystemTime::now();
    println!(
        "Verified Base Proof in {} seconds",
        check2.duration_since(check1).unwrap().as_secs()
    );

    // Step 2: Build the Recursive Circuit

    // Create a new circuit builder for the recursive circuit
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // Add the public input N to the recursive circuit (to match the public input in the base proof)
    let n_recursive = builder.add_virtual_public_input();

    // Add the verifier data as public inputs
    let verifier_data_target = builder.add_verifier_data_public_inputs();

    // Add the proof target for the base proof
    let proof_with_pis_target = builder.add_virtual_proof_with_pis(&factor_circuit_data.common);

    // Verify the base proof within the recursive circuit
    builder.verify_proof::<C>(&proof_with_pis_target, &verifier_data_target, &factor_circuit_data.common);

    // Enforce that the public input N in the recursive circuit matches the one in the base proof
    builder.connect(n_recursive, proof_with_pis_target.public_inputs[0]);

    // Build the recursive circuit data
    let recursive_circuit_data = builder.build::<C>();
    let check3 = SystemTime::now();
    println!(
        "Constructed Recursive Circuit in {} seconds",
        check3.duration_since(check2).unwrap().as_secs()
    );

    // Step 3: Generate and Verify the Recursive Proof

    // Create a partial witness for the recursive circuit
    let mut recursive_pw = PartialWitness::new();

    // Set the public input N in the recursive circuit
    recursive_pw.set_target(n_recursive, N);

    // Set the proof with public inputs target to the base proof we generated earlier
    recursive_pw.set_proof_with_pis_target(&proof_with_pis_target, &factor_proof);

    // Set the verifier data target
    recursive_pw.set_verifier_data_target(&verifier_data_target, &factor_circuit_data.verifier_only);

    // Generate the recursive proof
    let recursive_proof = recursive_circuit_data.prove(recursive_pw)?;
    let check4 = SystemTime::now();
    println!(
        "Constructed Recursive Proof in {} seconds",
        check4.duration_since(check3).unwrap().as_secs()
    );

    // Verify the recursive proof
    recursive_circuit_data.verify(recursive_proof.clone())?;
    let check5 = SystemTime::now();
    println!(
        "Verified Recursive Proof in {} seconds",
        check5.duration_since(check4).unwrap().as_secs()
    );

    println!("Recursive ZKP for Factorization completed successfully!");

    Ok(())
}