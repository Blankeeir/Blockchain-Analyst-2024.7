import { SelfProof, Field, Experimental, verify } from 'snarkyjs';

const FibonacciSequence = Experimental.ZkProgram({
  publicInput: Field,

  methods: {
    // Base case: fib(0) = 0
    fib0: {
      privateInputs: [],
      method(fib: Field) {
        fib.assertEquals(Field(0));
      },
    },

    // Base case: fib(1) = 1
    fib1: {
      privateInputs: [],
      method(fib: Field) {
        fib.assertEquals(Field(1));
      },
    },

    // Recursive case: fib(n) = fib(n-1) + fib(n-2)
    inductiveFib: {
      privateInputs: [SelfProof, SelfProof],
      method(
        fib: Field, 
        fib1: SelfProof<Field, void>, 
        fib2: SelfProof<Field, void>
      ) {
        // Verify previous proofs
        fib1.verify();
        fib2.verify();

        // Calculate new Fibonacci number
        let newFib = fib1.publicInput.add(fib2.publicInput);
        fib.assertEquals(newFib);
      },
    },
  },
});

async function main() {
  console.log('Compiling...');
  const { verificationKey } = await FibonacciSequence.compile();
  console.log('Compilation finished');

  // Generate base case proofs
  let fib_n_2 = await FibonacciSequence.fib0(Field(0));
  let fib_n_1 = await FibonacciSequence.fib1(Field(1));
  
  // Calculate and prove Fibonacci sequence up to N
  let fib_n: any = fib_n_1; // Start with fib(1)
  const N = 10;

  for (let n = 2; n <= N; n++) {
    console.log(`Working on fib_${n}...`);
    
    // Calculate next Fibonacci number
    let publicInput = fib_n_1.publicInput.add(fib_n_2.publicInput);
    
    // Generate proof for this number
    fib_n = await FibonacciSequence.inductiveFib(
      publicInput,
      fib_n_1,
      fib_n_2
    );

    // Move proofs forward
    fib_n_2 = fib_n_1;
    fib_n_1 = fib_n;

    console.log(`Got fib_${n} = ${fib_n.publicInput.toString()}`);
  }

  // Verify the final proof
  console.log('Verifying...');
  if (fib_n) {
    let ok = await verify(fib_n, verificationKey);
    console.log(`Is ${fib_n.publicInput.toString()} in the sequence? ${ok}`);
  }
}

// Run the program
main().catch((err) => {
  console.error(err);
  process.exit(1);
});