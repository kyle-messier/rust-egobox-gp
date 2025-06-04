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
nugget <- 1e-6
cov_matrix <- sill *
  fields::Exponential(d, aRange = alpha) +
  nugget * diag(n)

# simulate multivariate normal spatial random field
vals <- mvrnorm(1, mu = rep(pi, n), Sigma = cov_matrix)

df <- data.frame(
  x = locs[, 1],
  y = locs[, 2],
  value = vals
)

# Split the dataset into training and test sets
idx_train <- sample(seq_len(nrow(df)), 600, replace = FALSE)


df_train <- df[idx_train, ]
df_test <- df[-idx_train, ]

# Plot the generated data (1) all (2) training set (3) test set
ggplot(df, aes(x = x, y = y)) +
  geom_point(aes(color = value)) +
  scale_color_viridis_c(direction = -1, option = "magma") +
  labs(title = "Generated Spatial Data")
ggsave(snakemake@output[[1]], width = 8, height = 6)

ggplot(df_train, aes(x = x, y = y)) +
  geom_point(aes(color = value)) +
  scale_color_viridis_c(direction = -1, option = "magma") +
  labs(title = "Training Set Spatial Data")
ggsave(snakemake@output[[2]], width = 8, height = 6)
ggplot(df_test, aes(x = x, y = y)) +
  geom_point(aes(color = value)) +
  scale_color_viridis_c(direction = -1, option = "magma") +
  labs(title = "Test Set Spatial Data")
ggsave(snakemake@output[[3]], width = 8, height = 6)
# Save the training and test datasets to CSV files
train_path <- snakemake@output[[4]]
write_csv(df_train, train_path)
test_path <- snakemake@output[[5]]
write_csv(df_test, test_path)

# Create prediction grids on a fine, regular grid
grid_size <- 100
x_grid <- seq(-1, 1, length.out = grid_size)
y_grid <- seq(-1, 1, length.out = grid_size)
grid <- expand.grid(x = x_grid, y = y_grid)

pred_path <- snakemake@output[[6]]
write_csv(grid, pred_path)
