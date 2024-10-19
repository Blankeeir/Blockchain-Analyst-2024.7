#![allow(unused_imports)]
#![allow(unused_variables)]

use bellman::{
    gadgets::multipack,
    groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof,
    },
    Circuit, ConstraintSystem, SynthesisError,
};
use pairing::bn256::{Bn256, Fr};
use pairing::Engine;
use rand::os::OsRng; // Correct import for rand 0.4.6
use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};

mod verifier_circuit;

#[derive(Clone)]
struct FactorizationCircuit {
    // Public input: N
    n: Option<Fr>,
    // Private inputs: p and q
    p: Option<Fr>,
    q: Option<Fr>,
}

impl<E: Engine> Circuit<E::Fr> for FactorizationCircuit {
    fn synthesize<CS: ConstraintSystem<E::Fr>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        // Allocate public input: N
        let n_var = cs.alloc_input(
            || "N",
            || self.n.ok_or(SynthesisError::AssignmentMissing),
        )?;

        // Allocate private input: p
        let p_var = cs.alloc(
            || "p",
            || self.p.ok_or(SynthesisError::AssignmentMissing),
        )?;

        // Allocate private input: q
        let q_var = cs.alloc(
            || "q",
            || self.q.ok_or(SynthesisError::AssignmentMissing),
        )?;

