fn main() {
	// Two secret prime numbers
    let p = 53;
	let q = 59;

	// Small prime number that satisfies:
	// - Not an integer
	// - not a factor of n
	// - 1 < e < Phi(n)
	// Totient Phi(n) = (p-1)(q-1)
	let tot = (p-1)*(q-1);
	let e = 3;
	let n = p * q;
	// Public key is n and e


	// Generate private key
	// compute d to satisfy the congruence relation
	// calculate d=(1+x * tot)/e to be an integer
	// private key is d, p and q


	// encryption
	// turn plaintext into a number m smaller than n, by using an agreed upon padding scheme.
	// ciphertext is 
	// let c = m ^ e mod n;


	// decryption
	// m^ed --- m mod pq
	// c^d --- m mod n
	// m = c^d mod n
}