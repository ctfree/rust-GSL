//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

/*!
# Random Number Generation

The library provides a large collection of random number generators which can be accessed through a uniform interface.
Environment variables allow you to select different generators and seeds at runtime, so that you can easily switch between generators without needing to recompile your program.
Each instance of a generator keeps track of its own state, allowing the generators to be used in multi-threaded programs.
Additional functions are available for transforming uniform random numbers into samples from continuous or discrete probability distributions such as the Gaussian, log-normal or Poisson distributions.

## General comments on random numbers

In 1988, Park and Miller wrote a paper entitled “Random number generators: good ones are hard to find.” [Commun. ACM, 31, 1192–1201]. Fortunately, some excellent random number generators are available, though poor ones are still in common use. You may be happy with the system-supplied random number generator on your computer, but you should be aware that as computers get faster, requirements on random number generators increase. Nowadays, a simulation that calls a random number generator millions of times can often finish before you can make it down the hall to the coffee machine and back.

A very nice review of random number generators was written by Pierre L’Ecuyer, as Chapter 4 of the book: Handbook on Simulation, Jerry Banks, ed. (Wiley, 1997). The chapter is available in postscript from L’Ecuyer’s ftp site (see references). Knuth’s volume on Seminumerical Algorithms (originally published in 1968) devotes 170 pages to random number generators, and has recently been updated in its 3rd edition (1997). It is brilliant, a classic. If you don’t own it, you should stop reading right now, run to the nearest bookstore, and buy it.

A good random number generator will satisfy both theoretical and statistical properties. Theoretical properties are often hard to obtain (they require real math!), but one prefers a random number generator with a long period, low serial correlation, and a tendency not to “fall mainly on the planes.” Statistical tests are performed with numerical simulations. Generally, a random number generator is used to estimate some quantity for which the theory of probability provides an exact answer. Comparison to this exact answer provides a measure of “randomness”.

## The Random Number Generator Interface

It is important to remember that a random number generator is not a “real” function like sine or cosine. Unlike real functions, successive calls to a random number generator yield different return values. Of course that is just what you want for a random number generator, but to achieve this effect, the generator must keep track of some kind of “state” variable.
Sometimes this state is just an integer (sometimes just the value of the previously generated random number), but often it is more complicated than that and may involve a whole array of numbers, possibly with some indices thrown in. To use the random number generators, you do not need to know the details of what comprises the state, and besides that varies from algorithm to algorithm.

The random number generator library uses two special structs, RngType which holds static information about each type of generator and Rng which describes an instance of a generator created from a given RngType.

## Performance

The following table shows the relative performance of a selection the available random number generators. The fastest simulation quality generators are taus, gfsr4 and mt19937. The generators which offer the best mathematically-proven quality are those based on the RANLUX algorithm.

 * 1754 k ints/sec,    870 k doubles/sec, taus
 * 1613 k ints/sec,    855 k doubles/sec, gfsr4
 * 1370 k ints/sec,    769 k doubles/sec, mt19937
 *  565 k ints/sec,    571 k doubles/sec, ranlxs0
 *  400 k ints/sec,    405 k doubles/sec, ranlxs1
 *  490 k ints/sec,    389 k doubles/sec, mrg
 *  407 k ints/sec,    297 k doubles/sec, ranlux
 *  243 k ints/sec,    254 k doubles/sec, ranlxd1
 *  251 k ints/sec,    253 k doubles/sec, ranlxs2
 *  238 k ints/sec,    215 k doubles/sec, cmrg
 *  247 k ints/sec,    198 k doubles/sec, ranlux389
 *  141 k ints/sec,    140 k doubles/sec, ranlxd2

## Random number environment variables

The library allows you to choose a default generator and seed from the environment variables GSL_RNG_TYPE and GSL_RNG_SEED and the function gsl_rng_env_setup. This makes it easy try out different generators and seeds without having to recompile your program.

## References and Further Reading

The subject of random number generation and testing is reviewed extensively in Knuth’s Seminumerical Algorithms.

Donald E. Knuth, The Art of Computer Programming: Seminumerical Algorithms (Vol 2, 3rd Ed, 1997), Addison-Wesley, ISBN 0201896842.
Further information is available in the review paper written by Pierre L’Ecuyer,

P. L’Ecuyer, “Random Number Generation”, Chapter 4 of the Handbook on Simulation, Jerry Banks Ed., Wiley, 1998, 93–137.
http://www.iro.umontreal.ca/~lecuyer/papers.html

The source code for the DIEHARD random number generator tests is also available online,

DIEHARD source code G. Marsaglia,
http://stat.fsu.edu/pub/diehard/
A comprehensive set of random number generator tests is available from NIST,

NIST Special Publication 800-22, “A Statistical Test Suite for the Validation of Random Number Generators and Pseudo Random Number Generators for Cryptographic Applications”.
http://csrc.nist.gov/rng/

## Acknowledgements

Thanks to Makoto Matsumoto, Takuji Nishimura and Yoshiharu Kurita for making the source code to their generators (MT19937, MM&TN; TT800, MM&YK) available under the GNU General Public License. Thanks to Martin Lüscher for providing notes and source code for the RANLXS and RANLXD generators.
!*/

use crate::Value;
use ffi::FFI;
use std::os::raw::c_ulong;

ffi_wrapper!(Rng, *mut sys::gsl_rng, gsl_rng_free);

impl Rng {
    /// This function returns a pointer to a newly-created instance of a random number generator of type T. For example, the following code creates an instance of the Tausworthe generator,
    ///
    /// ```Rust
    /// let r = Rng::new(gsl_rng_taus);
    /// ```
    ///
    /// If there is insufficient memory to create the generator then the function returns a null pointer and the error handler is invoked with an error code of GSL_ENOMEM.
    ///
    /// The generator is automatically initialized with the default seed, gsl_rng_default_seed. This is zero by default but can be changed either directly or by using the environment variable
    /// GSL_RNG_SEED (see [`Random number environment variables`](https://www.gnu.org/software/gsl/manual/html_node/Random-number-environment-variables.html#Random-number-environment-variables)).
    #[doc(alias = "gsl_rng_alloc")]
    pub fn new(T: RngType) -> Option<Rng> {
        let tmp = unsafe { sys::gsl_rng_alloc(T.unwrap_shared()) };

        if tmp.is_null() {
            None
        } else {
            Some(Rng::wrap(tmp))
        }
    }

    /// This function initializes (or ‘seeds’) the random number generator. If the generator is seeded with the same value of s on two different runs, the same stream of random numbers will be generated by successive calls to the routines below.
    /// If different values of s >= 1 are supplied, then the generated streams of random numbers should be completely different. If the seed s is zero then the standard seed from the original implementation is used instead.
    /// For example, the original Fortran source code for the ranlux generator used a seed of 314159265, and so choosing s equal to zero reproduces this when using gsl_rng_ranlux.
    ///
    /// When using multiple seeds with the same generator, choose seed values greater than zero to avoid collisions with the default setting.
    ///
    /// Note that the most generators only accept 32-bit seeds, with higher values being reduced modulo 2^32. For generators with smaller ranges the maximum seed value will typically be lower.
    #[doc(alias = "gsl_rng_set")]
    pub fn set(&mut self, s: usize) {
        unsafe { sys::gsl_rng_set(self.unwrap_unique(), s as _) }
    }

    /// This function returns a random integer from the generator r. The minimum and maximum values depend on the algorithm used, but all integers in the range [min,max] are equally likely.
    /// The values of min and max can be determined using the auxiliary functions gsl_rng_max (r) and gsl_rng_min (r).
    #[doc(alias = "gsl_rng_get")]
    pub fn get(&mut self) -> usize {
        unsafe { sys::gsl_rng_get(self.unwrap_shared()) as _ }
    }

    /// This function returns a double precision floating point number uniformly distributed in the range [0,1). The range includes 0.0 but excludes 1.0.
    /// The value is typically obtained by dividing the result of gsl_rng_get(r) by gsl_rng_max(r) + 1.0 in double precision.
    /// Some generators compute this ratio internally so that they can provide floating point numbers with more than 32 bits of randomness (the maximum number of bits that can be portably represented in a single unsigned long int).
    #[doc(alias = "gsl_rng_uniform")]
    pub fn uniform(&mut self) -> f64 {
        unsafe { sys::gsl_rng_uniform(self.unwrap_unique()) }
    }

    /// This function returns a positive double precision floating point number uniformly distributed in the range (0,1), excluding both 0.0 and 1.0.
    /// The number is obtained by sampling the generator with the algorithm of gsl_rng_uniform until a non-zero value is obtained.
    /// You can use this function if you need to avoid a singularity at 0.0.
    #[doc(alias = "gsl_rng_uniform_pos")]
    pub fn uniform_pos(&mut self) -> f64 {
        unsafe { sys::gsl_rng_uniform_pos(self.unwrap_unique()) }
    }

    /// This function returns a random integer from 0 to n-1 inclusive by scaling down and/or discarding samples from the generator r.
    /// All integers in the range [0,n-1] are produced with equal probability. For generators with a non-zero minimum value an offset is applied so that zero is returned with the correct probability.
    ///
    /// Note that this function is designed for sampling from ranges smaller than the range of the underlying generator. The parameter n must be less than or equal to the range of the generator r.
    /// If n is larger than the range of the generator then the function calls the error handler with an error code of GSL_EINVAL and returns zero.
    ///
    /// In particular, this function is not intended for generating the full range of unsigned integer values [0,2^32-1].
    /// Instead choose a generator with the maximal integer range and zero minimum value, such as gsl_rng_ranlxd1, gsl_rng_mt19937 or gsl_rng_taus, and sample it directly using gsl_rng_get. The range of each generator can be found using the auxiliary functions described in the next section.
    #[doc(alias = "gsl_rng_uniform_int")]
    pub fn uniform_int(&mut self, n: usize) -> usize {
        unsafe { sys::gsl_rng_uniform_int(self.unwrap_unique(), n as c_ulong) as _ }
    }

    /// This function returns a pointer to the name of the generator. For example,
    ///
    /// ```Rust
    /// println!("r is a '{}' generator", r.get_name());
    /// ```
    ///
    /// would print something like "r is a 'taus' generator".
    #[doc(alias = "gsl_rng_name")]
    pub fn name(&self) -> String {
        unsafe {
            let tmp = sys::gsl_rng_name(self.unwrap_shared());

            String::from_utf8_lossy(::std::ffi::CStr::from_ptr(tmp).to_bytes()).to_string()
        }
    }

    /// This function returns the largest value that the get function can return.
    #[doc(alias = "gsl_rng_max")]
    pub fn max(&self) -> usize {
        unsafe { sys::gsl_rng_max(self.unwrap_shared()) as _ }
    }

