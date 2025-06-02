library(tidyverse)

# Grid predictions and visualization of results
# Read input CSV using Snakemake input variable
mdl_results <- read_csv(snakemake@input[["predictions"]])

# Do your visualization...
ggplot(mdl_results, aes(x = x, y = y), size = 3) +
  geom_point(aes(color = predicted_mean)) +
  scale_color_viridis_c(direction = -1, option = "magma")

# Save output to the path specified by Snakemake
ggsave(snakemake@output[[1]], width = 8, height = 6)


# Test set predictions, visualization of results, and validation statistics

test_results <- read_csv(snakemake@input[["test_predictions"]])
test_obs <- read_csv(snakemake@input[["test_data"]])

df <- test_results %>%
  left_join(test_obs, by = c("x", "y")) %>%
  mutate(
    residuals = value - predicted_mean,
    squared_error = residuals^2
  )
ggplot(df, aes(x = x, y = y)) +
  geom_point(aes(color = residuals)) +
  scale_color_gradient2(
    low = "blue",
    mid = "gray90",
    high = "red",
    midpoint = 0
  ) +
  labs(title = "Residuals of Predictions")
ggsave(snakemake@output[[2]], width = 8, height = 6)

#Classic obs-pred scatter plot
ggplot(df, aes(x = predicted_mean, y = value)) +
  geom_point() +
  geom_abline(slope = 1, intercept = 0, color = "red") +
  labs(
    x = "Predicted Mean",
    y = "Observed Value",
    title = "Observed vs Predicted"
  )
ggsave(snakemake@output[[3]], width = 8, height = 6)
# Calculate validation statistics
mse <- mean(df$squared_error, na.rm = TRUE)
mae <- mean(abs(df$residuals), na.rm = TRUE)
r2 <- cor(df$value, df$predicted_mean, use = "complete.obs")^2
# Save validation statistics to a text file
validation_stats <- tibble(
  mse = mse,
  mae = mae,
  r2 = r2
)
write_csv(validation_stats, snakemake@output[[4]])
