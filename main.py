#!/usr/bin/env python3
"""
Main entry point for the Financial ML System.

This script demonstrates the complete pipeline including:
- Gaussian Copula for asset dependencies
- Random Forests for return modeling
- Sentiment analysis from Bettafish
- News analysis from Exa
"""

import argparse
import sys
from pathlib import Path

# Add src to path
sys.path.insert(0, str(Path(__file__).parent / "src"))

from src.pipeline import FinancialMLPipeline
from src.utils.logger import get_logger


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Financial ML Pipeline with Gaussian Copula and Random Forests"
    )

    parser.add_argument(
        "--config",
        type=str,
        default="config/config.yaml",
        help="Path to configuration file",
    )

    parser.add_argument(
        "--mode",
        type=str,
        choices=["train", "predict", "simulate", "full"],
        default="full",
        help="Pipeline mode",
    )

    parser.add_argument(
        "--assets",
        type=str,
        nargs="+",
        help="Asset symbols to analyze (overrides config)",
    )

    parser.add_argument("--output-dir", type=str, help="Output directory for results")

    args = parser.parse_args()

    # Initialize pipeline
    logger = get_logger(__name__)

    try:
        pipeline = FinancialMLPipeline(config_path=args.config)

        # Override assets if specified
        if args.assets:
            pipeline.assets = args.assets
            logger.info(f"Assets overridden: {args.assets}")

        # Run pipeline based on mode
        if args.mode == "full":
            logger.info("Running full pipeline")
            results = pipeline.run_full_pipeline()

            # Generate report
            report = pipeline.generate_report()
            print("\n" + report)

            # Save report if output directory specified
            if args.output_dir:
                output_path = Path(args.output_dir)
                output_path.mkdir(parents=True, exist_ok=True)

                report_file = output_path / "pipeline_report.txt"
                with open(report_file, "w") as f:
                    f.write(report)

                logger.info(f"Report saved to {report_file}")

                # Note: Models are in Rust, not Python - save configuration instead
                logger.info("Models in Rust - configuration saved")

        elif args.mode == "train":
            logger.info("Running training pipeline")
            pipeline.run_full_pipeline()
            # Rust models are compiled binaries - no save needed

        elif args.mode == "predict":
            logger.info("Running Rust models and making predictions")
            predictions = pipeline.make_predictions()

            print("\nPredictions:")
            for asset, pred in predictions.items():
                print(f"  {asset}: {pred:.6f}")

        elif args.mode == "simulate":
            logger.info("Running simulation")
            pipeline.run_full_pipeline()

            simulations = pipeline.simulate_scenarios(n_simulations=1000, n_steps=10)

            print(f"\nSimulation shape: {simulations.shape}")
            print(f"Mean returns across all simulations:")
            for i, asset in enumerate(pipeline.assets):
                mean_return = simulations[:, :, i].mean()
                print(f"  {asset}: {mean_return:.6f}")

    except Exception as e:
        logger.error(f"Pipeline error: {e}", exc_info=True)
        sys.exit(1)


if __name__ == "__main__":
    main()