    /// This function returns the smallest value that gsl_rng_get can return. Usually this value is zero.
    /// There are some generators with algorithms that cannot return zero, and for these generators the minimum value is 1.
    #[doc(alias = "gsl_rng_min")]
    pub fn min(&self) -> usize {
        unsafe { sys::gsl_rng_min(self.unwrap_shared()) as _ }
    }

    /// This function returns a pointer to the state of generator r. You can use this information
    /// to access the state directly. For example, the following code will write the state of a
    /// generator to a stream,
    ///
    /// ```C
    /// void * state = gsl_rng_state (r);
    /// size_t n = gsl_rng_size (r);
    /// fwrite (state, n, 1, stream);
    /// ```
    #[doc(alias = "gsl_rng_state")]
    pub fn state<T>(&self) -> &T {
        unsafe { &(*(sys::gsl_rng_state(self.unwrap_shared()) as *const T)) }
    }

    /// This function returns a pointer to the state of generator r. You can use this information
    /// to access the state directly. For example, the following code will write the state of a
    /// generator to a stream,
    ///
    /// ```C
    /// void * state = gsl_rng_state (r);
    /// size_t n = gsl_rng_size (r);
    /// fwrite (state, n, 1, stream);
    /// ```
    // checker:ignore
    #[doc(alias = "gsl_rng_state")]
    pub fn state_mut<T>(&mut self) -> &mut T {
        unsafe { &mut (*(sys::gsl_rng_state(self.unwrap_shared()) as *mut T)) }
    }

    /// This function copies the random number generator src into the pre-existing generator dest, making dest into an exact copy of src. The two generators must be of the same type.
    #[doc(alias = "gsl_rng_memcpy")]
    pub fn copy(&self, other: &mut Rng) -> Value {
        Value::from(unsafe { sys::gsl_rng_memcpy(other.unwrap_unique(), self.unwrap_shared()) })
    }

    /// This function returns the size of the state of generator r. You can use this information to access the state directly. For example, the following code will write the state of a generator to a stream,
    ///
    /// ```C
    /// void * state = gsl_rng_state (r);
    /// size_t n = gsl_rng_size (r);
    /// fwrite (state, n, 1, stream);
    /// ```
    #[doc(alias = "gsl_rng_size")]
    pub fn size(&self) -> usize {
        unsafe { sys::gsl_rng_size(self.unwrap_shared()) }
    }

    /// Equivalent to DefaultRngSeed
    pub fn default_seed() -> usize {
        unsafe { sys::gsl_rng_default_seed as _ }
    }

    /// This function randomly shuffles the order of n objects, each of size size, stored in the array base[0..n-1]. The output of the random number generator r is used to
    /// produce the permutation. The algorithm generates all possible n! permutations with equal probability, assuming a perfect source of random numbers.
    ///
    /// The following code shows how to shuffle the numbers from 0 to 51,
    ///
    /// ```C
    /// int a[52];
    ///
    /// for (i = 0; i < 52; i++)
    ///   {
    ///     a[i] = i;
    ///   }
    ///
    /// gsl_ran_shuffle (r, a, 52, sizeof (int));
    /// ```
    #[doc(alias = "gsl_ran_shuffle")]
    pub fn shuffle<T>(&mut self, base: &mut [T]) {
        unsafe {
            sys::gsl_ran_shuffle(
                self.unwrap_unique(),
                base.as_mut_ptr() as *mut _,
                base.len() as _,
                ::std::mem::size_of::<T>() as _,
            )
        }
    }

    /// This function fills the array `dest[k]` with k objects taken randomly from the n elements of the array `src[0..n-1]`. The objects are each of size size.
    /// The output of the random number generator r is used to make the selection. The algorithm ensures all possible samples are equally likely, assuming a perfect source of randomness.
    ///
    /// The objects are sampled without replacement, thus each object can only appear once in `dest[k]`. It is required that k be less than or equal to n.
    /// The objects in dest will be in the same relative order as those in src. You will need to call gsl_ran_shuffle(r, dest, n, size) if you want to randomize the order.
    ///
    /// The following code shows how to select a random sample of three unique numbers from the set 0 to 99,
    ///
    /// ```C
    /// double a[3], b[100];
    ///
    /// for (i = 0; i < 100; i++)
    ///   {
    ///     b[i] = (double) i;
    ///   }
    ///
    /// gsl_ran_choose (r, a, 3, b, 100, sizeof (double));
    /// ```
    #[doc(alias = "gsl_ran_choose")]
    pub fn choose<T>(&mut self, src: &[T], dest: &mut [T]) -> Value {
        assert!(src.len() <= dest.len());
        Value::from(unsafe {
            sys::gsl_ran_choose(
                self.unwrap_unique(),
                dest.as_mut_ptr() as *mut _,
                dest.len() as _,
                src.as_ptr() as *mut _,
                src.len() as _,
                ::std::mem::size_of::<T>() as _,
            )
        })
    }

    /// This function is like gsl_ran_choose but samples k items from the original array of n items src with replacement, so the same object can appear more
    /// than once in the output sequence dest. There is no requirement that k be less than n in this case.
    #[doc(alias = "gsl_ran_sample")]
    pub fn sample<T>(&mut self, src: &[T], dest: &mut [T]) {
        assert!(src.len() <= dest.len());
        unsafe {
            sys::gsl_ran_sample(
                self.unwrap_unique(),
                dest.as_mut_ptr() as *mut _,
                dest.len() as _,
                src.as_ptr() as *mut _,
                src.len() as _,
                ::std::mem::size_of::<T>() as _,
            )
        }
    }

    /// This function computes a random sample n[] from the multinomial distribution formed by N trials from an underlying distribution `p[K]`. The distribution function for `n[]` is,
    ///
    /// ```text
    /// P(n_1, n_2, ..., n_K) =
    ///   (N!/(n_1! n_2! ... n_K!)) p_1^n_1 p_2^n_2 ... p_K^n_K
    /// ```
    ///
    /// where (n_1, n_2, ..., n_K) are nonnegative integers with
    /// `sum_{k=1}^K n_k = N, and (p_1, p_2, ..., p_K)` is a probability distribution with
    /// `\sum p_i = 1`. If the array `p[K]` is not normalized then its entries will be treated
    /// as weights and normalized appropriately. The arrays `n[]` and `p[]` must both be of length K.
    ///
    /// Random variates are generated using the conditional binomial method (see C.S. Davis, The computer generation of multinomial random variates, Comp. Stat. Data Anal. 16 (1993) 205–217 for details).
    #[doc(alias = "gsl_ran_multinomial")]
    pub fn multinomial(&mut self, N: u32, p: &[f64], n: &mut [u32]) {
        assert!(p.len() <= n.len());
        unsafe {
            sys::gsl_ran_multinomial(
                self.unwrap_unique(),
                p.len() as _,
                N,
                p.as_ptr(),
                n.as_mut_ptr(),
            )
        }
    }

    /// This function returns an array of K random variates from a Dirichlet distribution of order K-1. The distribution function is
    ///
    /// p(\theta_1, ..., \theta_K) d\theta_1 ... d\theta_K =
    ///
    ///   (1/Z) \prod_{i=1}^K \theta_i^{\alpha_i - 1} \delta(1 -\sum_{i=1}^K \theta_i) d\theta_1 ... d\theta_K
    ///
    /// for theta_i >= 0 and alpha_i > 0. The delta function ensures that \sum \theta_i = 1. The normalization factor Z is
    ///
    /// Z = {\prod_{i=1}^K \Gamma(\alpha_i)} / {\Gamma( \sum_{i=1}^K \alpha_i)}
    ///
    /// The random variates are generated by sampling K values from gamma distributions with parameters a=alpha_i, b=1, and renormalizing. See A.M. Law, W.D. Kelton, Simulation Modeling and Analysis (1991).
    #[doc(alias = "gsl_ran_dirichlet")]
    pub fn dirichlet(&mut self, alpha: &[f64], theta: &mut [f64]) {
        assert!(alpha.len() <= theta.len());
        unsafe {
            sys::gsl_ran_dirichlet(
                self.unwrap_unique(),
                alpha.len() as _,
                alpha.as_ptr(),
                theta.as_mut_ptr(),
            )
        }
    }

    /// This function returns either 0 or 1, the result of a Bernoulli trial with probability p. The probability distribution for a Bernoulli trial is,
    ///
    /// p(0) = 1 - p
    /// p(1) = p
    #[doc(alias = "gsl_ran_bernoulli")]
    pub fn bernoulli(&mut self, p: f64) -> u32 {
        unsafe { sys::gsl_ran_bernoulli(self.unwrap_unique(), p) }
    }

    /// This function returns a random variate from the beta distribution. The distribution function is,
    ///
    /// p(x) dx = {Gamma(a+b) over Gamma(a) Gamma(b)} x^{a-1} (1-x)^{b-1} dx
    ///
    /// for 0 <= x <= 1.
    #[doc(alias = "gsl_ran_beta")]
    pub fn beta(&mut self, a: f64, b: f64) -> f64 {
        unsafe { sys::gsl_ran_beta(self.unwrap_unique(), a, b) }
    }

    /// This function returns a random integer from the binomial distribution, the number of successes in n independent trials with probability p. The probability distribution for binomial variates is,
    ///
    /// p(k) = {n! \over k! (n-k)! } p^k (1-p)^{n-k}
    ///
    /// for 0 <= k <= n.
    #[doc(alias = "gsl_ran_binomial")]
    pub fn binomial(&mut self, p: f64, n: u32) -> u32 {
        unsafe { sys::gsl_ran_binomial(self.unwrap_unique(), p, n) }
    }

    /// This function generates a pair of correlated Gaussian variates, with mean zero, correlation coefficient rho and standard deviations sigma_x and sigma_y in the x and y directions.
    /// The probability distribution for bivariate Gaussian random variates is,
    ///
    /// p(x,y) dx dy = {1 \over 2 \pi \sigma_x \sigma_y \sqrt{1-\rho^2}} \exp (-(x^2/\sigma_x^2 + y^2/\sigma_y^2 - 2 \rho x y/(\sigma_x\sigma_y))/2(1-\rho^2)) dx dy
    ///
    /// for x,y in the range -\infty to +\infty. The correlation coefficient rho should lie between 1 and -1.
    #[doc(alias = "gsl_ran_bivariate_gaussian")]
    pub fn bivariate_gaussian(&mut self, sigma_x: f64, sigma_y: f64, rho: f64) -> (f64, f64) {
        let mut x = 0.;
        let mut y = 0.;

        unsafe {
            sys::gsl_ran_bivariate_gaussian(
                self.unwrap_unique(),
                sigma_x,
                sigma_y,
                rho,
                &mut x,
                &mut y,
            );
        }
        (x, y)
    }

