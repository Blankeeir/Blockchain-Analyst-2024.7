#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate bellman;
extern crate pairing;
extern crate rand;
extern crate serde;
extern crate serde_json;

use bellman::{
    gadgets::multipack,
    groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof,
    },
    Circuit, ConstraintSystem, SynthesisError,
};
use pairing::bls12_381::{Bls12, Fr};
use pairing::Engine;
use rand::OsRng;
use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};

// Define the Factorization Circuit
#[derive(Clone)]
struct FactorizationCircuit {
    // Public input: N
    n: Option<Fr>,
    // Private inputs: p and q
    p: Option<Fr>,
    q: Option<Fr>,
}

impl<E: Engine> Circuit<E> for FactorizationCircuit
where
    E::Fr: ff::PrimeField,
{
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
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

// Define ProofJson for serializing proofs
#[derive(Serialize, Deserialize)]
struct ProofJson {
    a: [String; 2],
    b: [[String; 2]; 2],
    c: [String; 2],
}

// Define PublicJson for serializing public inputs
#[derive(Serialize, Deserialize)]
struct PublicJson {
    N: String,
}

// Define the VerifierCircuit
use bellman::{
    groth16::Proof as Groth16Proof,
    Circuit, ConstraintSystem, SynthesisError,
};
use ff::PrimeField;

/// A placeholder verifier circuit that simulates proof verification.
/// TODO: Implement the actual Groth16 verification logic within the circuit.
#[derive(Clone)]
pub struct VerifierCircuit<E: Engine> {
    pub proof: Option<Groth16Proof<E>>,
    pub public_input: Option<E::Fr>,
}

impl<E: Engine> Circuit<E> for VerifierCircuit<E>
where
    E::Fr: ff::PrimeField,
{
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        // Get proof components
        let a = self.proof.as_ref().map(|p| p.a);
        let b = self.proof.as_ref().map(|p| p.b);
        let c = self.proof.as_ref().map(|p| p.c);

        // Allocate proof.a as public inputs
        let a0 = cs.alloc_input(
            || "proof.a.0",
            || a.ok_or(SynthesisError::AssignmentMissing).map(|p| p.x.into()),
        )?;
        let a1 = cs.alloc_input(
            || "proof.a.1",
            || a.ok_or(SynthesisError::AssignmentMissing).map(|p| p.y.into()),
        )?;

        // Allocate proof.b as public inputs
        let b00 = cs.alloc_input(
            || "proof.b.0.0",
            || b.ok_or(SynthesisError::AssignmentMissing).map(|p| p.x.c0.into()),
        )?;
        let b01 = cs.alloc_input(
            || "proof.b.0.1",
            || b.ok_or(SynthesisError::AssignmentMissing).map(|p| p.x.c1.into()),
        )?;
        let b10 = cs.alloc_input(
            || "proof.b.1.0",
            || b.ok_or(SynthesisError::AssignmentMissing).map(|p| p.y.c0.into()),
        )?;
        let b11 = cs.alloc_input(
            || "proof.b.1.1",
            || b.ok_or(SynthesisError::AssignmentMissing).map(|p| p.y.c1.into()),
        )?;

        // Allocate proof.c as public inputs
        let c0 = cs.alloc_input(
            || "proof.c.0",
            || c.ok_or(SynthesisError::AssignmentMissing).map(|p| p.x.into()),
        )?;
        let c1 = cs.alloc_input(
            || "proof.c.1",
            || c.ok_or(SynthesisError::AssignmentMissing).map(|p| p.y.into()),
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

        generate_random_parameters::<Bls12, _, _>(empty_circuit, &mut rng)
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
    // Since VerifyingKey does not implement Serialize, define a custom struct
    #[derive(Serialize, Deserialize)]
    struct VerifyingKeyJson {
        alpha: [String; 2],
        beta: [[String; 2]; 2],
        gamma: [String; 2],
        delta: [[String; 2]; 2],
        gamma_abc: Vec<String>,
    }

    // Extract components from vk and serialize
    let vk_json = VerifyingKeyJson {
        alpha: [
            format!("{}", vk.alpha.x),
            format!("{}", vk.alpha.y),
        ],
        beta: [
            [
                format!("{}", vk.beta.0.x),
                format!("{}", vk.beta.0.y),
            ],
            [
                format!("{}", vk.beta.1.x),
                format!("{}", vk.beta.1.y),
            ],
        ],
        gamma: [
            format!("{}", vk.gamma.x),
            format!("{}", vk.gamma.y),
        ],
        delta: [
            [
                format!("{}", vk.delta.0.x),
                format!("{}", vk.delta.0.y),
            ],
            [
                format!("{}", vk.delta.1.x),
                format!("{}", vk.delta.1.y),
            ],
        ],
        gamma_abc: vk.gamma_abc.iter().map(|v| format!("{}", v)).collect(),
    };

    let vk_file = File::create("verification_key.json").expect("Unable to create verification_key.json");
    serde_json::to_writer_pretty(pretty_print_writer(vk_file), &vk_json)
        .expect("Unable to write verification key");
    println!("Verification key saved to verification_key.json");

    println!("Creating proof for Factorization Circuit...");
    let proof = create_random_proof(circuit, &params, &mut rng).expect("Proof generation failed");

    let proof_json = ProofJson {
        a: [
            format!("{}", proof.a.x),
            format!("{}", proof.a.y),
        ],
        b: [
            [
                format!("{}", proof.b.x.c0),
                format!("{}", proof.b.x.c1),
            ],
            [
                format!("{}", proof.b.y.c0),
                format!("{}", proof.b.y.c1),
            ],
        ],
        c: [
            format!("{}", proof.c.x),
            format!("{}", proof.c.y),
        ],
    };

    let public_json = PublicJson {
        N: format!("{}", n_fr),
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

    let is_valid = verify_proof(&pvk, &proof, &[n_fr]).expect("Verification failed");

    println!("Proof is valid: {:?}", is_valid);

    // Create the VerifierCircuit instance
    let verifier_circuit_instance = VerifierCircuit::<Bls12> {
        proof: Some(proof.clone()),
        public_input: Some(Fr::one()), // As per the placeholder constraint
    };

    println!("Generating parameters for Verifier Circuit...");
    let verifier_params = {
        let empty_verifier_circuit = VerifierCircuit::<Bls12> {
            proof: None,
            public_input: None,
        };
        generate_random_parameters::<Bls12, _, _>(empty_verifier_circuit, &mut rng)
            .expect("Verifier parameter generation failed")
    };

    let verifier_params_file = File::create("verifier_params.bin").expect("Unable to create verifier_params.bin");
    verifier_params
        .write(&mut std::io::BufWriter::new(verifier_params_file))
        .expect("Unable to write verifier parameters");
    println!("Verifier parameters saved to verifier_params.bin");

    // Serialize verifier_vk
    let verifier_vk_json = VerifyingKeyJson {
        alpha: [
            format!("{}", verifier_params.vk.alpha.x),
            format!("{}", verifier_params.vk.alpha.y),
        ],
        beta: [
            [
                format!("{}", verifier_params.vk.beta.0.x),
                format!("{}", verifier_params.vk.beta.0.y),
            ],
            [
                format!("{}", verifier_params.vk.beta.1.x),
                format!("{}", verifier_params.vk.beta.1.y),
            ],
        ],
        gamma: [
            format!("{}", verifier_params.vk.gamma.x),
            format!("{}", verifier_params.vk.gamma.y),
        ],
        delta: [
            [
                format!("{}", verifier_params.vk.delta.0.x),
                format!("{}", verifier_params.vk.delta.0.y),
            ],
            [
                format!("{}", verifier_params.vk.delta.1.x),
                format!("{}", verifier_params.vk.delta.1.y),
            ],
        ],
        gamma_abc: verifier_params.vk.gamma_abc.iter().map(|v| format!("{}", v)).collect(),
    };

    let verifier_vk_file = File::create("verifier_verification_key.json").expect("Unable to create verifier_verification_key.json");
    serde_json::to_writer_pretty(pretty_print_writer(verifier_vk_file), &verifier_vk_json)
        .expect("Unable to write verifier verification key");
    println!("Verifier verification key saved to verifier_verification_key.json");

    println!("Creating recursive proof...");
    let recursive_proof = create_random_proof(verifier_circuit_instance, &verifier_params, &mut rng)
        .expect("Recursive proof generation failed");

    let recursive_proof_json = ProofJson {
        a: [
            format!("{}", recursive_proof.a.x),
            format!("{}", recursive_proof.a.y),
        ],
        b: [
            [
                format!("{}", recursive_proof.b.x.c0),
                format!("{}", recursive_proof.b.x.c1),
            ],
            [
                format!("{}", recursive_proof.b.y.c0),
                format!("{}", recursive_proof.b.y.c1),
            ],
        ],
        c: [
            format!("{}", recursive_proof.c.x),
            format!("{}", recursive_proof.c.y),
        ],
    };

    let recursive_public_json = PublicJson {
        N: format!("{}", Fr::one()), // As per the placeholder constraint in VerifierCircuit
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
    println!("Recursive proof is valid: {:?}", is_recursive_valid);
}

// Pretty print JSON
fn pretty_print_writer(file: File) -> std::io::BufWriter<File> {
    std::io::BufWriter::new(file)
}
