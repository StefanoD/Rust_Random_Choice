# Rust Random Choice
Chooses samples randomly by their weights/probabilities.

### Advantages

- There is a good diversity for the case that all weights are equally distributed (in contrast to the roulette wheel selection algorithm which tends to select the same sample n times)
- Blazingly fast: O(n) (Roulette wheel selection algorithm: O(n * log n))
- Memory Usage O(n); in place variant: O(1)
- The sum of the weights don't have to be 1.0, but must not overflow

This algorithm is based on the stochastic universal sampling algorithm.

### Applications
- **Evolutionary algorithms**: Choose the _n_ fittest populations by their fitness **_fi_**
- **Monte Carlo Localization**: Resampling of _n_ particles by their weight **_w_**