    /// This function returns a random variate from the Cauchy distribution with scale parameter a. The probability distribution for Cauchy random variates is,
    ///
    /// p(x) dx = {1 \over a\pi (1 + (x/a)^2) } dx
    ///
    /// for x in the range -\infty to +\infty. The Cauchy distribution is also known as the Lorentz distribution.
    #[doc(alias = "gsl_ran_cauchy")]
    pub fn cauchy(&mut self, a: f64) -> f64 {
        unsafe { sys::gsl_ran_cauchy(self.unwrap_unique(), a) }
    }

    /// This function returns a random variate from the chi-squared distribution with nu degrees of freedom. The distribution function is,
    ///
    /// p(x) dx = {1 \over 2 Gamma(\nu/2) } (x/2)^{\nu/2 - 1} \exp(-x/2) dx
    ///
    /// for x >= 0.
    #[doc(alias = "gsl_ran_chisq")]
    pub fn chisq(&mut self, nu: f64) -> f64 {
        unsafe { sys::gsl_ran_chisq(self.unwrap_unique(), nu) }
    }

    /// This function returns a random variate from the exponential distribution with mean mu. The distribution is,
    ///
    /// p(x) dx = {1 \over \mu} \exp(-x/\mu) dx
    ///
    /// for x >= 0.
    #[doc(alias = "gsl_ran_exponential")]
    pub fn exponential(&mut self, mu: f64) -> f64 {
        unsafe { sys::gsl_ran_exponential(self.unwrap_unique(), mu) }
    }

    /// This function returns a random variate from the exponential power distribution with scale parameter a and exponent b. The distribution is,
    ///
    /// p(x) dx = {1 \over 2 a Gamma(1+1/b)} \exp(-|x/a|^b) dx
    ///
    /// for x >= 0. For b = 1 this reduces to the Laplace distribution. For b = 2 it has the same form as a Gaussian distribution, but with a = \sqrt{2} \sigma.
    #[doc(alias = "gsl_ran_exppow")]
    pub fn exppow(&mut self, a: f64, b: f64) -> f64 {
        unsafe { sys::gsl_ran_exppow(self.unwrap_unique(), a, b) }
    }

    /// This function returns a random variate from the F-distribution with degrees of freedom nu1 and nu2. The distribution function is,
    ///
    /// ```latex
    /// p(x) dx =
    /// { Gamma((\nu_1 + \nu_2)/2)
    ///
    ///         over Gamma(nu_1/2) Gamma(nu_2/2) }
    ///
    /// \nu_1^{\nu_1/2} \nu_2^{\nu_2/2}
    ///
    ///    x^{\nu_1/2 - 1} (\nu_2 + \nu_1 x)^{-\nu_1/2 -\nu_2/2}
    /// ```
    ///
    /// for x >= 0.
    #[doc(alias = "gsl_ran_fdist")]
    pub fn fdist(&mut self, nu1: f64, nu2: f64) -> f64 {
        unsafe { sys::gsl_ran_fdist(self.unwrap_unique(), nu1, nu2) }
    }

    /// This function returns a random variate from the flat (uniform) distribution from a to b. The distribution is,
    ///
    /// p(x) dx = {1 \over (b-a)} dx
    ///
    /// if a <= x < b and 0 otherwise.
    #[doc(alias = "gsl_ran_flat")]
    pub fn flat(&mut self, a: f64, b: f64) -> f64 {
        unsafe { sys::gsl_ran_flat(self.unwrap_unique(), a, b) }
    }

    /// This function returns a random variate from the gamma distribution. The distribution function is,
    ///
    /// p(x) dx = {1 over Gamma(a) b^a} x^{a-1} e^{-x/b} dx
    ///
    /// for x > 0.
    ///
    /// The gamma distribution with an integer parameter a is known as the Erlang distribution.
    ///
    /// The variates are computed using the Marsaglia-Tsang fast gamma method. This function for this method was previously called gsl_ran_gamma_mt and can still be accessed using this name.
    #[doc(alias = "gsl_ran_gamma")]
    pub fn gamma(&mut self, a: f64, b: f64) -> f64 {
        unsafe { sys::gsl_ran_gamma(self.unwrap_unique(), a, b) }
    }

    /// This function returns a gamma variate using the algorithms from Knuth (vol 2).
    #[doc(alias = "gsl_ran_gamma_knuth")]
    pub fn gamma_knuth(&mut self, a: f64, b: f64) -> f64 {
        unsafe { sys::gsl_ran_gamma_knuth(self.unwrap_unique(), a, b) }
    }

    /// This function returns a Gaussian random variate, with mean zero and standard deviation sigma.
    /// The probability distribution for Gaussian random variates is,
    ///
    /// p(x) dx = {1 \over \sqrt{2 \pi \sigma^2}} \exp (-x^2 / 2\sigma^2) dx
    /// for x in the range -\infty to +\infty. Use the transformation z = \mu + x on the numbers returned by gsl_ran_gaussian to obtain a Gaussian distribution with mean \mu.
    /// This function uses the Box-Muller algorithm which requires two calls to the random number generator r.
    #[doc(alias = "gsl_ran_gaussian")]
    pub fn gaussian(&mut self, sigma: f64) -> f64 {
        unsafe { sys::gsl_ran_gaussian(self.unwrap_unique(), sigma) }
    }

    #[doc(alias = "gsl_ran_gaussian_ziggurat")]
    pub fn gaussian_ziggurat(&mut self, sigma: f64) -> f64 {
        unsafe { sys::gsl_ran_gaussian_ziggurat(self.unwrap_unique(), sigma) }
    }

    /// This function computes a Gaussian random variate using the alternative Marsaglia-Tsang ziggurat and Kinderman-Monahan-Leva ratio methods.
    /// The Ziggurat algorithm is the fastest available algorithm in most cases.
    #[doc(alias = "gsl_ran_gaussian_ratio_method")]
    pub fn gaussian_ratio_method(&mut self, sigma: f64) -> f64 {
        unsafe { sys::gsl_ran_gaussian_ratio_method(self.unwrap_unique(), sigma) }
    }

    /// This function computes results for the unit Gaussian distribution.
    /// They are equivalent to the functions above with a standard deviation of one, sigma = 1.
    #[doc(alias = "gsl_ran_ugaussian")]
    pub fn ugaussian(&mut self) -> f64 {
        unsafe { sys::gsl_ran_ugaussian(self.unwrap_unique()) }
    }

    /// This function computes results for the unit Gaussian distribution.
    /// They are equivalent to the functions above with a standard deviation of one, sigma = 1.
    #[doc(alias = "gsl_ran_ugaussian_ratio_method")]
    pub fn ugaussian_ratio_method(&mut self) -> f64 {
        unsafe { sys::gsl_ran_ugaussian_ratio_method(self.unwrap_unique()) }
    }

    /// This function provides random variates from the upper tail of a Gaussian distribution with standard deviation sigma.
    /// The values returned are larger than the lower limit a, which must be positive. The method is based on Marsaglia’s famous rectangle-wedge-tail algorithm (Ann. Math. Stat. 32, 894–899 (1961)), with this aspect explained in Knuth, v2, 3rd ed, p139,586 (exercise 11).
    ///
    /// The probability distribution for Gaussian tail random variates is,
    ///
    /// p(x) dx = {1 \over N(a;\sigma) \sqrt{2 \pi \sigma^2}} \exp (- x^2/(2 \sigma^2)) dx
    ///
    /// for x > a where N(a;\sigma) is the normalization constant,
    ///
    /// N(a;\sigma) = (1/2) erfc(a / sqrt(2 sigma^2)).
    #[doc(alias = "gsl_ran_gaussian_tail")]
    pub fn gaussian_tail(&mut self, a: f64, sigma: f64) -> f64 {
        unsafe { sys::gsl_ran_gaussian_tail(self.unwrap_unique(), a, sigma) }
    }

    /// This function computes results for the tail of a unit Gaussian distribution. They are equivalent to the functions above with a standard deviation of one, sigma = 1.
    #[doc(alias = "gsl_ran_ugaussian_tail")]
    pub fn ugaussian_tail(&mut self, a: f64) -> f64 {
        unsafe { sys::gsl_ran_ugaussian_tail(self.unwrap_unique(), a) }
    }

    /// This function returns a random integer from the geometric distribution, the number of independent trials with probability p until the first success.
    /// The probability distribution for geometric variates is,
    ///
    /// p(k) =  p (1-p)^(k-1)
    ///
    /// for k >= 1. Note that the distribution begins with k=1 with this definition. There is another convention in which the exponent k-1 is replaced by k.
    #[doc(alias = "gsl_ran_geometric")]
    pub fn geometric(&mut self, p: f64) -> u32 {
        unsafe { sys::gsl_ran_geometric(self.unwrap_unique(), p) }
    }

    /// This function returns a random variate from the Type-1 Gumbel distribution. The Type-1 Gumbel distribution function is,
    ///
    /// p(x) dx = a b \exp(-(b \exp(-ax) + ax)) dx
    ///
    /// for -\infty < x < \infty.
    #[doc(alias = "gsl_ran_gumbel1")]
    pub fn gumbel1(&mut self, a: f64, b: f64) -> f64 {
        unsafe { sys::gsl_ran_gumbel1(self.unwrap_unique(), a, b) }
    }

    /// This function returns a random variate from the Type-2 Gumbel distribution. The Type-2 Gumbel distribution function is,
    ///
    /// p(x) dx = a b x^{-a-1} \exp(-b x^{-a}) dx
    ///
    /// for 0 < x < \infty.
    #[doc(alias = "gsl_ran_gumbel2")]
    pub fn gumbel2(&mut self, a: f64, b: f64) -> f64 {
        unsafe { sys::gsl_ran_gumbel2(self.unwrap_unique(), a, b) }
    }

    /// This function returns a random integer from the hypergeometric distribution. The probability distribution for hypergeometric random variates is,
    ///
    /// p(k) =  C(n_1, k) C(n_2, t - k) / C(n_1 + n_2, t)
    ///
    /// where C(a,b) = a!/(b!(a-b)!) and t <= n_1 + n_2. The domain of k is max(0,t-n_2), ..., min(t,n_1).
    ///
    /// If a population contains n_1 elements of “type 1” and n_2 elements of “type 2” then the hypergeometric distribution gives the probability of obtaining
    /// k elements of “type 1” in t samples from the population without replacement.
    #[doc(alias = "gsl_ran_hypergeometric")]
    pub fn hypergeometric(&mut self, n1: u32, n2: u32, t: u32) -> u32 {
        unsafe { sys::gsl_ran_hypergeometric(self.unwrap_unique(), n1, n2, t) }
    }

