library(ggplot2)
library(readr)

# Load the CSV data
data <- read_csv("readings.csv")

# Extract the two temperature columns
temp_data <- data[, c("temperature1_c", "temperature2_c")]

# Scale the data
temp_scaled <- scale(temp_data)

# Perform PCA
pca_result <- prcomp(temp_scaled)

# Create a dataframe for plotting
pca_df <- as.data.frame(pca_result$x)
pca_df$index <- 1:nrow(pca_df)

# Save plot to PNG
png("pca_temperature_plot.png", width = 800, height = 600)
ggplot(pca_df, aes(x = PC1, y = PC2)) +
  geom_point(color = "blue") +
  geom_path(aes(group = 1), alpha = 0.3) +
  labs(
    title = "PCA of Temperature Sensors",
    x = "Principal Component 1",
    y = "Principal Component 2"
  ) +
  theme_minimal()
dev.off()