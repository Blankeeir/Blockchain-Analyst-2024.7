use anyhow::Result;
use plonky2::field::types::Field;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target; // Correct import for Target
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::{CircuitConfig, CommonCircuitData};
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig, PoseidonGoldilocksConfig};
use plonky2::recursion::cyclic_recursion::check_cyclic_proof_verifier_data;
use plonky2::recursion::dummy_circuit::cyclic_base_proof;
use std::time::SystemTime;
use hashbrown::HashMap;

// Function to create a factorization circuit
fn create_factorization_circuit<F: RichField + Extendable<D>, C: GenericConfig<D, F = F>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    _N: F, // Prefixed with underscore to suppress unused variable warning
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

    (a, b, ab)
}

// Function to create common circuit data for recursion with factorization
fn common_data_for_recursion<F: RichField + Extendable<D>, C: GenericConfig<D, F = F>, const D: usize>(
    N: F,
) -> (CommonCircuitData<F, D>, Target, Target)
where
    C::Hasher: AlgebraicHasher<F>,
{
    // Define the factorization circuit
    let config = CircuitConfig::standard_recursion_config();
    let mut sub_builder = CircuitBuilder::<F, D>::new(config);
    let (a, b, ab) = create_factorization_circuit::<F, C, D>(&mut sub_builder, N);

    // Connect ab to public input
    let n = sub_builder.add_virtual_public_input();
    sub_builder.connect(ab, n);

    // Build the sub-circuit
    let common_data = sub_builder.build::<C>().common;

    (common_data, a, b)
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

    // Building the Main Circuit
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // Defining Public Inputs
    let initial_value = builder.add_virtual_public_input(); // Public input 0: N
    let output_value = builder.add_virtual_public_input();  // Public input 1: N (for demonstration)
    let verifier_data_target = builder.add_verifier_data_public_inputs();

    // Generating Common Data for Recursion with Factorization
    let (common_data, a, b) = common_data_for_recursion::<F, C, D>(N);

    // Update number of public inputs in common_data
    let mut common_data = common_data;
    common_data.num_public_inputs = builder.num_public_inputs();

    // Adding a Boolean Target for Proof Verification
    let verify_proofs = builder.add_virtual_bool_target_safe();

    // Unpacking Inner Proof's Public Inputs
    let inner_cyclic_proof_with_pis = builder.add_virtual_proof_with_pis(&common_data);
    let inner_cyclic_pis = &inner_cyclic_proof_with_pis.public_inputs;
    let inner_cyclic_initial_value = inner_cyclic_pis[0];
    let inner_cyclic_output_value = inner_cyclic_pis[1];

    // Unpacking Other Proof's Public Inputs
    let other_proof_with_pis = builder.add_virtual_proof_with_pis(&common_data);
    let other_pis = &other_proof_with_pis.public_inputs;
    let _other_initial_value = other_pis[0];
    let other_output_value = other_pis[1];
    
    // Connecting Initial Values
    let n = builder.add_virtual_public_input(); // Public input 2: N (from sub-circuit)
    builder.connect(initial_value, inner_cyclic_initial_value);

    // Conditional Input Selection
    let actual_value_in = builder.select(verify_proofs, inner_cyclic_output_value, initial_value);

    // Calculating New Output Value
    let new_output_value = builder.mul_add(verify_proofs.target, other_output_value, actual_value_in);
    builder.connect(output_value, new_output_value);
    
    // Conditional Proof Verifications
    builder.conditionally_verify_cyclic_proof_or_dummy::<C>(
        verify_proofs,
        &inner_cyclic_proof_with_pis,
        &common_data,
    )?;
    builder.conditionally_verify_cyclic_proof_or_dummy::<C>(
        verify_proofs,
        &other_proof_with_pis,
        &common_data,
    )?;

    // Building the Final Circuit
    let cyclic_circuit_data = builder.build::<C>();
    let check1 = SystemTime::now();
    println!(
        "Constructed Circuit Builder in {} seconds",
        check1.duration_since(check0).unwrap().as_secs()
    );

    // Constructing the Base Proof (Factorization of N)
    let mut pw = PartialWitness::new();
    
    // Assigning Values: a = 3, b = 5 (since 3 * 5 = 15)
    let a_value = F::from_canonical_u32(3);
    let b_value = F::from_canonical_u32(5);

    // Setting Private Inputs: a and b
    pw.set_target(a, a_value);
    pw.set_target(b, b_value);

    // Setting Public Inputs: N
    pw.set_target(initial_value, N); // Updated line

    // Setting Proof Verification Boolean to False (Base Case)
    pw.set_bool_target(verify_proofs, false);

    // Creating empty HashMap for public inputs (base case)
    let empty_pis: HashMap<usize, F> = HashMap::new();

    // Setting Proofs to Dummy (Base Case)
    pw.set_proof_with_pis_target::<C, D>(
        &inner_cyclic_proof_with_pis,
        &cyclic_base_proof(
            &common_data,
            &cyclic_circuit_data.verifier_only,
            empty_pis.clone(), // No public inputs for base case
        ),
    );
    pw.set_proof_with_pis_target::<C, D>(
        &other_proof_with_pis,
        &cyclic_base_proof(
            &common_data,
            &cyclic_circuit_data.verifier_only,
            empty_pis, // No public inputs for base case
        ),
    );

    // Setting Verifier Data
    pw.set_verifier_data_target(&verifier_data_target, &cyclic_circuit_data.verifier_only);

    // Proving the Base Proof
    let proof = cyclic_circuit_data.prove(pw)?;
    let check2 = SystemTime::now();
    println!(
        "Constructed Base Proof in {} seconds",
        check2.duration_since(check1).unwrap().as_secs()
    );

    // Verifying the Base Proof
    check_cyclic_proof_verifier_data(
        &proof,
        &cyclic_circuit_data.verifier_only,
        &cyclic_circuit_data.common,
    )?;
    cyclic_circuit_data.verify(proof.clone())?;
    let check3 = SystemTime::now();
    println!(
        "Verified Base Proof in {} seconds",
        check3.duration_since(check2).unwrap().as_secs()
    );
    
    // Constructing the First-Level Proof (Recursive Proof)
    let mut pw = PartialWitness::new();
    
    // Setting Public Inputs: [0] = N, [1] = N
    pw.set_target(initial_value, N);  // Updated line
    pw.set_target(output_value, N);   // Updated line

    // Setting Proof Verification Boolean to True (Recursive Case)
    pw.set_bool_target(verify_proofs, true);

    // Linking Inner Proofs to the Base Proof
    pw.set_proof_with_pis_target::<C, D>(&inner_cyclic_proof_with_pis, &proof);
    pw.set_proof_with_pis_target::<C, D>(&other_proof_with_pis, &proof);

    // Setting Verifier Data
    pw.set_verifier_data_target(&verifier_data_target, &cyclic_circuit_data.verifier_only);

    // Proving the Recursive Proof
    let recursive_proof = cyclic_circuit_data.prove(pw)?;
    let check4 = SystemTime::now();
    println!(
        "Constructed Recursive Proof in {} seconds",
        check4.duration_since(check3).unwrap().as_secs()
    );

    // Verifying the Recursive Proof
    check_cyclic_proof_verifier_data(
        &recursive_proof,
        &cyclic_circuit_data.verifier_only,
        &cyclic_circuit_data.common,
    )?;
    cyclic_circuit_data.verify(recursive_proof.clone())?;
    let check5 = SystemTime::now();
    println!(
        "Verified Recursive Proof in {} seconds",
        check5.duration_since(check4).unwrap().as_secs()
    );

    Ok(())
}