    /// This function returns a random variate from the Landau distribution. The probability distribution for Landau random variates is defined analytically by the complex integral,
    ///
    /// p(x) = (1/(2 \pi i)) \int_{c-i\infty}^{c+i\infty} ds exp(s log(s) + x s)
    ///
    /// For numerical purposes it is more convenient to use the following equivalent form of the integral,
    ///
    /// p(x) = (1/\pi) \int_0^\infty dt \exp(-t \log(t) - x t) \sin(\pi t).
    #[doc(alias = "gsl_ran_landau")]
    pub fn landau(&mut self) -> f64 {
        unsafe { sys::gsl_ran_landau(self.unwrap_unique()) }
    }

    /// This function returns a random variate from the Laplace distribution with width a. The distribution is,
    ///
    /// p(x) dx = {1 \over 2 a}  \exp(-|x/a|) dx
    ///
    /// for -\infty < x < \infty.
    #[doc(alias = "gsl_ran_laplace")]
    pub fn laplace(&mut self, a: f64) -> f64 {
        unsafe { sys::gsl_ran_laplace(self.unwrap_unique(), a) }
    }

    /// This function returns a random variate from the Levy symmetric stable distribution with scale c and exponent alpha. The symmetric stable probability distribution is defined by a Fourier transform,
    ///
    /// p(x) = {1 \over 2 \pi} \int_{-\infty}^{+\infty} dt \exp(-it x - |c t|^alpha)
    ///
    /// There is no explicit solution for the form of p(x) and the library does not define a corresponding pdf function. For \alpha = 1 the distribution reduces to the Cauchy distribution. For \alpha = 2 it is a Gaussian distribution with \sigma = \sqrt{2} c. For \alpha < 1 the tails of the distribution become extremely wide.
    ///
    /// The algorithm only works for 0 < alpha <= 2.
    #[doc(alias = "gsl_ran_levy")]
    pub fn levy(&mut self, c: f64, alpha: f64) -> f64 {
        unsafe { sys::gsl_ran_levy(self.unwrap_unique(), c, alpha) }
    }

    /// This function returns a random variate from the Levy skew stable distribution with scale c, exponent alpha and skewness parameter beta.
    /// The skewness parameter must lie in the range [-1,1]. The Levy skew stable probability distribution is defined by a Fourier transform,
    ///
    /// p(x) = {1 \over 2 \pi} \int_{-\infty}^{+\infty} dt \exp(-it x - |c t|^alpha (1-i beta sign(t) tan(pi alpha/2)))
    ///
    /// When \alpha = 1 the term \tan(\pi \alpha/2) is replaced by -(2/\pi)\log|t|. There is no explicit solution for the form of p(x) and the library does not define a corresponding pdf function.
    /// For \alpha = 2 the distribution reduces to a Gaussian distribution with \sigma = \sqrt{2} c and the skewness parameter has no effect. For \alpha < 1 the tails of the distribution become extremely wide.
    /// The symmetric distribution corresponds to \beta = 0.
    ///
    /// The algorithm only works for 0 < alpha <= 2.
    ///
    /// The Levy alpha-stable distributions have the property that if N alpha-stable variates are drawn from the distribution p(c, \alpha, \beta) then the sum Y = X_1 + X_2 + \dots + X_N will also be distributed as an alpha-stable variate, p(N^(1/\alpha) c, \alpha, \beta).
    #[doc(alias = "gsl_ran_levy_skew")]
    pub fn levy_skew(&mut self, c: f64, alpha: f64, beta: f64) -> f64 {
        unsafe { sys::gsl_ran_levy_skew(self.unwrap_unique(), c, alpha, beta) }
    }

    /// This function returns a random integer from the logarithmic distribution. The probability distribution for logarithmic random variates is,
    ///
    /// p(k) = {-1 \over \log(1-p)} {(p^k \over k)}
    ///
    /// for k >= 1.
    #[doc(alias = "gsl_ran_logarithmic")]
    pub fn logarithmic(&mut self, p: f64) -> u32 {
        unsafe { sys::gsl_ran_logarithmic(self.unwrap_unique(), p) }
    }

    /// This function returns a random variate from the logistic distribution. The distribution function is,
    ///
    /// p(x) dx = { \exp(-x/a) \over a (1 + \exp(-x/a))^2 } dx
    ///
    /// for -\infty < x < +\infty.
    #[doc(alias = "gsl_ran_logistic")]
    pub fn logistic(&mut self, a: f64) -> f64 {
        unsafe { sys::gsl_ran_logistic(self.unwrap_unique(), a) }
    }

    /// This function returns a random variate from the lognormal distribution. The distribution function is,
    ///
    /// p(x) dx = {1 \over x \sqrt{2 \pi \sigma^2} } \exp(-(\ln(x) - \zeta)^2/2 \sigma^2) dx
    ///
    /// for x > 0.
    #[doc(alias = "gsl_ran_lognormal")]
    pub fn lognormal(&mut self, zeta: f64, sigma: f64) -> f64 {
        unsafe { sys::gsl_ran_lognormal(self.unwrap_unique(), zeta, sigma) }
    }

    /// This function returns a random integer from the negative binomial distribution, the number of failures occurring before n successes in independent trials with
    /// probability p of success. The probability distribution for negative binomial variates is,
    ///
    /// p(k) = {\Gamma(n + k) \over \Gamma(k+1) \Gamma(n) } p^n (1-p)^k
    ///
    /// Note that n is not required to be an integer.
    #[doc(alias = "gsl_ran_negative_binomial")]
    pub fn negative_binomial(&mut self, p: f64, n: f64) -> u32 {
        unsafe { sys::gsl_ran_negative_binomial(self.unwrap_unique(), p, n) }
    }

    /// This function returns a random variate from the Pareto distribution of order a. The distribution function is,
    ///
    /// p(x) dx = (a/b) / (x/b)^{a+1} dx
    ///
    /// for x >= b.
    #[doc(alias = "gsl_ran_pareto")]
    pub fn pareto(&mut self, a: f64, b: f64) -> f64 {
        unsafe { sys::gsl_ran_pareto(self.unwrap_unique(), a, b) }
    }

    /// This function returns a random integer from the Pascal distribution. The Pascal distribution is simply a negative binomial distribution with an integer value of n.
    ///
    /// p(k) = {(n + k - 1)! \over k! (n - 1)! } p^n (1-p)^k
    ///
    /// for k >= 0
    #[doc(alias = "gsl_ran_pascal")]
    pub fn pascal(&mut self, p: f64, n: u32) -> u32 {
        unsafe { sys::gsl_ran_pascal(self.unwrap_unique(), p, n) }
    }

    /// This function returns a random integer from the Poisson distribution with mean mu. The probability distribution for Poisson variates is,
    ///
    /// p(k) = {\mu^k \over k!} \exp(-\mu)
    ///
    /// for k >= 0.
    #[doc(alias = "gsl_ran_poisson")]
    pub fn poisson(&mut self, mu: f64) -> u32 {
        unsafe { sys::gsl_ran_poisson(self.unwrap_unique(), mu) }
    }

    /// This function returns a random variate from the Rayleigh distribution with scale parameter sigma. The distribution is,
    ///
    /// p(x) dx = {x \over \sigma^2} \exp(- x^2/(2 \sigma^2)) dx
    ///
    /// for x > 0.
    #[doc(alias = "gsl_ran_rayleigh")]
    pub fn rayleigh(&mut self, sigma: f64) -> f64 {
        unsafe { sys::gsl_ran_rayleigh(self.unwrap_unique(), sigma) }
    }

    /// This function returns a random variate from the tail of the Rayleigh distribution with scale parameter sigma and a lower limit of a. The distribution is,
    ///
    /// p(x) dx = {x \over \sigma^2} \exp ((a^2 - x^2) /(2 \sigma^2)) dx
    ///
    /// for x > a.
    #[doc(alias = "gsl_ran_rayleigh_tail")]
    pub fn rayleigh_tail(&mut self, a: f64, sigma: f64) -> f64 {
        unsafe { sys::gsl_ran_rayleigh_tail(self.unwrap_unique(), a, sigma) }
    }

    /// This function returns a random direction vector v = (x,y) in two dimensions. The vector is normalized such that |v|^2 = x^2 + y^2 = 1.
    /// The obvious way to do this is to take a uniform random number between 0 and 2\pi and let x and y be the sine and cosine respectively.
    /// Two trig functions would have been expensive in the old days, but with modern hardware implementations, this is sometimes the fastest way to go.
    /// This is the case for the Pentium (but not the case for the Sun Sparcstation).
    /// One can avoid the trig evaluations by choosing x and y in the interior of a unit circle (choose them at random from the interior of the enclosing square,
    /// and then reject those that are outside the unit circle), and then dividing by \sqrt{x^2 + y^2}. A much cleverer approach, attributed to von Neumann
    /// (See Knuth, v2, 3rd ed, p140, exercise 23), requires neither trig nor a square root. In this approach, u and v are chosen at random from the interior of
    /// a unit circle, and then x=(u^2-v^2)/(u^2+v^2) and y=2uv/(u^2+v^2).
    ///
    /// Returns `(x, y)`.
    #[doc(alias = "gsl_ran_dir_2d")]
    pub fn dir_2d(&mut self) -> (f64, f64) {
        let mut x = 0.;
        let mut y = 0.;
        unsafe { sys::gsl_ran_dir_2d(self.unwrap_unique(), &mut x, &mut y) };
        (x, y)
    }

    /// This function returns a random direction vector v = (x,y) in two dimensions. The vector is normalized such that |v|^2 = x^2 + y^2 = 1.
    /// The obvious way to do this is to take a uniform random number between 0 and 2\pi and let x and y be the sine and cosine respectively.
    /// Two trig functions would have been expensive in the old days, but with modern hardware implementations, this is sometimes the fastest way to go.
    /// This is the case for the Pentium (but not the case for the Sun Sparcstation).
    /// One can avoid the trig evaluations by choosing x and y in the interior of a unit circle (choose them at random from the interior of the enclosing square,
    /// and then reject those that are outside the unit circle), and then dividing by \sqrt{x^2 + y^2}. A much cleverer approach, attributed to von Neumann
    /// (See Knuth, v2, 3rd ed, p140, exercise 23), requires neither trig nor a square root. In this approach, u and v are chosen at random from the interior of
    /// a unit circle, and then x=(u^2-v^2)/(u^2+v^2) and y=2uv/(u^2+v^2).
    ///
    /// Returns `(x, y)`.
    #[doc(alias = "gsl_ran_dir_2d_trig_method")]
    pub fn dir_2d_trig_method(&mut self) -> (f64, f64) {
        let mut x = 0.;
        let mut y = 0.;
        unsafe { sys::gsl_ran_dir_2d_trig_method(self.unwrap_unique(), &mut x, &mut y) };
        (x, y)
    }

