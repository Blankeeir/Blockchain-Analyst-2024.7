use std::rc::Rc;

use halo2_axiom::halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    dev::MockProver,
    plonk::*,
    poly::{
        commitment::ParamsProver,
        kzg::{
            commitment::{KZGCommitmentScheme, ParamsKZG},
            strategy::SingleStrategy,
        },
    },
    transcript::{Challenge255, TranscriptReadBuffer, TranscriptWriterBuffer},
};

use halo2_axiom::{
    halo2_curves::bn256::{Bn256, Fr, G1Affine},
    utils::calculate_k,
};
use halo2_axiom::{CircuitExt, Snark};

use rand::rngs::OsRng;

#[derive(Clone)]
struct FactorizationConfig {
    a: Column<Advice>,
    b: Column<Advice>,
    ab: Column<Advice>,
    n: Column<Instance>,
    selector: Selector,
}

#[derive(Clone)]
struct FactorizationCircuit {
    a: Value<Fr>,
    b: Value<Fr>,
    n: Fr,
}

impl Circuit<Fr> for FactorizationCircuit {
    type Config = FactorizationConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            a: Value::unknown(),
            b: Value::unknown(),
            n: Fr::zero(),
        }
    }

    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self::Config {
        let a = meta.advice_column();
        let b = meta.advice_column();
        let ab = meta.advice_column();
        let n = meta.instance_column();

        let selector = meta.selector();

        meta.enable_equality(a);
        meta.enable_equality(b);
        meta.enable_equality(ab);
        meta.enable_equality(n);

        meta.create_gate("a * b = ab", |meta| {
            let s = meta.query_selector(selector);
            let a = meta.query_advice(a, Rotation::cur());
            let b = meta.query_advice(b, Rotation::cur());
            let ab = meta.query_advice(ab, Rotation::cur());
            vec![s * (a * b - ab)]
        });

        meta.create_gate("ab = n", |meta| {
            let s = meta.query_selector(selector);
            let ab = meta.query_advice(ab, Rotation::cur());
            let n = meta.query_instance(n, Rotation::cur());
            vec![s * (ab - n)]
        });

        FactorizationConfig {
            a,
            b,
            ab,
            n,
            selector,
        }
    }

    fn synthesize(
        &self,
        config: FactorizationConfig,
        mut layouter: impl Layouter<Fr>,
    ) -> Result<(), Error> {
        let a_value = self.a;
        let b_value = self.b;

        layouter.assign_region(
            || "Multiply a and b",
            |mut region| {
                config.selector.enable(&mut region, 0)?;

                region.assign_advice(|| "a", config.a, 0, || a_value)?;
                region.assign_advice(|| "b", config.b, 0, || b_value)?;
                region.assign_advice(|| "ab", config.ab, 0, || {
                    a_value * b_value
                })?;

                Ok(())
            },
        )
    }
}

impl CircuitExt<Fr> for FactorizationCircuit {
    fn num_instance(&self) -> Vec<usize> {
        vec![1] // We have one public instance (n)
    }

    fn instances(&self) -> Vec<Vec<Fr>> {
        vec![vec![self.n]] // Provide the public instance
    }
}

fn test_factorization_circuit() {
    // Example values
    let a = Fr::from(3u64);
    let b = Fr::from(5u64);
    let n = Fr::from(15u64);

    // Create the circuit instance
    let circuit = FactorizationCircuit {
        a: Value::known(a),
        b: Value::known(b),
        n,
    };

    // Public inputs vector
    let public_inputs = vec![vec![n]];

    // Run the mock prover
    let k = calculate_k(5); // Calculate k based on the circuit constraints
    let prover = MockProver::run(k, &circuit, public_inputs).unwrap();
    prover.assert_satisfied();
}

