# Install required libraries if not already installed
#install.packages("ggplot2")
#install.packages("readr")
#install.packages("gridExtra")

# Load libraries
library(ggplot2)
library(readr)
library(gridExtra)

# Load the data
data <- read_csv("readings.csv")

# Create an index for x-axis (row numbers)
data$index <- 1:nrow(data)

# Plot 1: Temperature 1
p1 <- ggplot(data, aes(x = index, y = temperature1_c)) +
  geom_line() +
  labs(title = "Temperature 1 (C)", x = "Index", y = "C") +
  theme_minimal()

# Plot 2: Temperature 2
p2 <- ggplot(data, aes(x = index, y = temperature2_c)) +
  geom_line() +
  labs(title = "Temperature 2 (C)", x = "Index", y = "C") +
  theme_minimal()

# Plot 3: Humidity 2
p3 <- ggplot(data, aes(x = index, y = humidity2_pct)) +
  geom_line() +
  labs(title = "Humidity 2 (%)", x = "Index", y = "%") +
  theme_minimal()

# Plot 4: Pressure
p4 <- ggplot(data, aes(x = index, y = pressure_hpa)) +
  geom_line() +
  labs(title = "Pressure (hPa)", x = "Index", y = "hPa") +
  theme_minimal()

# Arrange plots in a 2x2 grid
png("sensor_4plots.png", width = 1000, height = 800)
grid.arrange(p1, p2, p3, p4, ncol = 2)
dev.off()