    /// This function returns a random direction vector v = (x,y,z) in three dimensions. The vector is normalized such that |v|^2 = x^2 + y^2 + z^2 = 1.
    /// The method employed is due to Robert E. Knop (CACM 13, 326 (1970)), and explained in Knuth, v2, 3rd ed, p136. It uses the surprising fact that the
    /// distribution projected along any axis is actually uniform (this is only true for 3 dimensions).
    ///
    /// Returns `(x, y, z)`.
    #[doc(alias = "gsl_ran_dir_3d")]
    pub fn dir_3d(&mut self) -> (f64, f64, f64) {
        let mut x = 0.;
        let mut y = 0.;
        let mut z = 0.;
        unsafe { sys::gsl_ran_dir_3d(self.unwrap_unique(), &mut x, &mut y, &mut z) };
        (x, y, z)
    }

    /// This function returns a random direction vector v = (x_1,x_2,...,x_n) in n dimensions. The vector is normalized such that |v|^2 = x_1^2 + x_2^2 + ... + x_n^2 = 1.
    /// The method uses the fact that a multivariate Gaussian distribution is spherically symmetric. Each component is generated to have a Gaussian distribution, and then
    /// the components are normalized. The method is described by Knuth, v2, 3rd ed, p135–136, and attributed to G. W. Brown, Modern Mathematics for the Engineer (1956).
    #[doc(alias = "gsl_ran_dir_nd")]
    pub fn dir_nd(&mut self, x: &mut [f64]) {
        unsafe { sys::gsl_ran_dir_nd(self.unwrap_unique(), x.len() as _, x.as_mut_ptr()) }
    }

    /// This function returns a random variate from the t-distribution. The distribution function is,
    ///
    /// p(x) dx = {Gamma((\nu + 1)/2) \over \sqrt{\pi \nu} Gamma(\nu/2)}
    ///
    ///    (1 + x^2/\nu)^{-(\nu + 1)/2} dx
    ///
    /// for -\infty < x < +\infty.
    #[doc(alias = "gsl_ran_tdist")]
    pub fn tdist(&mut self, nu: f64) -> f64 {
        unsafe { sys::gsl_ran_tdist(self.unwrap_unique(), nu) }
    }

    /// This function returns a random variate from the Weibull distribution. The distribution function is,
    ///
    /// p(x) dx = {b \over a^b} x^{b-1}  \exp(-(x/a)^b) dx
    ///
    /// for x >= 0.
    #[doc(alias = "gsl_ran_weibull")]
    pub fn weibull(&mut self, a: f64, b: f64) -> f64 {
        unsafe { sys::gsl_ran_weibull(self.unwrap_unique(), a, b) }
    }
}

impl Clone for Rng {
    /// This function returns a pointer to a newly created generator which is an exact copy of the generator r.
    #[doc(alias = "gsl_rng_clone")]
    fn clone(&self) -> Rng {
        unsafe { FFI::wrap(sys::gsl_rng_clone(self.unwrap_shared())) }
    }
}

ffi_wrapper!(RngType, *const sys::gsl_rng_type);

impl RngType {
    /// wrapper for name element
    pub fn name(&self) -> String {
        let ptr = self.unwrap_shared();
        if ptr.is_null() {
            String::new()
        } else {
            unsafe {
                String::from_utf8_lossy(::std::ffi::CStr::from_ptr((*ptr).name).to_bytes())
                    .to_string()
            }
        }
    }

    #[doc(alias = "gsl_rng_default")]
    pub fn default() -> Self {
        ffi_wrap!(gsl_rng_default)
    }

    /// wrapper for max element
    pub fn max(&self) -> usize {
        let ptr = self.unwrap_shared();
        if ptr.is_null() {
            0
        } else {
            unsafe { (*ptr).max as _ }
        }
    }

    /// wrapper for min element
    pub fn min(&self) -> usize {
        let ptr = self.unwrap_shared();
        if ptr.is_null() {
            0
        } else {
            unsafe { (*ptr).min as _ }
        }
    }

    /// wrapper for size element
    pub fn size(&self) -> usize {
        let ptr = self.unwrap_shared();
        if ptr.is_null() {
            0
        } else {
            unsafe { (*ptr).size }
        }
    }

    /// This function returns a pointer to an array of all the available generator types, terminated by a null pointer.
    /// The function should be called once at the start of the program, if needed. The following code fragment shows how to iterate over the array of generator types to print the names of the available algorithms,
    ///
    /// ```Rust
    /// let t = RngType::types_setup ();
    ///
    /// println!("Available generators:");
    /// for tmp in t.iter() {
    ///     println!("{}", tmp.name);
    /// }
    /// ```
    #[doc(alias = "gsl_rng_types_setup")]
    pub fn types_setup() -> Vec<RngType> {
        let ptr = unsafe { sys::gsl_rng_types_setup() };
        let mut ret = Vec::new();

        if !ptr.is_null() {
            unsafe {
                let mut it = 0;
                loop {
                    let tmp = ptr.offset(it);

                    if (*tmp).is_null() {
                        break;
                    }
                    ret.push(RngType::wrap(*tmp as *mut sys::gsl_rng_type));
                    it += 1;
                }
            }
        }
        ret
    }

    /// This function reads the environment variables GSL_RNG_TYPE and GSL_RNG_SEED and uses their values to set the corresponding library variables gsl_rng_default and gsl_rng_default_seed. These global variables are defined as follows,
    ///
    /// ```C
    /// extern const gsl_rng_type *gsl_rng_default
    /// extern unsigned long int gsl_rng_default_seed
    /// ```
    ///
    /// The environment variable GSL_RNG_TYPE should be the name of a generator, such as taus or mt19937. The environment variable GSL_RNG_SEED should contain the desired seed value.
    /// It is converted to an unsigned long int using the C library function strtoul.
    ///
    /// If you don’t specify a generator for GSL_RNG_TYPE then gsl_rng_mt19937 is used as the default. The initial value of gsl_rng_default_seed is zero.
    /// See rng example in examples folder for more details.
    #[doc(alias = "gsl_rng_env_setup")]
    pub fn env_setup() -> Option<RngType> {
        let tmp = unsafe { sys::gsl_rng_env_setup() };

        if tmp.is_null() {
            None
        } else {
            Some(RngType::wrap(tmp as *mut sys::gsl_rng_type))
        }
    }
}

/// The functions described above make no reference to the actual algorithm used. This is deliberate so that you can switch algorithms without having
/// to change any of your application source code. The library provides a large number of generators of different types, including simulation quality
/// generators, generators provided for compatibility with other libraries and historical generators from the past.
///
/// The following generators are recommended for use in simulation. They have extremely long periods, low correlation and pass most statistical tests.
/// For the most reliable source of uncorrelated numbers, the second-generation RANLUX generators have the strongest proof of randomness.
pub mod algorithms {
    use types::RngType;

    /// The MT19937 generator of Makoto Matsumoto and Takuji Nishimura is a variant of the twisted generalized feedback shift-register algorithm, and
    /// is known as the “Mersenne Twister” generator. It has a Mersenne prime period of 2^19937 - 1 (about 10^6000) and is equi-distributed in 623 dimensions.
    /// It has passed the DIEHARD statistical tests. It uses 624 words of state per generator and is comparable in speed to the other generators. The original
    /// generator used a default seed of 4357 and choosing s equal to zero in gsl_rng_set reproduces this. Later versions switched to 5489 as the default seed,
    /// you can choose this explicitly via gsl_rng_set instead if you require it.
    ///
    /// For more information see,
    ///
    /// Makoto Matsumoto and Takuji Nishimura, “Mersenne Twister: A 623-dimensionally equidistributed uniform pseudorandom number generator”. ACM Transactions
    /// on Modeling and Computer Simulation, Vol. 8, No. 1 (Jan. 1998), Pages 3–30
    ///
    /// The generator gsl_rng_mt19937 uses the second revision of the seeding procedure published by the two authors above in 2002. The original seeding
    /// procedures could cause spurious artifacts for some seed values. They are still available through the alternative generators gsl_rng_mt19937_1999 and
    /// gsl_rng_mt19937_1998.
    #[doc(alias = "gsl_rng_mt19937")]
    pub fn mt19937() -> RngType {
        ffi_wrap!(gsl_rng_mt19937)
    }

    /// The generator ranlxs0 is a second-generation version of the RANLUX algorithm of Lüscher, which produces “luxury random numbers”. This generator
    /// provides single precision output (24 bits) at three luxury levels ranlxs0, ranlxs1 and ranlxs2, in increasing order of strength. It uses double-precision
    /// floating point arithmetic internally and can be significantly faster than the integer version of ranlux, particularly on 64-bit architectures. The period
    /// of the generator is about 10^171. The algorithm has mathematically proven properties and can provide truly decorrelated numbers at a known level of
    /// randomness. The higher luxury levels provide increased decorrelation between samples as an additional safety margin.
    ///
    /// Note that the range of allowed seeds for this generator is [0,2^31-1]. Higher seed values are wrapped modulo 2^31.
    #[doc(alias = "gsl_rng_ranlxs0")]
    pub fn ranlxs0() -> RngType {
        ffi_wrap!(gsl_rng_ranlxs0)
    }

    /// The generator ranlxs0 is a second-generation version of the RANLUX algorithm of Lüscher, which produces “luxury random numbers”. This generator
    /// provides single precision output (24 bits) at three luxury levels ranlxs0, ranlxs1 and ranlxs2, in increasing order of strength. It uses double-precision
    /// floating point arithmetic internally and can be significantly faster than the integer version of ranlux, particularly on 64-bit architectures. The period
    /// of the generator is about 10^171. The algorithm has mathematically proven properties and can provide truly decorrelated numbers at a known level of
    /// randomness. The higher luxury levels provide increased decorrelation between samples as an additional safety margin.
    ///
    /// Note that the range of allowed seeds for this generator is [0,2^31-1]. Higher seed values are wrapped modulo 2^31.
    #[doc(alias = "gsl_rng_ranlxs1")]
    pub fn ranlxs1() -> RngType {
        ffi_wrap!(gsl_rng_ranlxs1)
    }