fn create_and_verify_base_proof() -> (
    Snark<G1Affine>,
    Rc<ParamsKZG<Bn256>>,
) {
    // Example values
    let a = Fr::from(3u64);
    let b = Fr::from(5u64);
    let n = Fr::from(15u64);

    // Create the circuit instance
    let circuit = FactorizationCircuit {
        a: Value::known(a),
        b: Value::known(b),
        n,
    };

    // Initialize the proving parameters
    let k = calculate_k(5); // Calculate k based on the circuit constraints
    let params = Rc::new(ParamsKZG::<Bn256>::setup(k, OsRng));

    // Create the proving key
    let vk = keygen_vk(&params, &circuit).expect("keygen_vk should not fail");
    let pk = keygen_pk(&params, vk.clone(), &circuit).expect("keygen_pk should not fail");

    // Create the proof
    let mut transcript = halo2_axiom::transcript::PoseidonTranscript::<_, G1Affine, _>::new(Vec::new());
    create_proof(
        &params,
        &pk,
        &[circuit.clone()],
        &[&circuit.instances().iter().map(|x| &x[..]).collect::<Vec<_>>()[..]],
        OsRng,
        &mut transcript,
    ).expect("proof generation should not fail");
    let proof = transcript.finalize();

    let snark = Snark::new(vk.clone(), circuit.instances(), proof);

    // Verify the proof
    {
        let verifier_params = params.verifier_params();
        let strategy = SingleStrategy::new(&params);
        let mut transcript = halo2_axiom::transcript::PoseidonTranscript::<_, G1Affine, _>::from_reader(&snark.proof[..]);
        let result = verify_proof(
            verifier_params,
            &vk,
            strategy,
            &[&snark.instances.iter().map(|v| &v[..]).collect::<Vec<_>>()],
            &mut transcript,
        );
        assert!(result.is_ok(), "Base proof verification failed");
    }

    println!("Base proof created and verified successfully.");

    (snark, params)
}

fn create_and_verify_recursive_proof(
    snark: Snark<G1Affine>,
    params: Rc<ParamsKZG<Bn256>>,
) {
    use halo2_axiom::halo2_proofs::poly::commitment::ParamsVerifier;
    use halo2_axiom::aggregation::{AggregationCircuit, SelfVerificationCircuit};

    // Create the aggregation circuit instance
    let mut aggregation_circuit = AggregationCircuit::new(&params, &[snark.clone()]);

    // Initialize proving parameters for the aggregation circuit
    let k = aggregation_circuit.calculate_k(); // Calculate appropriate k
    let agg_params = Rc::new(ParamsKZG::<Bn256>::setup(k, OsRng));

    // Create the proving key for the aggregation circuit
    let vk = keygen_vk(&agg_params, &aggregation_circuit).expect("keygen_vk should not fail");
    let pk = keygen_pk(&agg_params, vk.clone(), &aggregation_circuit).expect("keygen_pk should not fail");

    // Create the recursive proof
    let mut transcript = halo2_axiom::transcript::PoseidonTranscript::<_, G1Affine, _>::new(Vec::new());
    create_proof(
        &agg_params,
        &pk,
        &[aggregation_circuit.clone()],
        &[],
        OsRng,
        &mut transcript,
    ).expect("proof generation should not fail");
    let proof = transcript.finalize();

    // Verify the recursive proof
    {
        let verifier_params = agg_params.verifier_params();
        let strategy = SingleStrategy::new(&agg_params);
        let mut transcript = halo2_axiom::transcript::PoseidonTranscript::<_, G1Affine, _>::from_reader(&proof[..]);
        let result = verify_proof(
            verifier_params,
            &vk,
            strategy,
            &[],
            &mut transcript,
        );
        assert!(result.is_ok(), "Recursive proof verification failed");
    }

    println!("Recursive proof created and verified successfully.");
}

fn main() {
    // Step 1: Test the factorization circuit
    test_factorization_circuit();
    println!("Factorization circuit satisfied the constraints.");

    // Step 2: Create and verify the base proof
    let (snark, params) = create_and_verify_base_proof();

    // Step 3: Create and verify the recursive proof
    create_and_verify_recursive_proof(snark, params);
}