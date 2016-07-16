# Rust Random Choice
Chooses samples randomly by their weights/probabilities.

### Advantages

- There is a good diversity for the case that all weights are equally distributed (in contrast to the roulette wheel selection algorithm which tends to select the same sample n times)
- Blazingly fast: O(n) (Roulette wheel selection algorithm: O(n * log n))
- The sum of the weights don't have to be 1.0, but must not overflow

This algorithm is based on the Low Variance Resampling algorithm.

### Applications
- **Evolutionary algorithms**: Choose the n fittest populations by their fitness **_x_**
- **Monte Carlo Localization**: Resampling of n particles by their weight **_w_**