    /// The generator ranlxs0 is a second-generation version of the RANLUX algorithm of Lüscher, which produces “luxury random numbers”. This generator
    /// provides single precision output (24 bits) at three luxury levels ranlxs0, ranlxs1 and ranlxs2, in increasing order of strength. It uses double-precision
    /// floating point arithmetic internally and can be significantly faster than the integer version of ranlux, particularly on 64-bit architectures. The period
    /// of the generator is about 10^171. The algorithm has mathematically proven properties and can provide truly decorrelated numbers at a known level of
    /// randomness. The higher luxury levels provide increased decorrelation between samples as an additional safety margin.
    ///
    /// Note that the range of allowed seeds for this generator is [0,2^31-1]. Higher seed values are wrapped modulo 2^31.
    #[doc(alias = "gsl_rng_ranlxs2")]
    pub fn ranlxs2() -> RngType {
        ffi_wrap!(gsl_rng_ranlxs2)
    }

    /// This generator produces double precision output (48 bits) from the RANLXS generator. The library provides two luxury levels ranlxd1 and ranlxd2,
    /// in increasing order of strength.
    #[doc(alias = "gsl_rng_ranlxd1")]
    pub fn ranlxd1() -> RngType {
        ffi_wrap!(gsl_rng_ranlxd1)
    }

    /// This generator produces double precision output (48 bits) from the RANLXS generator. The library provides two luxury levels ranlxd1 and ranlxd2,
    /// in increasing order of strength.
    #[doc(alias = "gsl_rng_ranlxd2")]
    pub fn ranlxd2() -> RngType {
        ffi_wrap!(gsl_rng_ranlxd2)
    }

    /// The ranlux generator is an implementation of the original algorithm developed by Lüscher. It uses a lagged-fibonacci-with-skipping algorithm to
    /// produce “luxury random numbers”. It is a 24-bit generator, originally designed for single-precision IEEE floating point numbers. This
    /// implementation is based on integer arithmetic, while the second-generation versions RANLXS and RANLXD described above provide floating-point
    /// implementations which will be faster on many platforms. The period of the generator is about 10^171. The algorithm has mathematically proven
    /// properties and it can provide truly decorrelated numbers at a known level of randomness. The default level of decorrelation recommended by
    /// Lüscher is provided by gsl_rng_ranlux, while gsl_rng_ranlux389 gives the highest level of randomness, with all 24 bits decorrelated. Both
    /// types of generator use 24 words of state per generator.
    ///
    /// For more information see,
    ///
    /// M. Lüscher, “A portable high-quality random number generator for lattice field theory calculations”, Computer Physics Communications, 79 (1994) 100–110.
    /// F. James, “RANLUX: A Fortran implementation of the high-quality pseudo-random number generator of Lüscher”, Computer Physics Communications, 79 (1994) 111–114
    #[doc(alias = "gsl_rng_ranlux")]
    pub fn ranlux() -> RngType {
        ffi_wrap!(gsl_rng_ranlux)
    }

    /// The ranlux generator is an implementation of the original algorithm developed by Lüscher. It uses a lagged-fibonacci-with-skipping algorithm to
    /// produce “luxury random numbers”. It is a 24-bit generator, originally designed for single-precision IEEE floating point numbers. This
    /// implementation is based on integer arithmetic, while the second-generation versions RANLXS and RANLXD described above provide floating-point
    /// implementations which will be faster on many platforms. The period of the generator is about 10^171. The algorithm has mathematically proven
    /// properties and it can provide truly decorrelated numbers at a known level of randomness. The default level of decorrelation recommended by
    /// Lüscher is provided by gsl_rng_ranlux, while gsl_rng_ranlux389 gives the highest level of randomness, with all 24 bits decorrelated. Both
    /// types of generator use 24 words of state per generator.
    ///
    /// For more information see,
    ///
    /// M. Lüscher, “A portable high-quality random number generator for lattice field theory calculations”, Computer Physics Communications, 79 (1994) 100–110.
    /// F. James, “RANLUX: A Fortran implementation of the high-quality pseudo-random number generator of Lüscher”, Computer Physics Communications, 79 (1994) 111–114
    #[doc(alias = "gsl_rng_ranlux389")]
    pub fn ranlux389() -> RngType {
        ffi_wrap!(gsl_rng_ranlux389)
    }

    /// This is a combined multiple recursive generator by L’Ecuyer. Its sequence is,
    ///
    /// z_n = (x_n - y_n) mod m_1
    ///
    /// where the two underlying generators x_n and y_n are,
    ///
    /// x_n = (a_1 x_{n-1} + a_2 x_{n-2} + a_3 x_{n-3}) mod m_1
    /// y_n = (b_1 y_{n-1} + b_2 y_{n-2} + b_3 y_{n-3}) mod m_2
    ///
    /// with coefficients a_1 = 0, a_2 = 63308, a_3 = -183326, b_1 = 86098, b_2 = 0, b_3 = -539608, and moduli m_1 = 2^31 - 1 = 2147483647 and m_2 = 2145483479.
    ///
    //// The period of this generator is lcm(m_1^3-1, m_2^3-1), which is approximately 2^185 (about 10^56). It uses 6 words of state per generator. For more information see,
    ///
    /// P. L’Ecuyer, “Combined Multiple Recursive Random Number Generators”, Operations Research, 44, 5 (1996), 816–822.
    #[doc(alias = "gsl_rng_cmrg")]
    pub fn cmrg() -> RngType {
        ffi_wrap!(gsl_rng_cmrg)
    }

    /// This is a fifth-order multiple recursive generator by L’Ecuyer, Blouin and Coutre. Its sequence is,
    ///
    /// x_n = (a_1 x_{n-1} + a_5 x_{n-5}) mod m
    ///
    /// with a_1 = 107374182, a_2 = a_3 = a_4 = 0, a_5 = 104480 and m = 2^31 - 1.
    ///
    /// The period of this generator is about 10^46. It uses 5 words of state per generator. More information can be found in the following paper,
    ///
    /// P. L’Ecuyer, F. Blouin, and R. Coutre, “A search for good multiple recursive random number generators”, ACM Transactions on Modeling and Computer Simulation 3, 87–98 (1993).
    #[doc(alias = "gsl_rng_mrg")]
    pub fn mrg() -> RngType {
        ffi_wrap!(gsl_rng_mrg)
    }

    /// This is a maximally equidistributed combined Tausworthe generator by L’Ecuyer. The sequence is,
    ///
    /// x_n = (s1_n ^^ s2_n ^^ s3_n)
    ///
    /// where,
    ///
    /// s1_{n+1} = (((s1_n&4294967294)<<12)^^(((s1_n<<13)^^s1_n)>>19))
    /// s2_{n+1} = (((s2_n&4294967288)<< 4)^^(((s2_n<< 2)^^s2_n)>>25))
    /// s3_{n+1} = (((s3_n&4294967280)<<17)^^(((s3_n<< 3)^^s3_n)>>11))
    ///
    /// computed modulo 2^32. In the formulas above ^^ denotes “exclusive-or”. Note that the algorithm relies on the properties of 32-bit
    /// unsigned integers and has been implemented using a bitmask of 0xFFFFFFFF to make it work on 64 bit machines.
    ///
    /// The period of this generator is 2^88 (about 10^26). It uses 3 words of state per generator. For more information see,
    ///
    /// P. L’Ecuyer, “Maximally Equidistributed Combined Tausworthe Generators”, Mathematics of Computation, 65, 213 (1996), 203–213.
    ///
    /// The generator gsl_rng_taus2 uses the same algorithm as gsl_rng_taus but with an improved seeding procedure described in the paper,
    ///
    /// P. L’Ecuyer, “Tables of Maximally Equidistributed Combined LFSR Generators”, Mathematics of Computation, 68, 225 (1999), 261–269
    ///
    /// The generator gsl_rng_taus2 should now be used in preference to gsl_rng_taus.
    #[doc(alias = "gsl_rng_taus")]
    pub fn taus() -> RngType {
        ffi_wrap!(gsl_rng_taus)
    }

    /// This is a maximally equidistributed combined Tausworthe generator by L’Ecuyer. The sequence is,
    ///
    /// x_n = (s1_n ^^ s2_n ^^ s3_n)
    ///
    /// where,
    ///
    /// s1_{n+1} = (((s1_n&4294967294)<<12)^^(((s1_n<<13)^^s1_n)>>19))
    /// s2_{n+1} = (((s2_n&4294967288)<< 4)^^(((s2_n<< 2)^^s2_n)>>25))
    /// s3_{n+1} = (((s3_n&4294967280)<<17)^^(((s3_n<< 3)^^s3_n)>>11))
    ///
    /// computed modulo 2^32. In the formulas above ^^ denotes “exclusive-or”. Note that the algorithm relies on the properties of 32-bit
    /// unsigned integers and has been implemented using a bitmask of 0xFFFFFFFF to make it work on 64 bit machines.
    ///
    /// The period of this generator is 2^88 (about 10^26). It uses 3 words of state per generator. For more information see,
    ///
    /// P. L’Ecuyer, “Maximally Equidistributed Combined Tausworthe Generators”, Mathematics of Computation, 65, 213 (1996), 203–213.
    ///
    /// The generator gsl_rng_taus2 uses the same algorithm as gsl_rng_taus but with an improved seeding procedure described in the paper,
    ///
    /// P. L’Ecuyer, “Tables of Maximally Equidistributed Combined LFSR Generators”, Mathematics of Computation, 68, 225 (1999), 261–269
    ///
    /// The generator gsl_rng_taus2 should now be used in preference to gsl_rng_taus.
    #[doc(alias = "gsl_rng_taus2")]
    pub fn taus2() -> RngType {
        ffi_wrap!(gsl_rng_taus2)
    }

