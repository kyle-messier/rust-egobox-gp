# Generate a simple spatial simulation dataset

library(fields)
library(readr)
library(MASS)
# set seed for reproducibility
set.seed(1234)


locs <- cbind(
  runif(30, -1, 1), # x-coordinates
  runif(30, -1, 1) # y-coordinates
)

# Generate a covariance matrix for the spatial data
locs_expand <- expand.grid(locs[, 1], locs[, 2])
n <- nrow(locs_expand)
d <- fields::rdist(locs_expand)
sill <- 2.5
range <- 0.5
nugget <- 0.1
cov_matrix <- sill *
  fields::Matern(d, range = range, smoothness = 2.5) *
  (nugget * diag(n))

# simulate multivariate normal spatial random field
vals <- mvrnorm(1, mu = rep(0, n), Sigma = cov_matrix)

df <- data.frame(
  x = locs_expand[, 1],
  y = locs_expand[, 2],
  value = vals
)

df_train <- df[1:600, ]
df_test <- df[601:nrow(df), ]
# Save the training and test datasets to CSV files
train_path <- snakemake@output[[1]]
write_csv(df, train_path)
test_path <- snakemake@output[[2]]
write_csv(df_test, test_path)

# Create prediction grids on a fine, regular grid
grid_size <- 100
x_grid <- seq(-1, 1, length.out = grid_size)
y_grid <- seq(-1, 1, length.out = grid_size)
grid <- expand.grid(x = x_grid, y = y_grid)

pred_path <- snakemake@output[[3]]
write_csv(grid, pred_path)