        // Enforce p * q = N
        cs.enforce(
            || "p * q = N",
            |lc| lc + p_var,
            |lc| lc + q_var,
            |lc| lc + n_var,
        );

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct ProofJson {
    a: [String; 2],
    b: [[String; 2]; 2],
    c: [String; 2],
}

#[derive(Serialize, Deserialize)]
struct PublicJson {
    N: String,
}

fn main() {
    // Initialize the random number generator
    let mut rng = OsRng::new().expect("OsRng initialization failed");

    // Example values; adjust as needed
    let p = 17u64;
    let q = 23u64;
    let n = p * q;

    // Convert to Fr
    let p_fr = Fr::from_str(&p.to_string()).expect("Invalid Fr");
    let q_fr = Fr::from_str(&q.to_string()).expect("Invalid Fr");
    let n_fr = Fr::from_str(&n.to_string()).expect("Invalid Fr");

    let circuit = FactorizationCircuit {
        n: Some(n_fr),
        p: Some(p_fr),
        q: Some(q_fr),
    };

    println!("Generating parameters for Factorization Circuit...");

    // Create random parameters for the circuit
    let params = {
        let empty_circuit = FactorizationCircuit {
            n: None,
            p: None,
            q: None,
        };

        generate_random_parameters::<Bn256, _, _>(empty_circuit, &mut rng)
            .expect("Parameter generation failed")
    };

    // Save the parameters to a file for later use
    let params_file = File::create("params.bin").expect("Unable to create params.bin");
    params
        .write(&mut std::io::BufWriter::new(params_file))
        .expect("Unable to write parameters");
    println!("Parameters saved to params.bin");

    // Prepare the verification key (for proof verification)
    let vk = params.vk.clone(); // Clone to keep original vk for serialization
    let pvk = prepare_verifying_key(&params.vk);

    // Save the verification key to a file
    let vk_file = File::create("verification_key.json").expect("Unable to create verification_key.json");
    serde_json::to_writer_pretty(pretty_print_writer(vk_file), &vk)
        .expect("Unable to write verification key");
    println!("Verification key saved to verification_key.json");

    println!("Creating proof for Factorization Circuit...");
    let proof = create_random_proof(circuit, &params, &mut rng).expect("Proof generation failed");

    let proof_json = ProofJson {
        a: [
            format!("{:?}", proof.a.0),
            format!("{:?}", proof.a.1),
        ],
        b: [
            [
                format!("{:?}", proof.b.0 .0),
                format!("{:?}", proof.b.0 .1),
            ],
            [
                format!("{:?}", proof.b.1 .0),
                format!("{:?}", proof.b.1 .1),
            ],
        ],
        c: [
            format!("{:?}", proof.c.0),
            format!("{:?}", proof.c.1),
        ],
    };

    let public_json = PublicJson {
        N: format!("{:?}", n_fr),
    };

    // Save proof.json and other generated proof files
    let proof_file = File::create("proof.json").expect("Unable to create proof.json");
    serde_json::to_writer_pretty(pretty_print_writer(proof_file), &proof_json)
        .expect("Unable to write proof");
    let public_file = File::create("public.json").expect("Unable to create public.json");
    serde_json::to_writer_pretty(pretty_print_writer(public_file), &public_json)
        .expect("Unable to write public inputs");
    println!("Proof and public inputs saved to proof.json and public.json");

    // Now verify the proof
    println!("Verifying proof...");

    let n_fr_verify = Fr::from_str(&n.to_string()).expect("Invalid Fr for verification");
    let is_valid = verify_proof(&pvk, &proof, &[n_fr_verify]).expect("Verification failed");

    println!("Proof is valid: {:?}", is_valid); // Changed to use {:?}

    // Create the VerifierCircuit instance
    let verifier_circuit_instance = verifier_circuit::VerifierCircuit::<Bn256> {
        proof: Some(proof.clone()),
        public_input: Some(n_fr_verify),
    };

    println!("Generating parameters for Verifier Circuit...");
    let verifier_params = {
        let empty_verifier_circuit = verifier_circuit::VerifierCircuit::<Bn256> {
            proof: None,
            public_input: None,
        };
        generate_random_parameters::<Bn256, _, _>(empty_verifier_circuit, &mut rng)
            .expect("Verifier parameter generation failed")
    };

    let verifier_params_file = File::create("verifier_params.bin").expect("Unable to create verifier_params.bin");
    verifier_params
        .write(&mut std::io::BufWriter::new(verifier_params_file))
        .expect("Unable to write verifier parameters");
    println!("Verifier parameters saved to verifier_params.bin");

    let verifier_vk = verifier_params.vk.clone();
    let verifier_pvk = prepare_verifying_key(&verifier_params.vk);

    let verifier_vk_file = File::create("verifier_verification_key.json").expect("Unable to create verifier_verification_key.json");
    serde_json::to_writer_pretty(pretty_print_writer(verifier_vk_file), &verifier_vk)
        .expect("Unable to write verifier verification key");
    println!("Verifier verification key saved to verifier_verification_key.json");

    println!("Creating recursive proof...");
    let recursive_proof = create_random_proof(verifier_circuit_instance, &verifier_params, &mut rng)
        .expect("Recursive proof generation failed");

    let recursive_proof_json = ProofJson {
        a: [
            format!("{:?}", recursive_proof.a.0),
            format!("{:?}", recursive_proof.a.1),
        ],
        b: [
            [
                format!("{:?}", recursive_proof.b.0 .0),
                format!("{:?}", recursive_proof.b.0 .1),
            ],
            [
                format!("{:?}", recursive_proof.b.1 .0),
                format!("{:?}", recursive_proof.b.1 .1),
            ],
        ],
        c: [
            format!("{:?}", recursive_proof.c.0),
            format!("{:?}", recursive_proof.c.1),
        ],
    };

    let recursive_public_json = PublicJson {
        N: format!("{:?}", Fr::one()), // As per the placeholder constraint in VerifierCircuit
    };

    let recursive_proof_file = File::create("recursive_proof.json").expect("Unable to create recursive_proof.json");
    serde_json::to_writer_pretty(pretty_print_writer(recursive_proof_file), &recursive_proof_json)
        .expect("Unable to write recursive proof");
    let recursive_public_file = File::create("recursive_public.json").expect("Unable to create recursive_public.json");
    serde_json::to_writer_pretty(pretty_print_writer(recursive_public_file), &recursive_public_json)
        .expect("Unable to write recursive public inputs");

    println!("Recursive proof and public inputs saved to recursive_proof.json and recursive_public.json");

    println!("Verifying recursive proof...");
    let is_recursive_valid = verify_proof(&verifier_pvk, &recursive_proof, &[Fr::one()])
        .expect("Recursive verification failed");
    println!("Recursive proof is valid: {:?}", is_recursive_valid); // Changed to use {:?}
}

// Pretty print JSON
fn pretty_print_writer(file: File) -> std::io::BufWriter<File> {
    std::io::BufWriter::new(file)
}