    /// The gfsr4 generator is like a lagged-fibonacci generator, and produces each number as an xor’d sum of four previous values.
    ///
    /// r_n = r_{n-A} ^^ r_{n-B} ^^ r_{n-C} ^^ r_{n-D}
    ///
    /// Ziff (ref below) notes that “it is now widely known” that two-tap registers (such as R250, which is described below) have serious
    /// flaws, the most obvious one being the three-point correlation that comes from the definition of the generator. Nice mathematical
    /// properties can be derived for GFSR’s, and numerics bears out the claim that 4-tap GFSR’s with appropriately chosen offsets are as
    /// random as can be measured, using the author’s test.
    ///
    /// This implementation uses the values suggested the example on p392 of Ziff’s article: A=471, B=1586, C=6988, D=9689.
    ///
    /// If the offsets are appropriately chosen (such as the one ones in this implementation), then the sequence is said to be maximal;
    /// that means that the period is 2^D - 1, where D is the longest lag. (It is one less than 2^D because it is not permitted to have all
    /// zeros in the ra[] array.) For this implementation with D=9689 that works out to about 10^2917.
    ///
    /// Note that the implementation of this generator using a 32-bit integer amounts to 32 parallel implementations of one-bit generators.
    /// One consequence of this is that the period of this 32-bit generator is the same as for the one-bit generator. Moreover, this
    /// independence means that all 32-bit patterns are equally likely, and in particular that 0 is an allowed random value. (We are grateful
    /// to Heiko Bauke for clarifying for us these properties of GFSR random number generators.)
    ///
    /// For more information see,
    ///
    /// Robert M. Ziff, “Four-tap shift-register-sequence random-number generators”, Computers in Physics, 12(4), Jul/Aug 1998, pp 385–392.
    #[doc(alias = "gsl_rng_gfsr4")]
    pub fn gfsr4() -> RngType {
        ffi_wrap!(gsl_rng_gfsr4)
    }
}

/// The standard Unix random number generators rand, random and rand48 are provided as part of GSL. Although these generators are widely
/// available individually often they aren’t all available on the same platform. This makes it difficult to write portable code using them
/// and so we have included the complete set of Unix generators in GSL for convenience. Note that these generators don’t produce high-quality
/// randomness and aren’t suitable for work requiring accurate statistics. However, if you won’t be measuring statistical quantities and just
/// want to introduce some variation into your program then these generators are quite acceptable.
pub mod unix {
    use types::RngType;

    /// This is the BSD rand generator. Its sequence is
    ///
    /// x_{n+1} = (a x_n + c) mod m
    ///
    /// with a = 1103515245, c = 12345 and m = 2^31. The seed specifies the initial value, x_1. The period of this generator is 2^31, and it
    /// uses 1 word of storage per generator.
    #[doc(alias = "gsl_rng_rand")]
    pub fn rand() -> RngType {
        ffi_wrap!(gsl_rng_rand)
    }

    /// These generators implement the random family of functions, a set of linear feedback shift register generators originally used in BSD
    /// Unix. There are several versions of random in use today: the original BSD version (e.g. on SunOS4), a libc5 version (found on older
    /// GNU/Linux systems) and a glibc2 version. Each version uses a different seeding procedure, and thus produces different sequences.
    ///
    /// The original BSD routines accepted a variable length buffer for the generator state, with longer buffers providing higher-quality
    /// randomness. The random function implemented algorithms for buffer lengths of 8, 32, 64, 128 and 256 bytes, and the algorithm with the
    /// largest length that would fit into the user-supplied buffer was used. To support these algorithms additional generators are available
    /// with the following names,
    ///
    /// * gsl_rng_random8_bsd
    /// * gsl_rng_random32_bsd
    /// * gsl_rng_random64_bsd
    /// * gsl_rng_random128_bsd
    /// * gsl_rng_random256_bsd
    ///
    /// where the numeric suffix indicates the buffer length. The original BSD random function used a 128-byte default buffer and so
    /// gsl_rng_random_bsd has been made equivalent to gsl_rng_random128_bsd. Corresponding versions of the libc5 and glibc2 generators are
    /// also available, with the names gsl_rng_random8_libc5, gsl_rng_random8_glibc2, etc.
    #[doc(alias = "gsl_rng_random_bsd")]
    pub fn random_bsd() -> RngType {
        ffi_wrap!(gsl_rng_random_bsd)
    }

    /// These generators implement the random family of functions, a set of linear feedback shift register generators originally used in BSD
    /// Unix. There are several versions of random in use today: the original BSD version (e.g. on SunOS4), a libc5 version (found on older
    /// GNU/Linux systems) and a glibc2 version. Each version uses a different seeding procedure, and thus produces different sequences.
    ///
    /// The original BSD routines accepted a variable length buffer for the generator state, with longer buffers providing higher-quality
    /// randomness. The random function implemented algorithms for buffer lengths of 8, 32, 64, 128 and 256 bytes, and the algorithm with the
    /// largest length that would fit into the user-supplied buffer was used. To support these algorithms additional generators are available
    /// with the following names,
    ///
    /// * gsl_rng_random8_bsd
    /// * gsl_rng_random32_bsd
    /// * gsl_rng_random64_bsd
    /// * gsl_rng_random128_bsd
    /// * gsl_rng_random256_bsd
    ///
    /// where the numeric suffix indicates the buffer length. The original BSD random function used a 128-byte default buffer and so
    /// gsl_rng_random_bsd has been made equivalent to gsl_rng_random128_bsd. Corresponding versions of the libc5 and glibc2 generators are
    /// also available, with the names gsl_rng_random8_libc5, gsl_rng_random8_glibc2, etc.
    #[doc(alias = "gsl_rng_random_libc5")]
    pub fn random_libc5() -> RngType {
        ffi_wrap!(gsl_rng_random_libc5)
    }

    /// These generators implement the random family of functions, a set of linear feedback shift register generators originally used in BSD
    /// Unix. There are several versions of random in use today: the original BSD version (e.g. on SunOS4), a libc5 version (found on older
    /// GNU/Linux systems) and a glibc2 version. Each version uses a different seeding procedure, and thus produces different sequences.
    ///
    /// The original BSD routines accepted a variable length buffer for the generator state, with longer buffers providing higher-quality
    /// randomness. The random function implemented algorithms for buffer lengths of 8, 32, 64, 128 and 256 bytes, and the algorithm with the
    /// largest length that would fit into the user-supplied buffer was used. To support these algorithms additional generators are available
    /// with the following names,
    ///
    /// * gsl_rng_random8_bsd
    /// * gsl_rng_random32_bsd
    /// * gsl_rng_random64_bsd
    /// * gsl_rng_random128_bsd
    /// * gsl_rng_random256_bsd
    ///
    /// where the numeric suffix indicates the buffer length. The original BSD random function used a 128-byte default buffer and so
    /// gsl_rng_random_bsd has been made equivalent to gsl_rng_random128_bsd. Corresponding versions of the libc5 and glibc2 generators are
    /// also available, with the names gsl_rng_random8_libc5, gsl_rng_random8_glibc2, etc.
    #[doc(alias = "gsl_rng_random_glibc2")]
    pub fn random_glibc2() -> RngType {
        ffi_wrap!(gsl_rng_random_glibc2)
    }

    /// This is the Unix rand48 generator. Its sequence is
    ///
    /// x_{n+1} = (a x_n + c) mod m
    /// defined on 48-bit unsigned integers with a = 25214903917, c = 11 and m = 2^48. The seed specifies the upper 32 bits of the initial
    /// value, x_1, with the lower 16 bits set to 0x330E. The function gsl_rng_get returns the upper 32 bits from each term of the sequence.
    /// This does not have a direct parallel in the original rand48 functions, but forcing the result to type long int reproduces the output
    /// of mrand48. The function gsl_rng_uniform uses the full 48 bits of internal state to return the double precision number x_n/m, which
    /// is equivalent to the function drand48. Note that some versions of the GNU C Library contained a bug in mrand48 function which caused
    /// it to produce different results (only the lower 16-bits of the return value were set).
    #[doc(alias = "gsl_rng_rand48")]
    pub fn rand48() -> RngType {
        ffi_wrap!(gsl_rng_rand48)
    }
}

/// ## Other random number generators
///
/// The generators in this section are provided for compatibility with existing libraries. If you are converting an existing program to use GSL then
/// you can select these generators to check your new implementation against the original one, using the same random number generator. After verifying
/// that your new program reproduces the original results you can then switch to a higher-quality generator.
///
/// Note that most of the generators in this section are based on single linear congruence relations, which are the least sophisticated type of generator.
/// In particular, linear congruences have poor properties when used with a non-prime modulus, as several of these routines do (e.g. with a power of two modulus,
/// 2^31 or 2^32). This leads to periodicity in the least significant bits of each number, with only the higher bits having any randomness.
/// Thus if you want to produce a random bitstream it is best to avoid using the least significant bits.
pub mod other {
    use types::RngType;

    /// This is the CRAY random number generator RANF. Its sequence is
    ///
    /// x_{n+1} = (a x_n) mod m
    /// defined on 48-bit unsigned integers with a = 44485709377909 and m = 2^48. The seed specifies the lower 32 bits of the initial value, x_1, with the lowest bit set to prevent the seed taking an even value. The upper 16 bits of x_1 are set to 0. A consequence of this procedure is that the pairs of seeds 2 and 3, 4 and 5, etc. produce the same sequences.
    ///
    /// The generator compatible with the CRAY MATHLIB routine RANF. It produces double precision floating point numbers which should be identical to those from the original RANF.
    ///
    /// There is a subtlety in the implementation of the seeding. The initial state is reversed through one step, by multiplying by the modular inverse of a mod m. This is done for compatibility with the original CRAY implementation.
    ///
    /// Note that you can only seed the generator with integers up to 2^32, while the original CRAY implementation uses non-portable wide integers which can cover all 2^48 states of the generator.
    ///
    /// The function gsl_rng_get returns the upper 32 bits from each term of the sequence. The function gsl_rng_uniform uses the full 48 bits to return the double precision number x_n/m.
    ///
    /// The period of this generator is 2^46.
    #[doc(alias = "gsl_rng_ranf")]
    pub fn ranf() -> RngType {
        ffi_wrap!(gsl_rng_ranf)
    }

    /// This is the RANMAR lagged-fibonacci generator of Marsaglia, Zaman and Tsang. It is a 24-bit generator, originally designed for single-precision IEEE floating point numbers.
    /// It was included in the CERNLIB high-energy physics library.
    #[doc(alias = "gsl_rng_ranmar")]
    pub fn ranmar() -> RngType {
        ffi_wrap!(gsl_rng_ranmar)
    }

    /// This is the shift-register generator of Kirkpatrick and Stoll. The sequence is based on the recurrence
    ///
    /// x_n = x_{n-103} ^^ x_{n-250}
    /// where ^^ denotes “exclusive-or”, defined on 32-bit words. The period of this generator is about 2^250 and it uses 250 words of state per generator.
    ///
    /// For more information see,
    ///
    /// S. Kirkpatrick and E. Stoll, “A very fast shift-register sequence random number generator”, Journal of Computational Physics, 40, 517–526 (1981)
    #[doc(alias = "gsl_rng_r250")]
    pub fn r250() -> RngType {
        ffi_wrap!(gsl_rng_r250)
    }

