"""Main pipeline orchestrating Java backend and Rust models."""

import asyncio
import pandas as pd
import numpy as np
import requests
import subprocess
import json
from pathlib import Path
from typing import Dict, List, Optional
from datetime import datetime

from .utils.config import Config
from .utils.logger import setup_logger, get_logger


class FinancialMLPipeline:
    """
    Main pipeline for financial ML system using Java backend and Rust models.

    Architecture:
    - Java: Data ingestion, processing, storage (Redis)
    - Rust: Random Forest and Gaussian Copula models
    - Python: Orchestration and UI integration
    """

    def __init__(self, config_path: Optional[str] = None):
        """Initialize pipeline."""
        self.config = Config(config_path)
        self.logger = setup_logger(
            __name__,
            log_file=self.config.get("logging.file"),
            level=self.config.get("logging.level"),
            format_type=self.config.get("logging.format"),
        )

        self.logger.info("Initializing Financial ML Pipeline (Java/Rust Architecture)")

        # Get assets
        self.assets = self.config.get_assets()
        self.logger.info(f"Assets to track: {self.assets}")

        # Java backend configuration
        self.java_api_url = f"http://{self.config.get('architecture.backend.api.host')}:{self.config.get('architecture.backend.api.port')}"
        self.redis_config = self.config.get("architecture.backend.redis")
        self.logger.info(f"Java API: {self.java_api_url}")
        self.logger.info(
            f"Redis: {self.redis_config['host']}:{self.redis_config['port']}"
        )

        # Initialize components
        self._init_components()

    def _init_components(self):
        """Initialize Java backend and Rust models."""
        # Check if Java backend is running
        try:
            response = requests.get(f"{self.java_api_url}/api/health", timeout=2)
            if response.status_code == 200:
                self.logger.info("Java backend is running")
            else:
                self.logger.warning("Java backend may not be fully initialized")
        except requests.exceptions.RequestException as e:
            self.logger.error(f"Cannot connect to Java backend: {e}")
            self.logger.info(
                "Please start Java backend: cd java-backend && mvn exec:java"
            )

        # Rust model paths
        self.rust_model_path = (
            Path(__file__).parent.parent
            / "rust-model"
            / "target"
            / "release"
            / "rust-model"
        )
        self.logger.info(f"Rust model path: {self.rust_model_path}")

    def check_java_backend(self) -> bool:
        """Check if Java backend is running."""
        try:
            response = requests.get(f"{self.java_api_url}/api/health", timeout=2)
            return response.status_code == 200
        except:
            return False

    def start_java_backend(self):
        """Start Java backend if not running."""
        if self.check_java_backend():
            self.logger.info("Java backend already running")
            return

        self.logger.info("Starting Java backend...")
        java_backend_dir = Path(__file__).parent.parent / "java-backend"
        try:
            subprocess.Popen(
                ["mvn", "exec:java", "-Dexec.mainClass=com.financial.backend.Main"],
                cwd=str(java_backend_dir),
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )
            self.logger.info("Java backend starting...")

            # Wait for backend to start
            import time

            for i in range(10):
                time.sleep(2)
                if self.check_java_backend():
                    self.logger.info("Java backend started successfully")
                    return

            self.logger.error("Failed to start Java backend within timeout")
        except Exception as e:
            self.logger.error(f"Failed to start Java backend: {e}")

    def run_rust_models(self) -> Dict:
        """Run Rust models and get results."""
        self.logger.info("Running Rust models...")

        if not self.rust_model_path.exists():
            self.logger.error(
                "Rust models not built. Please run: cd rust-model && cargo build --release"
            )
            return {}

        try:
            # Run Rust model executable
            result = subprocess.run(
                [str(self.rust_model_path)], capture_output=True, text=True, timeout=60
            )

            if result.returncode != 0:
                self.logger.error(f"Rust models failed: {result.stderr}")
                return {}

            # Parse output
            output_lines = result.stdout.strip().split("\n")
            return self._parse_rust_output(output_lines)

        except subprocess.TimeoutExpired:
            self.logger.error("Rust models timed out")
            return {}
        except Exception as e:
            self.logger.error(f"Failed to run Rust models: {e}")
            return {}

    def _parse_rust_output(self, lines: List[str]) -> Dict:
        """Parse Rust model output."""
        results = {
            "strongest_movers": [],
            "highest_volume": [],
            "highest_probable_alpha": [],
            "correlation_analysis": {},
        }

        current_section = None

        for line in lines:
            line = line.strip()

            if "Strongest Movers" in line:
                current_section = "strongest_movers"
            elif "Highest Volume" in line:
                current_section = "highest_volume"
            elif "Highest Probable Alpha" in line:
                current_section = "highest_probable_alpha"
            elif "Correlation Analysis" in line:
                current_section = "correlation"
            elif current_section and "|" in line:
                # Parse table row
                parts = [p.strip() for p in line.split("|")]
                if len(parts) >= 4 and "Symbol" not in parts:
                    result = {
                        "symbol": parts[1],
                        "alpha": float(parts[2])
                        if current_section != "highest_volume"
                        else None,
                        "probability": float(parts[3].rstrip("%")) / 100
                        if "%" in parts[3]
                        else None,
                        "volume": float(parts[2].replace(",", ""))
                        if current_section == "highest_volume"
                        else None,
                        "change": float(parts[4].rstrip("%"))
                        if len(parts) > 4 and "%" in parts[4]
                        else None,
                    }
                    results[f"{current_section}s"].append(result)
            elif "Highest Correlation" in line:
                if ":" in line:
                    results["correlation_analysis"]["highest"] = line.split(":")[
                        1
                    ].strip()
            elif "Lowest Correlation" in line:
                if ":" in line:
                    results["correlation_analysis"]["lowest"] = line.split(":")[
                        1
                    ].strip()
            elif "Matrix Size" in line:
                if ":" in line:
                    results["correlation_analysis"]["size"] = int(
                        line.split(":")[1].strip()
                    )

        return results

    def get_data_from_java(self, symbol: str, limit: int = 100) -> pd.DataFrame:
        """Get data from Java backend."""
        try:
            response = requests.get(
                f"{self.java_api_url}/api/equity/symbol",
                params={"symbol": symbol, "limit": limit},
                timeout=5,
            )

            if response.status_code == 200:
                data = response.json()
                return pd.DataFrame(data)
            else:
                self.logger.error(
                    f"Failed to get data for {symbol}: {response.status_code}"
                )
                return pd.DataFrame()
        except Exception as e:
            self.logger.error(f"Error fetching data for {symbol}: {e}")
            return pd.DataFrame()

    def get_all_data(self) -> pd.DataFrame:
        """Get all data from Java backend."""
        try:
            response = requests.get(f"{self.java_api_url}/api/equity/all", timeout=10)

            if response.status_code == 200:
                data = response.json()
                df = pd.DataFrame(data)
                self.logger.info(f"Retrieved {len(df)} records from Java backend")
                return df
            else:
                self.logger.error(f"Failed to get all data: {response.status_code}")
                return pd.DataFrame()
        except Exception as e:
            self.logger.error(f"Error fetching all data: {e}")
            return pd.DataFrame()

    def run_full_pipeline(self) -> Dict:
        """Run full pipeline."""
        self.logger.info("=" * 60)
        self.logger.info("Running Full Pipeline (Java/Rust Architecture)")
        self.logger.info("=" * 60)

        results = {
            "timestamp": datetime.now().isoformat(),
            "assets": self.assets,
            "java_backend_status": self.check_java_backend(),
            "data": None,
            "rust_models": None,
        }

        # Get data from Java backend
        if results["java_backend_status"]:
            self.logger.info("Fetching data from Java backend...")
            results["data"] = self.get_all_data()

            if not results["data"].empty:
                self.logger.info(f"Data fetched: {len(results['data'])} records")
            else:
                self.logger.warning("No data available from Java backend")
        else:
            self.logger.warning("Java backend not running, skipping data fetch")

        # Run Rust models
        self.logger.info("Running Rust models...")
        results["rust_models"] = self.run_rust_models()

        if results["rust_models"]:
            self.logger.info("Rust models completed successfully")
        else:
            self.logger.warning("Rust models produced no results")

        return results

    def generate_report(self, results: Optional[Dict] = None) -> str:
        """Generate report from results."""
        if results is None:
            results = self.run_full_pipeline()

        report = []
        report.append("=" * 70)
        report.append("FINANCIAL ML PIPELINE REPORT")
        report.append(f"Generated: {results.get('timestamp', 'N/A')}")
        report.append(f"Assets: {', '.join(results.get('assets', []))}")
        report.append("=" * 70)

        # Java Backend Status
        report.append("\n[Java Backend Status]")
        if results.get("java_backend_status"):
            report.append("  ✓ Java backend is running")
            if results.get("data") is not None:
                report.append(f"  ✓ Records retrieved: {len(results['data'])}")
        else:
            report.append("  ✗ Java backend is not running")

        # Rust Model Results
        report.append("\n[Rust Model Results]")
        rust_results = results.get("rust_models", {})

        if rust_results:
            # Strongest Movers
            if rust_results.get("strongest_movers"):
                report.append("\n  Strongest Movers (Alpha Search):")
                for i, mover in enumerate(rust_results["strongest_movers"][:5], 1):
                    report.append(
                        f"    {i}. {mover.get('symbol', 'N/A'):<8} | "
                        f"Alpha: {mover.get('alpha', 0):>8.4f} | "
                        f"Prob: {mover.get('probability', 0):>6.2%} | "
                        f"Change: {mover.get('change', 0):>7.2f}%"
                    )

            # Highest Volume
            if rust_results.get("highest_volume"):
                report.append("\n  Highest Volume:")
                for i, item in enumerate(rust_results["highest_volume"][:5], 1):
                    report.append(
                        f"    {i}. {item.get('symbol', 'N/A'):<8} | "
                        f"Volume: {item.get('volume', 0):>12,.0f} | "
                        f"Alpha: {item.get('alpha', 0):>8.4f}"
                    )

            # Highest Probable Alpha
            if rust_results.get("highest_probable_alpha"):
                report.append("\n  Highest Probable Alpha (Gaussian Copula):")
                for i, item in enumerate(rust_results["highest_probable_alpha"][:5], 1):
                    report.append(
                        f"    {i}. {item.get('symbol', 'N/A'):<8} | "
                        f"Alpha: {item.get('alpha', 0):>8.4f} | "
                        f"Prob: {item.get('probability', 0):>6.2%} | "
                        f"Change: {item.get('change', 0):>7.2f}%"
                    )

            # Correlation Analysis
            corr = rust_results.get("correlation_analysis", {})
            if corr:
                report.append("\n  Correlation Analysis:")
                report.append(f"    Matrix Size: {corr.get('size', 'N/A')}")
                report.append(f"    Highest Correlation: {corr.get('highest', 'N/A')}")
                report.append(f"    Lowest Correlation: {corr.get('lowest', 'N/A')}")
        else:
            report.append("  No results from Rust models")

        report.append("\n" + "=" * 70)

        return "\n".join(report)

    def make_predictions(self) -> Dict:
        """Make predictions using Rust models."""
        results = self.run_rust_models()
        predictions = {}

        # Get alpha predictions from strongest movers
        for mover in results.get("strongest_movers", []):
            predictions[mover["symbol"]] = mover.get("alpha", 0)

        return predictions

    def simulate_scenarios(
        self, n_simulations: int = 1000, n_steps: int = 10
    ) -> np.ndarray:
        """
        Run simulations using Gaussian Copula from Rust.

        Note: Full simulation requires Rust model to be updated with simulation output.
        """
        self.logger.warning("Full simulation not yet implemented in Rust models")
        self.logger.info("Use Rust model output for correlation analysis instead")

        # Return empty array for now
        return np.array([])


def main():
    """Main entry point for testing."""
    pipeline = FinancialMLPipeline()
    results = pipeline.run_full_pipeline()
    print(pipeline.generate_report(results))


if __name__ == "__main__":
    main()
