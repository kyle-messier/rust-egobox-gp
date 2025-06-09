# Attempting to fit a basic spatial model using the Rust package `egobox-gp` 

I've created a snakemake (snakefile) to run the model fitting process.

1) Simulate MVN data in R. Save a training and test set (random locs) and a grid of locs for prediction
2) Fit the model using the training set
3) Predict on the out of sample test set and prediction grid
4) Save the results
5) Plot the results


# Running the snakemake workflow
Assuming you have snakemake installed, you can run the workflow with the following command:

```bash
snakemake --cores 1
```

All of the files created and used by the workflow should be referenced to the relative paths in this repository. The workflow is designed to be run in the root directory of the repository.