    /// This is an earlier version of the twisted generalized feedback shift-register generator, and has been superseded by the development of MT19937. However, it is
    /// still an acceptable generator in its own right. It has a period of 2^800 and uses 33 words of storage per generator.
    ///
    /// For more information see,
    ///
    /// Makoto Matsumoto and Yoshiharu Kurita, “Twisted GFSR Generators II”, ACM Transactions on Modelling and Computer Simulation, Vol. 4, No. 3, 1994, pages 254–266.
    #[doc(alias = "gsl_rng_tt800")]
    pub fn tt800() -> RngType {
        ffi_wrap!(gsl_rng_tt800)
    }

    /// This is the VAX generator MTH$RANDOM. Its sequence is,
    ///
    /// x_{n+1} = (a x_n + c) mod m
    ///
    /// with a = 69069, c = 1 and m = 2^32. The seed specifies the initial value, x_1. The period of this generator is 2^32 and it uses 1 word of storage per generator.
    #[doc(alias = "gsl_rng_vax")]
    pub fn vax() -> RngType {
        ffi_wrap!(gsl_rng_vax)
    }

    /// This is the random number generator from the INMOS Transputer Development system. Its sequence is,
    ///
    /// x_{n+1} = (a x_n) mod m
    ///
    /// with a = 1664525 and m = 2^32. The seed specifies the initial value, x_1.
    #[doc(alias = "gsl_rng_transputer")]
    pub fn transputer() -> RngType {
        ffi_wrap!(gsl_rng_transputer)
    }

    /// This is the IBM RANDU generator. Its sequence is
    ///
    /// x_{n+1} = (a x_n) mod m
    ///
    /// with a = 65539 and m = 2^31. The seed specifies the initial value, x_1. The period of this generator was only 2^29. It has become a textbook example of a poor generator.
    #[doc(alias = "gsl_rng_randu")]
    pub fn randu() -> RngType {
        ffi_wrap!(gsl_rng_randu)
    }

    /// This is Park and Miller’s “minimal standard” MINSTD generator, a simple linear congruence which takes care to avoid the major pitfalls of such algorithms. Its sequence is,
    ///
    /// x_{n+1} = (a x_n) mod m
    ///
    /// with a = 16807 and m = 2^31 - 1 = 2147483647. The seed specifies the initial value, x_1. The period of this generator is about 2^31.
    ///
    /// This generator was used in the IMSL Library (subroutine RNUN) and in MATLAB (the RAND function) in the past. It is also sometimes known by the acronym "GGL" (I'm not sure what that stands for).
    ///
    /// For more information see,
    ///
    /// Park and Miller, "Random Number Generators: Good ones are hard to find", Communications of the ACM, October 1988, Volume 31, No 10, pages 1192–1201.
    #[doc(alias = "gsl_rng_minstd")]
    pub fn minstd() -> RngType {
        ffi_wrap!(gsl_rng_minstd)
    }

    /// This is a reimplementation of the 16-bit SLATEC random number generator RUNIF. A generalization of the generator to 32 bits is provided by gsl_rng_uni32.
    /// The original source code is available from NETLIB.
    #[doc(alias = "gsl_rng_uni")]
    pub fn uni() -> RngType {
        ffi_wrap!(gsl_rng_uni)
    }

    /// This is a reimplementation of the 16-bit SLATEC random number generator RUNIF. A generalization of the generator to 32 bits is provided by gsl_rng_uni32.
    /// The original source code is available from NETLIB.
    #[doc(alias = "gsl_rng_uni32")]
    pub fn uni32() -> RngType {
        ffi_wrap!(gsl_rng_uni32)
    }

    /// This is the SLATEC random number generator RAND. It is ancient. The original source code is available from NETLIB.
    #[doc(alias = "gsl_rng_slatec")]
    pub fn slatec() -> RngType {
        ffi_wrap!(gsl_rng_slatec)
    }

    /// This is the ZUFALL lagged Fibonacci series generator of Peterson. Its sequence is,
    ///
    /// t = u_{n-273} + u_{n-607}
    /// u_n  = t - floor(t)
    ///
    /// The original source code is available from NETLIB. For more information see,
    ///
    /// W. Petersen, “Lagged Fibonacci Random Number Generators for the NEC SX-3”, International Journal of High Speed Computing (1994).
    #[doc(alias = "gsl_rng_zuf")]
    pub fn zuf() -> RngType {
        ffi_wrap!(gsl_rng_zuf)
    }

    /// This is a second-order multiple recursive generator described by Knuth in Seminumerical Algorithms, 3rd Ed., page 108. Its sequence is,
    ///
    /// x_n = (a_1 x_{n-1} + a_2 x_{n-2}) mod m
    ///
    /// with a_1 = 271828183, a_2 = 314159269, and m = 2^31 - 1.
    #[doc(alias = "gsl_rng_knuthran2")]
    pub fn knuthran2() -> RngType {
        ffi_wrap!(gsl_rng_knuthran2)
    }

    /// This is a second-order multiple recursive generator described by Knuth in Seminumerical Algorithms, 3rd Ed., Section 3.6. Knuth provides
    /// its C code. The updated routine gsl_rng_knuthran2002 is from the revised 9th printing and corrects some weaknesses in the earlier version,
    /// which is implemented as gsl_rng_knuthran.
    #[doc(alias = "gsl_rng_knuthran2002")]
    pub fn knuthran2002() -> RngType {
        ffi_wrap!(gsl_rng_knuthran2002)
    }

    /// This is a second-order multiple recursive generator described by Knuth in Seminumerical Algorithms, 3rd Ed., Section 3.6. Knuth provides
    /// its C code. The updated routine gsl_rng_knuthran2002 is from the revised 9th printing and corrects some weaknesses in the earlier version,
    /// which is implemented as gsl_rng_knuthran.
    #[doc(alias = "gsl_rng_knuthran")]
    pub fn knuthran() -> RngType {
        ffi_wrap!(gsl_rng_knuthran)
    }

    /// This multiplicative generator is taken from Knuth’s Seminumerical Algorithms, 3rd Ed., pages 106–108. Their sequence is,
    ///
    /// x_{n+1} = (a x_n) mod m
    ///
    /// where the seed specifies the initial value, x_1. The parameters a and m are as follows, Borosh-Niederreiter: a = 1812433253,
    /// m = 2^32, Fishman18: a = 62089911, m = 2^31 - 1, Fishman20: a = 48271, m = 2^31 - 1, L’Ecuyer: a = 40692, m = 2^31 - 249,
    /// Waterman: a = 1566083941, m = 2^32.
    #[doc(alias = "gsl_rng_borosh13")]
    pub fn borosh13() -> RngType {
        ffi_wrap!(gsl_rng_borosh13)
    }

    /// This multiplicative generator is taken from Knuth’s Seminumerical Algorithms, 3rd Ed., pages 106–108. Their sequence is,
    ///
    /// x_{n+1} = (a x_n) mod m
    ///
    /// where the seed specifies the initial value, x_1. The parameters a and m are as follows, Borosh-Niederreiter: a = 1812433253,
    /// m = 2^32, Fishman18: a = 62089911, m = 2^31 - 1, Fishman20: a = 48271, m = 2^31 - 1, L’Ecuyer: a = 40692, m = 2^31 - 249,
    /// Waterman: a = 1566083941, m = 2^32.
    #[doc(alias = "gsl_rng_fishman18")]
    pub fn fishman18() -> RngType {
        ffi_wrap!(gsl_rng_fishman18)
    }

    /// This multiplicative generator is taken from Knuth’s Seminumerical Algorithms, 3rd Ed., pages 106–108. Their sequence is,
    ///
    /// x_{n+1} = (a x_n) mod m
    ///
    /// where the seed specifies the initial value, x_1. The parameters a and m are as follows, Borosh-Niederreiter: a = 1812433253,
    /// m = 2^32, Fishman18: a = 62089911, m = 2^31 - 1, Fishman20: a = 48271, m = 2^31 - 1, L’Ecuyer: a = 40692, m = 2^31 - 249,
    /// Waterman: a = 1566083941, m = 2^32.
    #[doc(alias = "gsl_rng_fishman20")]
    pub fn fishman20() -> RngType {
        ffi_wrap!(gsl_rng_fishman20)
    }

    /// This multiplicative generator is taken from Knuth’s Seminumerical Algorithms, 3rd Ed., pages 106–108. Their sequence is,
    ///
    /// x_{n+1} = (a x_n) mod m
    ///
    /// where the seed specifies the initial value, x_1. The parameters a and m are as follows, Borosh-Niederreiter: a = 1812433253,
    /// m = 2^32, Fishman18: a = 62089911, m = 2^31 - 1, Fishman20: a = 48271, m = 2^31 - 1, L’Ecuyer: a = 40692, m = 2^31 - 249,
    /// Waterman: a = 1566083941, m = 2^32.
    #[doc(alias = "gsl_rng_lecuyer21")]
    pub fn lecuyer21() -> RngType {
        ffi_wrap!(gsl_rng_lecuyer21)
    }

    /// This multiplicative generator is taken from Knuth’s Seminumerical Algorithms, 3rd Ed., pages 106–108. Their sequence is,
    ///
    /// x_{n+1} = (a x_n) mod m
    ///
    /// where the seed specifies the initial value, x_1. The parameters a and m are as follows, Borosh-Niederreiter: a = 1812433253,
    /// m = 2^32, Fishman18: a = 62089911, m = 2^31 - 1, Fishman20: a = 48271, m = 2^31 - 1, L’Ecuyer: a = 40692, m = 2^31 - 249,
    /// Waterman: a = 1566083941, m = 2^32.
    #[doc(alias = "gsl_rng_waterman14")]
    pub fn waterman14() -> RngType {
        ffi_wrap!(gsl_rng_waterman14)
    }

    /// This is the L’Ecuyer–Fishman random number generator. It is taken from Knuth’s Seminumerical Algorithms, 3rd Ed., page 108. Its sequence is,
    ///
    /// z_{n+1} = (x_n - y_n) mod m
    ///
    /// with m = 2^31 - 1. x_n and y_n are given by the fishman20 and lecuyer21 algorithms. The seed specifies the initial value, x_1.
    #[doc(alias = "gsl_rng_fishman2x")]
    pub fn fishman2x() -> RngType {
        ffi_wrap!(gsl_rng_fishman2x)
    }

    /// This is the Coveyou random number generator. It is taken from Knuth’s Seminumerical Algorithms, 3rd Ed., Section 3.2.2. Its sequence is,
    ///
    /// x_{n+1} = (x_n (x_n + 1)) mod m
    ///
    /// with m = 2^32. The seed specifies the initial value, x_1.
    #[doc(alias = "gsl_rng_coveyou")]
    pub fn coveyou() -> RngType {
        ffi_wrap!(gsl_rng_coveyou)
    }
}
