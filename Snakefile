rule all:
    input:
        "inst/data/train_data.csv",
        "inst/data/test_data.csv",
        "inst/data/prediction_locs.csv",
        "inst/output/model_output.csv",
        "inst/output/model_kernel.csv",
        "inst/output/model_noise.csv",
        "inst/plots/model_mean_chloro.png",
        "inst/plots/model_var_chloro.png",
        "inst/plots/model_results_residuals.png",
        "inst/plots/model_results_scatter.png",
        "inst/output/model_results_validation.csv",
        "inst/output/test_model_output.csv"  


rule generate_data:
    output:
        sim_plots="inst/plots/simulated_data_plots.png",
        train_plots="inst/plots/train_data_plots.png",
        prediction_plots="inst/plots/prediction_locs_plots.png",
        # Data files
        train="inst/data/train_data.csv",
        test = "inst/data/test_data.csv",
        prediction_locs="inst/data/prediction_locs.csv"
    conda:
        "envs/r-env.yaml"
    script:
        "R/generate_data.R"

rule build_rust:
    output:
        "target/release/rustgp"
    shell:
        "cargo build --release"

rule fit_model:
    input:
        binary="target/release/rustgp",
        train=rules.generate_data.output.train,
        predict=rules.generate_data.output.prediction_locs
    output:
        pred="inst/output/model_output.csv",
        kernel="inst/output/model_kernel.csv",
        noise="inst/output/model_noise.csv"

    conda:
        "envs/rust-env.yaml"
    log:
        "logs/fit_model.log"        
    shell:
        """
        {input.binary} \
          --input-csv {input.train} \
          --predict-csv {input.predict} \
          --output-pred-csv {output.pred} \
          --output-kernel-csv {output.kernel} \
          --output-noise-csv {output.noise} 
        """
rule test_model:
    input:
        binary="target/release/rustgp",
        train=rules.generate_data.output.train,
        predict=rules.generate_data.output.test,
    output:
        pred="inst/output/test_model_output.csv"
    conda:
        "envs/rust-env.yaml"
    log:
        "logs/fit_test.log"        
    shell:
        """
        {input.binary} \
          --input-csv {input.train} \
          --predict-csv {input.predict} \
          --output-pred-csv {output.pred}
        """        
rule visualize_results:
    input:
        predictions="inst/output/model_output.csv",
        test_predictions="inst/output/test_model_output.csv",
        test_data="inst/data/test_data.csv",
    output:
        chloro_mean_plot="inst/plots/model_mean_chloro.png",
        chloro_sd_plot="inst/plots/model_var_chloro.png",
        residuals_plot="inst/plots/model_results_residuals.png",
        scatter_plot="inst/plots/model_results_scatter.png",
        val_results ="inst/output/model_results_validation.csv"
    conda:
        "envs/r-env.yaml"
    script:
        "R/visualize_results.R"