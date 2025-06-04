# Generate a simple spatial simulation dataset
library(tidyverse)
library(fields)
library(readr)
library(MASS)
# set seed for reproducibility
set.seed(123)


locs <- cbind(
  runif(1000, -1, 1), # x-coordinates
  runif(1000, -1, 1) # y-coordinates
)

# Generate a covariance matrix for the spatial data
n <- nrow(locs)
d <- fields::rdist(locs)
sill <- 3 * pi
alpha <- 1
nugget <- 0.33 * pi
cov_matrix <- sill *
  fields::Matern(d, alpha = alpha, smoothness = 2.5) +
  nugget * diag(n)

# simulate multivariate normal spatial random field
vals <- mvrnorm(1, mu = rep(pi, n), Sigma = cov_matrix)

df <- data.frame(
  x = locs[, 1],
  y = locs[, 2],
  value = vals
)

idx_train <- sample(seq_len(nrow(df)), 600, replace = FALSE)
# Split the dataset into training and test sets

df_train <- df[idx_train, ]
df_test <- df[-idx_train, ]
# Save the training and test datasets to CSV files
train_path <- snakemake@output[[1]]
write_csv(df_train, train_path)
test_path <- snakemake@output[[2]]
write_csv(df_test, test_path)

# Create prediction grids on a fine, regular grid
grid_size <- 100
x_grid <- seq(-1, 1, length.out = grid_size)
y_grid <- seq(-1, 1, length.out = grid_size)
grid <- expand.grid(x = x_grid, y = y_grid)

pred_path <- snakemake@output[[3]]
write_csv(grid, pred_path)
