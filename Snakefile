# Snakefile for Snakemake workflow

rule all:
    input:
        "inst/output/model_output.json"

rule generate_data:
    output:
        "inst/output/generated_data.csv"
    conda:
        "envs/r-env.yaml"
    script:
        "R/generate_data.R"

rule build_rust:
    output:
        "target/release/model_fit"
    shell:
        "cargo build --release --manifest-path src/Cargo.toml"

rule fit_model:
    input:
        binary="target/release/model_fit",
        data="inst/output/generated_data.csv"
    output:
        "inst/output/model_output.json"
    conda:
        "envs/rust-env.yaml"
    shell:
        """
        {input.binary} {input.data} > {output}
        """
